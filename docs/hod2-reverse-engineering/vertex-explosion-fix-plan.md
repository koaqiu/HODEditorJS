# Vertex Explosion Fix Plan

## Status: Root Cause Identified

**Created:** 2026-05-28  
**Priority:** CRITICAL  
**Estimated Time:** 2-3 hours  

---

## Root Cause Analysis

### Problem

Generated HOD files cause vertex explosion in game - vertices stretch to infinity with random color bands.

### Evidence

**Generated HOD (from assets):**
- Binormal: (0.08448632, 0.6289825, 0.7728151)

**Vanilla HOD:**
- Binormal: (-0.038397547, 0.65264744, 0.7566881)

**Difference:** Binormal values are completely different!

### Root Cause

Our Gram-Schmidt orthogonalization algorithm is calculating different binormal values than RODOH. The binormal calculation is incorrect, causing the vertex shader to compute incorrect tangent-space transformations, leading to vertex explosion.

---

## Technical Details

### Current Implementation

**File:** `parser/src/compiler.rs:193-213`

```rust
// Apply Gram-Schmidt orthogonalization
for (idx, vertex) in vertices.iter_mut().enumerate() {
    let normal = vertex.normal.clone().unwrap_or(Vector3 { x: 0.0, y: 1.0, z: 0.0 });
    let tangent = tangents[idx].clone();
    let binormal = binormals[idx].clone();
    
    // Gram-Schmidt: tangent = normalize(tangent - normal * dot(tangent, normal))
    let tangent_dot_normal = dot_vec3(&tangent, &normal);
    let tangent_orthogonal = sub_vec3(&tangent, &mul_vec3(&normal, tangent_dot_normal));
    let tangent_normalized = normalize_vec3(tangent_orthogonal, Vector3 { x: 1.0, y: 0.0, z: 0.0 });
    
    // Calculate binormal as cross product of normal and tangent
    let binormal_cross = cross_vec3(&normal, &tangent_normalized);
    let binormal_normalized = normalize_vec3(binormal_cross, Vector3 { x: 0.0, y: 0.0, z: 1.0 });
    
    // Calculate handedness
    let handedness = if dot_vec3(&cross_vec3(&normal, &tangent_normalized), &binormal) >= 0.0 { 1.0 } else { -1.0 };
    
    vertex.tangent = Some(tangent_normalized);
    vertex.binormal = Some(mul_vec3(&binormal_normalized, handedness));
}
```

### Issue

The binormal calculation is using:
```rust
let binormal_cross = cross_vec3(&normal, &tangent_normalized);
```

But RODOH likely uses a different formula. The cross product order or handedness calculation is different.

---

## Proposed Solutions

### Solution 1: Use Standard TBN Calculation

**Formula:**
```
binormal = cross(normal, tangent) * handedness
```

**Where handedness is calculated from UV handedness:**
```
handedness = dot(cross(normal, tangent), bitangent) >= 0 ? 1.0 : -1.0
```

### Solution 2: Use UV Handedness

**Calculate handedness from UV coordinates:**
```
handedness = sign(du1 * dv2 - du2 * dv1)
```

**Where:**
- du1 = uv1.u - uv0.u
- dv1 = uv1.v - uv0.v
- du2 = uv2.u - uv0.u
- dv2 = uv2.v - uv0.v

### Solution 3: Match RODOH Algorithm

**Analyze RODOH's tangent calculation:**
1. Extract tangent data from vanilla HOD
2. Compare with our calculation
3. Identify the difference
4. Match the algorithm

---

## Implementation Plan

### Step 1: Analyze RODOH's Binormal Calculation

**Method:**
1. Extract tangent/binormal data from vanilla HOD
2. Calculate what RODOH's algorithm would produce
3. Compare with our output
4. Identify the difference

**Tools:**
- `dump_mesh_pool` - Extract vertex data
- Custom analysis script

### Step 2: Fix Binormal Calculation

**Option A: Use UV Handedness**
```rust
fn compute_tangent_space_with_uv_handedness(vertices: &mut [HODVertex], indices: &[u16]) {
    // ... (similar to current implementation)
    
    // Calculate UV handedness for each triangle
    for tri in indices.chunks_exact(3) {
        // ... (get vertices and UVs)
        
        let du1 = uv1.u - uv0.u;
        let dv1 = uv1.v - uv0.v;
        let du2 = uv2.u - uv0.u;
        let dv2 = uv2.v - uv0.v;
        let handedness = (du1 * dv2 - du2 * dv1).signum();
        
        // Accumulate handedness per vertex
        handedness_per_vertex[idx] += handedness;
    }
    
    // Apply average handedness
    for (idx, vertex) in vertices.iter_mut().enumerate() {
        let avg_handedness = handedness_per_vertex[idx].signum();
        vertex.binormal = Some(mul_vec3(&binormal_normalized, avg_handedness));
    }
}
```

**Option B: Use Cross Product Order**
```rust
// Try different cross product order
let binormal_cross = cross_vec3(&tangent_normalized, &normal);
```

### Step 3: Test Fix

**Test Cases:**
1. pebble_0 - Simple mesh
2. pebble_1 - Multiple LODs
3. pebble_2 - More complex

**Validation:**
- Compare binormal values with vanilla
- Test in game
- Verify no vertex explosion

---

## Expected Outcome

### Before Fix

- Binormal: (0.08448632, 0.6289825, 0.7728151)
- Vertex explosion in game

### After Fix

- Binormal: (-0.038397547, 0.65264744, 0.7566881)
- Correct rendering in game

---

## Risk Assessment

### Risk 1: Wrong Algorithm

**Probability:** Medium  
**Mitigation:** Test multiple algorithms, compare with vanilla

### Risk 2: Performance Impact

**Probability:** Low  
**Mitigation:** Binormal calculation is pre-computed, no runtime impact

### Risk 3: Breaking Existing Tests

**Probability:** Medium  
**Mitigation:** Run validation suite after fix

---

## Success Criteria

### Primary Success

- [ ] Binormal values match vanilla HOD
- [ ] No vertex explosion in game
- [ ] Correct rendering

### Secondary Success

- [ ] All validation tests pass
- [ ] No performance regression
- [ ] Ready for HOD 1.0 testing

---

## Timeline

### Immediate (Next Session)

1. **Analyze RODOH's binormal calculation** (1 hour)
2. **Implement fix** (1 hour)
3. **Test fix** (30 minutes)

**Total:** 2.5 hours

---

**Plan Version:** 1.0  
**Last Updated:** 2026-05-28  
**Status:** Ready for Implementation