# Phase 2: Gap Analysis - HOD 2.0 Creation

## Status: In Progress

**Started:** 2026-05-27  
**Last Updated:** 2026-05-27  
**Current Focus:** Texture Compression & HOD 1.0 Differences

---

## Executive Summary

Phase 2 focuses on identifying and documenting gaps in our understanding of HOD 2.0 creation. Based on initial analysis, we have identified three critical gaps that need to be addressed.

---

## Critical Gaps Identified

### Gap 1: Texture Compression Settings

**Status:** Partially Understood  
**Priority:** HIGH

**Current Understanding:**
- POOL chunk contains compressed texture data
- Uses Microsoft Xpress LZ77 compression
- Compression ratio varies (~4:1 typical)
- **POOL type identifier:** 3518 (0x0DB6)

**Evidence from testing_diff:**

**pebble_0:**
- Vanilla: 1,673,248 bytes compressed / 2,097,120 decompressed
- Roundtrip: 1,122,994 bytes compressed / 2,097,120 decompressed
- From assets: 1,124,432 bytes compressed / 2,097,120 decompressed
- **33% difference** between vanilla and roundtrip/from_assets

**pebble_2:**
- Vanilla: 1,673,248 bytes compressed / 2,097,120 decompressed
- Roundtrip: 1,122,994 bytes compressed / 2,097,120 decompressed
- From assets: 1,124,432 bytes compressed / 2,097,120 decompressed

**Xpress Compression Stats (pebble_0):**
- Vanilla texture: inds=32299, lits=704301, matches=296968
- Roundtrip texture: inds=15456, lits=131478, matches=347650
- **Significant difference** in compression patterns

**Unknowns:**
1. **DXT compression quality** - What quality level does RODOH use?
2. **Block alignment** - How are textures aligned in POOL chunk?
3. **Header format** - What headers precede compressed texture data?
4. **Multiple textures** - How are multiple textures packed together?

**Next Steps:**
1. Analyze vanilla HOD texture pool structure
2. Compare with RODOH output
3. Document exact compression settings

---

### Gap 2: Mesh Compression Differences

**Status:** Partially Understood  
**Priority:** HIGH

**Current Understanding:**
- Mesh data is compressed in POOL chunk
- Uses Microsoft Xpress compression
- Compression varies based on vertex data

**Evidence from testing_diff:**

**pebble_0:**
- Vanilla: 35,441 bytes / 71,040 decompressed
- Roundtrip: 35,739 bytes / 71,040 decompressed
- From assets: 18,156 bytes / 71,040 decompressed
- **Significant difference** in from_assets mesh compression

**pebble_2:**
- Vanilla: 46,530 bytes / 89,856 decompressed
- Roundtrip: 46,809 bytes / 89,856 decompressed
- From assets: 25,913 bytes / 89,856 decompressed

**Xpress Compression Stats (pebble_0):**
- Vanilla mesh: inds=890, lits=25523, matches=2046
- Roundtrip mesh: inds=881, lits=24851, matches=2456
- From assets mesh: inds=371, lits=9088, matches=2390

**Analysis:**
- Roundtrip mesh compression is very similar to vanilla (good sign)
- From assets mesh compression is significantly different
- **Hypothesis:** From assets uses different vertex ordering or padding

**Unknowns:**
1. **Vertex ordering** - How are vertices ordered in mesh pool?
2. **Vertex padding** - Is there alignment padding?
3. **Index compression** - How are indices compressed?
4. **Tangent calculation** - Does tangent calculation affect compression?

**Next Steps:**
1. Analyze vertex ordering in vanilla vs from_assets
2. Compare tangent values
3. Document mesh compression rules

---

### Gap 3: HOD 1.0 vs 2.0 Structural Differences

**Status:** Not Understood  
**Priority:** HIGH

**Current Understanding:**
- HOD 1.0 files exist in `testing/` directory
- Parser can read both versions
- Conversion logic exists but is incomplete

**Unknowns:**
1. **Structural differences** - What chunks differ between versions?
2. **Endianness changes** - How does endianness differ?
3. **Chunk mapping** - How do HOD 1.0 chunks map to HOD 2.0?
4. **Metadata handling** - How is metadata preserved?

**Evidence:**
- HOD 1.0 files have different chunk structure
- Parser has `is_v2` flag to detect version
- Conversion logic in `hod.rs` is incomplete

**Next Steps:**
1. Analyze HOD 1.0 file structure
2. Compare with HOD 2.0 structure
3. Document conversion rules

---

## Additional Gaps

### Gap 4: RODOH Tangent Calculation

**Status:** Not Understood  
**Priority:** MEDIUM

**Current Understanding:**
- Tangents are stored in vertex data (16 bytes)
- Used for normal mapping
- Calculator exists in `compiler.rs`

**Unknowns:**
1. **Algorithm** - What algorithm does RODOH use?
2. **Handedness** - How is tangent handedness determined?
3. **Bitangent calculation** - How are bitangents computed?
4. **Default values** - What are default tangent values?

**Evidence:**
- Vanilla HOD has tangent data in vertices
- Roundtrip HOD has different tangent values
- `compiler.rs` has basic tangent calculation

**Next Steps:**
1. Analyze vanilla tangent data
2. Compare with RODOH output
3. Document algorithm

---

### Gap 5: SHADERS.MAP Integration

**Status:** Not Understood  
**Priority:** LOW

**Current Understanding:**
- `SHADERS.MAP` file defines shader mappings
- RODOH uses it for texture channel mapping
- Located at `GBXTools/HODOR/SHADERS.MAP`

**Unknowns:**
1. **Mapping rules** - How are textures mapped to shaders?
2. **Channel mapping** - How are texture channels interpreted?
3. **Shader selection** - How are shaders chosen?

**Next Steps:**
1. Analyze `SHADERS.MAP` format
2. Document mapping rules
3. Implement in parser

### Gap 6: Pool Chunk Internal Structure

**Status:** Partially Understood  
**Priority:** MEDIUM

**Current Understanding:**
- POOL contains compressed data
- Has texture, mesh, and face pools
- Uses Microsoft Xpress compression
- **POOL type identifier:** 3518 (0x0DB6)

**Unknowns:**
1. **Header format** - What headers exist in POOL chunk?
2. **Pool boundaries** - How are pools separated?
3. **Alignment rules** - What alignment is required?
4. **Metadata** - What metadata is stored?

**Evidence:**
- `testing_diff` shows POOL type=3518
- Decompression works correctly
- Headers are not documented

**Next Steps:**
1. Analyze POOL chunk structure
2. Document headers and boundaries
3. Create specification

---

## Test Case Analysis

### Available Test Cases

**pebble_0:**
- **Structure:** 2 LODs, 1 material, 3 textures
- **Size:** 1,680,248 bytes (vanilla), 1,130,002 bytes (roundtrip)
- **Status:** Good for basic testing

**pebble_1:**
- **Structure:** 3 LODs, 1 material, 3 textures
- **Status:** Good for LOD testing

**pebble_2:**
- **Structure:** Similar to pebble_0/pebble_1
- **Status:** Good for validation

### Test Case Gaps

**Missing Test Cases:**
1. **HOD 1.0 files** - Need to test conversion
2. **Complex ships** - Need multi-mesh, multi-material
3. **Animated ships** - Need animation testing
4. **Ships with dockpaths** - Need dockpath testing

**Recommendation:** Use `testing/pebble_0/` as primary test case due to simplicity.

---

## Compression Analysis

### Microsoft Xpress Algorithm

**Implementation:** `parser/src/xpress.rs`

**Current Status:**
- Decompression works correctly
- Compression works but may differ from RODOH
- Block size: 256 bytes (typical)

**Compression Ratios:**

| Data Type | Vanilla | Roundtrip | Difference |
|-----------|---------|-----------|------------|
| Texture   | 1,528,396 bytes | 977,396 bytes | -36% |
| Mesh      | 114,516 bytes | 114,516 bytes | 0% |
| Face      | 344 bytes | 332 bytes | -3% |

**Analysis:**
- Mesh compression is identical (good sign)
- Face compression varies slightly (acceptable)
- Texture compression varies significantly (needs investigation)

**Hypothesis:**
RODOH uses different DXT compression quality settings than our implementation.

---

## HOD 1.0 vs 2.0 Analysis

### Structural Differences

**HOD 1.0:**
- Different chunk structure
- May have top-level FORM wrapper
- Different endianness rules
- Different metadata format

**HOD 2.0:**
- Flat chunk sequence (no FORM wrapper)
- Big-Endian headers, Little-Endian payloads
- POOL chunk with compression
- Specific chunk order (VERS, NAME, POOL, HVMD, DTRM, INFO)

### Conversion Requirements

**To convert HOD 1.0 → 2.0:**
1. Parse HOD 1.0 structure
2. Map chunks to HOD 2.0 equivalents
3. Apply compression to textures/meshes
4. Restructure to HOD 2.0 format
5. Validate output

---

## RODOH Analysis

### Tangent Calculation

**Current Implementation:** `compiler.rs` lines 127-179

**Algorithm:**
1. Calculate face tangents from UV coordinates
2. Accumulate tangent/bitangent per vertex
3. Normalize accumulated values
4. Gram-Schmidt orthogonalization

**Issues:**
- May not match RODOH exactly
- Handedness handling unclear
- Default values may differ

**Next Steps:**
1. Compare tangent values with RODOH
2. Document differences
3. Adjust algorithm if needed

---

## Recommendations

### Priority 1: Texture Compression

**Action:** Investigate texture compression settings
**Method:**
1. Extract texture pool from vanilla HOD
2. Decompress and analyze structure
3. Compare with RODOH output
4. Document settings

**Expected Outcome:** Understand exact DXT compression settings

### Priority 2: HOD 1.0 Conversion

**Action:** Test HOD 1.0 → 2.0 conversion
**Method:**
1. Use `asteroid_3.hod` (HOD 1.0)
2. Convert using parser
3. Compare with vanilla
4. Document differences

**Expected Outcome:** Working HOD 1.0 → 2.0 conversion

### Priority 3: Minimal Test Case

**Action:** Create minimal HOD 2.0 from scratch
**Method:**
1. Create simplest possible HOD (1 mesh, 1 material)
2. Use test assets from `testing/pebble_0/`
3. Generate HOD 2.0
4. Validate in game

**Expected Outcome:** Working HOD 2.0 creation from assets

---

## Next Steps

### Immediate (Next Session)

1. **Analyze texture pool structure** - Document headers and layout
2. **Test HOD 1.0 conversion** - Use `asteroid_3.hod`
3. **Create minimal test case** - Generate HOD from `testing/pebble_0/`

### Short Term (This Week)

1. **Document compression settings** - Exact DXT quality
2. **Implement HOD 1.0 conversion** - Complete conversion logic
3. **Validate in game** - Test generated HOD files

### Long Term (Next Phase)

1. **Implement DAE import** - Parse COLLADA files
2. **Complete SHADERS.MAP integration** - Shader mapping
3. **Full validation suite** - Automated testing

---

## Documentation Gaps

### Missing Documentation

1. **POOL chunk internal structure** - Headers, boundaries
2. **HOD 1.0 format specification** - Complete format
3. **RODOH tangent algorithm** - Exact calculation
4. **Texture compression settings** - DXT quality

### Documentation Needed

1. **POOL chunk spec** - Create detailed specification
2. **HOD 1.0 vs 2.0 comparison** - Document differences
3. **Compression guide** - Settings and parameters
4. **Conversion tutorial** - Step-by-step guide

---

## Open Questions

### For Phase 2 Completion

1. What DXT quality does RODOH use for textures?
2. How are multiple textures packed in POOL chunk?
3. What are the exact HOD 1.0 → 2.0 conversion rules?
4. How does RODOH calculate tangent vectors?

### For Phase 3

1. How to validate HOD files in game?
2. What automated tests are needed?
3. How to handle edge cases (animations, dockpaths)?

---

**Phase 2 Status:** In Progress  
**Completion:** ~30%  
**Next Update:** After texture compression analysis