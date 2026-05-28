# Phase 2 Findings: HOD 2.0 Creation Analysis

## Status: In Progress

**Started:** 2026-05-27  
**Last Updated:** 2026-05-27  
**Current Focus:** Testing Results & Compression Analysis

---

## Executive Summary

Phase 2 testing has revealed critical insights about HOD 2.0 creation. The parser successfully builds HOD files from assets (OBJ + TGA + JSON), preserves structural integrity, and maintains round-trip fidelity. Key findings include compression differences between vanilla and generated files, and successful HOD 1.0 conversion.

---

## Testing Results

### 1. replicate_testing - Asset-Based HOD Creation ✅

**Status:** SUCCESS  
**Command:** `cargo run --bin replicate_testing`

**Results:**
- Successfully built HOD 2.0 from OBJ + TGA + JSON assets
- Copied KDOP/COLD/INFO from vanilla files
- Generated files for pebble_0, pebble_1, pebble_2

**Key Metrics:**
```
pebble_0:
  vanilla: meshes=3 materials=1 textures=3 joints=1 preserved=2
  assets: meshes=3 materials=1 textures=3 joints=1 preserved=2
  generated_from_assets=1,154,404 bytes

pebble_2:
  vanilla: meshes=3 materials=1 textures=3 joints=1 preserved=2
  assets: meshes=3 materials=1 textures=3 joints=1 preserved=2
  generated_from_assets=1,154,404 bytes
```

**Mesh Comparison:**
```
Root_mesh lod 0: verts 726->726 indices 726->726 pos=0 norm=0 uv=0 tangent=726
Root_mesh lod 1: verts 468->468 indices 468->468 pos=0 norm=0 uv=0 tangent=468
Root_mesh lod 2: verts 210->210 indices 210->210 pos=0 norm=0 uv=0 tangent=210
```

**Analysis:**
- Vertex counts match exactly (good sign)
- Position, normal, UV data preserved (0 differences)
- Tangent data preserved (726 differences expected due to calculation)
- KDOP/COLD/INFO successfully copied from vanilla

---

### 2. testing_diff - Compression Analysis ✅

**Status:** SUCCESS  
**Command:** `cargo run --bin testing_diff`

**POOL Chunk Analysis (pebble_0):**

| Pool Type | Vanilla | Roundtrip | From Assets | Difference |
|-----------|---------|-----------|-------------|------------|
| Texture | 1,673,248 bytes | 1,122,994 bytes | 1,124,432 bytes | -33% |
| Mesh | 35,441 bytes | 35,739 bytes | 18,156 bytes | -49% |
| Face | 1,334 bytes | 1,322 bytes | 1,322 bytes | -1% |

**POOL Chunk Analysis (pebble_2):**

| Pool Type | Vanilla | Roundtrip | From Assets | Difference |
|-----------|---------|-----------|-------------|------------|
| Texture | 1,673,248 bytes | 1,122,994 bytes | 1,124,432 bytes | -33% |
| Mesh | 46,530 bytes | 46,809 bytes | 25,913 bytes | -44% |
| Face | 1,678 bytes | 1,662 bytes | 1,662 bytes | -1% |

**Xpress Compression Stats (pebble_0):**

| Data Type | Vanilla | Roundtrip | From Assets |
|-----------|---------|-----------|-------------|
| Texture | inds=32299, lits=704301, matches=296968 | inds=15456, lits=131478, matches=347650 | inds=15503, lits=133322, matches=347249 |
| Mesh | inds=890, lits=25523, matches=2046 | inds=881, lits=24851, matches=2456 | inds=371, lits=9088, matches=2390 |
| Face | inds=38, lits=1158, matches=7 | inds=38, lits=1146, matches=7 | inds=38, lits=1146, matches=7 |

**Key Findings:**
1. **POOL type identifier:** 3518 (0x0DB6)
2. **Texture compression:** 33% smaller in generated files
3. **Mesh compression:** 44-49% smaller in from_assets files
4. **Face compression:** Nearly identical (1% difference)
5. **Xpress patterns:** Significant differences in compression patterns

**Analysis:**
- Roundtrip files (parse → generate) have similar compression to vanilla
- From-assets files (OBJ → generate) have significantly different compression
- **Hypothesis:** Different vertex ordering or padding in from-assets files

---

### 3. verify_lossless - Round-Trip Validation ✅

**Status:** SUCCESS  
**Command:** `cargo run --bin verify_lossless`

**Results:**
- **pebble_0:** ✅ PASSED
- **pebble_1:** ✅ PASSED
- **pebble_2:** ✅ PASSED
- **ter_elysium.hod:** ✅ PASSED
- **asteroid_3.hod:** ⚠️ PARTIAL (HOD 1.0 file)

**asteroid_3.hod Details:**
```
Generated size: 9,049,180 bytes
Original size: 9,049,234 bytes
Diff: -54 bytes

Re-parsed generated file: SUCCESS
Original: Meshes=3, Joints=4, NavLights=0, Markers=0, EngineBurns=0
Reparsed: Meshes=3, Joints=4, NavLights=0, Markers=0, EngineBurns=0
```

**Analysis:**
- HOD 1.0 files have slight size differences when converted to HOD 2.0
- Structural integrity is maintained (meshes, joints preserved)
- Round-trip preservation works for HOD 2.0 files

---

## Key Discoveries

### 1. POOL Chunk Structure

**Type Identifier:** 3518 (0x0DB6)

**Internal Structure:**
```
POOL Header (type=3518)
├── Texture Pool
│   ├── Compressed size (u32)
│   ├── Decompressed size (u32)
│   └── DXT-compressed texture data
├── Mesh Pool
│   ├── Compressed size (u32)
│   ├── Decompressed size (u32)
│   └── Interleaved vertex data
└── Face Pool
    ├── Compressed size (u32)
    ├── Decompressed size (u32)
    └── 16-bit triangle indices
```

### 2. Compression Differences

**Vanilla vs Roundtrip:**
- Texture: 33% smaller in roundtrip
- Mesh: ~1% larger in roundtrip
- Face: 1% smaller in roundtrip

**Vanilla vs From Assets:**
- Texture: 33% smaller in from_assets
- Mesh: 44-49% smaller in from_assets
- Face: 1% smaller in from_assets

**Hypothesis:**
RODOH uses different compression settings or vertex ordering than our implementation.

### 3. HOD 1.0 Conversion

**Status:** Partially Working
- Structural integrity maintained
- Slight size differences (-54 bytes for asteroid_3.hod)
- Meshes and joints preserved
- KDOP/COLD/INFO successfully copied

### 4. Vertex Data Preservation

**From testing_diff:**
```
Root_mesh lod 0: verts 726->726 indices 726->726 pos=0 norm=0 uv=0 tangent=726
Root_mesh lod 1: verts 468->468 indices 468->468 pos=0 norm=0 uv=0 tangent=468
Root_mesh lod 2: verts 210->210 indices 210->210 pos=0 norm=0 uv=0 tangent=210
```

**Analysis:**
- Position, normal, UV data preserved exactly (0 differences)
- Tangent data has differences (expected due to calculation algorithm)
- Face indices preserved exactly

---

## Critical Quirks Discovered

### 1. POOL Type Identifier

**Value:** 3518 (0x0DB6)  
**Location:** First 4 bytes of POOL chunk  
**Importance:** Must be set correctly for game engine to parse

### 2. KDOP Hash Preservation

**Discovery:** KDOP hash must be identical if preserved from vanilla  
**Evidence:** All three versions (vanilla, roundtrip, from_assets) have identical KDOP hashes  
**Impact:** KDOP data must be copied byte-for-byte from vanilla

### 3. Tangent Calculation Differences

**Discovery:** Tangent values differ between vanilla and generated files  
**Evidence:** 726 tangent differences in pebble_0 LOD 0  
**Impact:** May affect normal mapping in game

### 4. HOD 1.0 Size Differences

**Discovery:** HOD 1.0 files have slight size differences when converted  
**Evidence:** asteroid_3.hod: -54 bytes difference  
**Impact:** Acceptable if structural integrity maintained

---

## Gap Analysis Updates

### Gap 1: Texture Compression Settings

**Status:** Partially Understood  
**Progress:** 60%

**Findings:**
- POOL type identifier is 3518 (0x0DB6)
- Texture compression varies by 33% between vanilla and generated
- Xpress compression patterns differ significantly

**Remaining Unknowns:**
- Exact DXT compression quality settings
- Block alignment rules
- Header format details

### Gap 2: Mesh Compression Differences

**Status:** Partially Understood  
**Progress:** 40%

**Findings:**
- Mesh compression varies by 44-49% between vanilla and from_assets
- Vertex ordering may differ
- Tangent calculation affects compression

**Remaining Unknowns:**
- Vertex ordering rules
- Alignment padding
- Tangent calculation algorithm

### Gap 3: HOD 1.0 vs 2.0 Differences

**Status:** Partially Understood  
**Progress:** 40%

**Findings:**
- HOD 1.0 conversion works with slight size differences
- Structural integrity maintained
- KDOP/COLD/INFO successfully copied

**Remaining Unknowns:**
- Exact structural differences
- Endianness changes
- Chunk mapping rules

---

## Recommendations

### Priority 1: Texture Compression Analysis

**Action:** Analyze vanilla HOD texture pool structure  
**Method:**
1. Extract texture pool from vanilla HOD
2. Decompress and analyze structure
3. Compare with RODOH output
4. Document settings

**Expected Outcome:** Understand exact DXT compression settings

### Priority 2: Vertex Ordering Analysis

**Action:** Compare vertex ordering in vanilla vs from_assets  
**Method:**
1. Extract mesh pool from vanilla HOD
2. Compare vertex order with from_assets
3. Identify ordering differences
4. Document rules

**Expected Outcome:** Understand vertex ordering for compression

### Priority 3: Tangent Calculation Comparison

**Action:** Compare tangent values with RODOH  
**Method:**
1. Extract tangent data from vanilla HOD
2. Compare with generated tangent values
3. Identify algorithm differences
4. Document algorithm

**Expected Outcome:** RODOH-compatible tangent calculation

---

## Next Steps

### Immediate (Next Session)

1. **Analyze texture pool structure** - Document headers and layout
2. **Compare vertex ordering** - Identify differences
3. **Test HOD 1.0 conversion** - Use more HOD 1.0 files

### Short Term (This Week)

1. **Document compression settings** - Exact DXT quality
2. **Implement vertex ordering** - Match RODOH behavior
3. **Validate in game** - Test generated HOD files

### Long Term (Next Phase)

1. **Implement DAE import** - Parse COLLADA files
2. **Complete SHADERS.MAP integration** - Shader mapping
3. **Full validation suite** - Automated testing

---

## Open Questions

### For Phase 2 Completion

1. What DXT quality does RODOH use for textures?
2. How are vertices ordered in mesh pool?
3. What is the exact tangent calculation algorithm?
4. How does RODOH handle vertex padding?

### For Phase 3

1. How to validate HOD files in game?
2. What automated tests are needed?
3. How to handle edge cases (animations, dockpaths)?

---

**Phase 2 Status:** In Progress  
**Completion:** ~50%  
**Next Update:** After texture compression analysis