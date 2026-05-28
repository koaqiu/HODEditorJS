# HODOR Replication Test Results

## Status: Two Issues Identified

**Created:** 2026-05-28  
**Test Cases:** ter_pharos, ter_centaur  
**Result:** 0/2 passed (0% success rate)  

---

## Test Results Summary

| Test Case | HODOR Size | Generated Size | Meshes (HODOR) | Meshes (Generated) | Status |
|-----------|------------|----------------|----------------|-------------------|--------|
| ter_pharos | 236,648 bytes | 10,810 bytes | 3 | 2 | ❌ FAILED |
| ter_centaur | 232,860 bytes | 119,193 bytes | 4 | 2 | ❌ FAILED |

---

## Issue 1: ter_pharos - Mesh Count Mismatch

**Error:** Mesh count mismatch: 3 vs 2

**HODOR Output:**
- 3 meshes (1,488, 624, 180 vertices)
- 1 material
- Texture pool: 183,988 comp / 524,256 decomp
- Mesh pool: 39,015 comp / 153,900 decomp

**Our Output:**
- 2 meshes
- 1 material
- Generated file is much smaller (10,810 vs 236,648 bytes)

**Possible Causes:**
- DAE parser not parsing all mesh LODs
- Mesh deduplication removing LODs
- Incorrect mesh naming

---

## Issue 2: ter_centaur - BURN Chunk Error

**Error:** Failed to re-parse: Error in BURN (data size=40): BURN total_vertices exceeds buffer space

**HODOR Output:**
- 4 meshes (multiple parts per mesh)
- 2 materials
- 1 engine burn
- Texture pool: 65,852 comp / 87,424 decomp
- Mesh pool: 125,480 comp / 1,218,456 decomp

**Our Output:**
- 2 meshes
- 2 materials
- Generated file is smaller (119,193 vs 232,860 bytes)
- BURN chunk generation error

**Possible Causes:**
- BURN chunk generation bug
- Incorrect vertex count calculation
- Buffer size mismatch

---

## Root Cause Analysis

### Issue 1: Mesh Count Mismatch

**Possible Root Causes:**

1. **DAE Parser Not Parsing All LODs**
   - The DAE parser might not be parsing all mesh LODs
   - HODOR has 3 LODs, we only have 2

2. **Mesh Deduplication**
   - The mesh deduplication logic might be removing LODs
   - Check if LODs are being incorrectly combined

3. **Incorrect Mesh Naming**
   - Mesh names might not match HODOR's naming convention
   - Check if mesh names are correct

### Issue 2: BURN Chunk Error

**Possible Root Causes:**

1. **BURN Chunk Generation Bug**
   - The BURN chunk generation logic might be incorrect
   - Check if vertex count is being calculated correctly

2. **Buffer Size Mismatch**
   - The generated BURN chunk might have incorrect buffer size
   - Check if buffer allocation is correct

3. **Vertex Data Corruption**
   - Vertex data might be corrupted during generation
   - Check if vertex data is being written correctly

---

## Next Steps

### Immediate

1. **Investigate mesh count mismatch** - Check DAE parser and mesh deduplication
2. **Investigate BURN chunk error** - Check BURN chunk generation logic
3. **Fix issues** - Implement fixes for both problems

### Short Term

1. **Test with more HODOR files** - Expand test cases
2. **Compare compression settings** - Match HODOR's compression
3. **Validate in game** - Test generated HOD files

---

**Document Version:** 1.0  
**Last Updated:** 2026-05-28  
**Status:** Two Issues Identified