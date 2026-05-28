# Generated HOD Issues Fix Plan

## Status: Two Critical Issues Identified

**Created:** 2026-05-28  
**Priority:** CRITICAL  
**Estimated Time:** 4-6 hours  

---

## Issues Identified

### Issue 1: Vertex Explosion (Binormal Calculation)

**Symptom:** Mesh stretches to infinity with random color bands  
**Root Cause:** Binormal values are different from vanilla  
**Evidence:**
- Generated binormal: (0.08448632, 0.6289825, 0.7728151)
- Vanilla binormal: (-0.038397547, 0.65264744, 0.7566881)

### Issue 2: Texture Corruption (Compression Settings)

**Symptom:** Textures appear corrupted in game  
**Root Cause:** Texture pool is 33% smaller than vanilla  
**Evidence:**
- Generated texture pool: 1,124,443 bytes compressed
- Vanilla texture pool: 1,673,248 bytes compressed

---

## Root Cause Analysis

### Issue 1: Binormal Calculation

**Current Implementation:**
```rust
let binormal_cross = cross_vec3(&normal, &tangent_normalized);
let binormal_normalized = normalize_vec3(binormal_cross, Vector3 { x: 0.0, y: 0.0, z: 1.0 });
let handedness = if dot_vec3(&cross_vec3(&normal, &tangent_normalized), &binormal) >= 0.0 { 1.0 } else { -1.0 };
vertex.binormal = Some(mul_vec3(&binormal_normalized, handedness));
```

**Problem:** The cross product order or handedness calculation is different from RODOH.

**Solution:** Use UV-based handedness calculation.

### Issue 2: Texture Compression

**Current Implementation:**
- Uses `xpress::compress_or_raw` for texture compression
- No DXT compression applied to TGA textures
- Textures are stored as raw RGBA data

**Problem:** RODOH applies DXT compression to textures, which is required for correct rendering in game.

**Solution:** Implement DXT1/DXT5 compression for textures.

---

## Implementation Plan

### Phase 1: Fix Binormal Calculation (2 hours)

**Step 1: Analyze RODOH's binormal calculation**
- Extract tangent/binormal data from vanilla HOD
- Compare with our calculation
- Identify the difference

**Step 2: Implement UV-based handedness**
```rust
fn compute_tangent_space_with_uv_handedness(vertices: &mut [HODVertex], indices: &[u16]) {
    let mut tangents = vec![Vector3 { x: 0.0, y: 0.0, z: 0.0 }; vertices.len()];
    let mut binormals = vec![Vector3 { x: 0.0, y: 0.0, z: 0.0 }; vertices.len()];
    let mut handedness_per_vertex = vec![0.0f32; vertices.len()];

    for tri in indices.chunks_exact(3) {
        // ... (get vertices and UVs)
        
        let du1 = uv1.u - uv0.u;
        let dv1 = uv1.v - uv0.v;
        let du2 = uv2.u - uv0.u;
        let dv2 = uv2.v - uv0.v;
        let handedness = (du1 * dv2 - du2 * dv1).signum();
        
        // Accumulate handedness per vertex
        for idx in [i0, i1, i2] {
            handedness_per_vertex[idx] += handedness;
        }
    }
    
    // Apply average handedness
    for (idx, vertex) in vertices.iter_mut().enumerate() {
        let avg_handedness = handedness_per_vertex[idx].signum();
        vertex.binormal = Some(mul_vec3(&binormal_normalized, avg_handedness));
    }
}
```

**Step 3: Test fix**
- Compare binormal values with vanilla
- Verify no vertex explosion

### Phase 2: Fix Texture Compression (2-4 hours)

**Step 1: Analyze RODOH's texture compression**
- Extract texture data from vanilla HOD
- Compare with our compression
- Identify the difference

**Step 2: Implement DXT1/DXT5 compression**
```rust
fn compress_texture_to_dxt(rgba: &[u8], width: usize, height: usize, format: &str) -> Vec<u8> {
    match format {
        "DXT1" => compress_dxt1(rgba, width, height),
        "DXT5" => compress_dxt5(rgba, width, height),
        _ => rgba.to_vec(),
    }
}
```

**Step 3: Test fix**
- Compare texture pool sizes with vanilla
- Verify textures render correctly in game

---

## Testing Strategy

### Test Case 1: pebble_0

**Input:**
- OBJ files (Root_mesh_lod0.obj, Root_mesh_lod1.obj)
- TGA files (Pebble_DIFF.tga, Pebble_GLOW.tga, Pebble_NORM.tga)
- JSON material definition

**Expected Output:**
- Valid HOD 2.0 file
- Correct binormal values
- Correct texture compression

**Validation:**
- Compare binormal values with vanilla
- Compare texture pool sizes with vanilla
- Test in game

### Test Case 2: pebble_1

**Input:**
- Same texture files as pebble_0
- Different mesh data

**Expected Output:**
- Valid HOD 2.0 file
- Correct binormal values
- Correct texture compression

**Validation:**
- Compare binormal values with vanilla
- Compare texture pool sizes with vanilla
- Test in game

---

## Expected Outcome

### Before Fix

**Binormal:**
- Generated: (0.08448632, 0.6289825, 0.7728151)
- Vanilla: (-0.038397547, 0.65264744, 0.7566881)

**Texture Pool:**
- Generated: 1,124,443 bytes compressed
- Vanilla: 1,673,248 bytes compressed

**Result:** Vertex explosion and texture corruption

### After Fix

**Binormal:**
- Generated: (-0.038397547, 0.65264744, 0.7566881)
- Vanilla: (-0.038397547, 0.65264744, 0.7566881)

**Texture Pool:**
- Generated: ~1,673,248 bytes compressed
- Vanilla: 1,673,248 bytes compressed

**Result:** Correct rendering in game

---

## Risk Assessment

### Risk 1: Wrong Binormal Algorithm

**Probability:** Medium  
**Mitigation:** Test multiple algorithms, compare with vanilla

### Risk 2: DXT Compression Quality

**Probability:** Medium  
**Mitigation:** Match RODOH's compression settings

### Risk 3: Breaking Existing Tests

**Probability:** Medium  
**Mitigation:** Run validation suite after fix

---

## Success Criteria

### Primary Success

- [ ] Binormal values match vanilla HOD
- [ ] Texture pool sizes match vanilla HOD
- [ ] No vertex explosion in game
- [ ] Correct texture rendering in game

### Secondary Success

- [ ] All validation tests pass
- [ ] No performance regression
- [ ] Ready for HOD 1.0 testing

---

## Timeline

### Immediate (Next Session)

1. **Analyze RODOH's binormal calculation** (1 hour)
2. **Implement binormal fix** (1 hour)
3. **Analyze RODOH's texture compression** (1 hour)
4. **Implement texture compression fix** (1-2 hours)
5. **Test fix** (30 minutes)

**Total:** 4-5 hours

---

**Plan Version:** 1.0  
**Last Updated:** 2026-05-28  
**Status:** Ready for Implementation