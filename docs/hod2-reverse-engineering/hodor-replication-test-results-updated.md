# HODOR Replication Test Results (Updated)

## Superseded

This document is historical and reflects an earlier failing state. Current status is tracked in:

- `PROGRESS.md`
- `hodor-replication-test-success.md`
- `from-assets-to-hod-analysis.md`

Current source-asset HODOR replication tests pass 2/2 and do not use `model.json` as implementation source data.

## Status: Test Working with OBJ Files

**Created:** 2026-05-28  
**Test Cases:** ter_pharos, ter_centaur  
**Result:** 0/2 passed (0% success rate)  

---

## Test Results Summary

| Test Case | HODOR Size | Generated Size | Meshes (HODOR) | Meshes (Generated) | Status |
|-----------|------------|----------------|----------------|-------------------|--------|
| ter_pharos | 236,648 bytes | - | 3 | - | Pending |
| ter_centaur | 232,860 bytes | 478,481 bytes | 4 | 4 | ❌ FAILED |

---

## ter_centaur Test Results

### HODOR Output
- **Size:** 232,860 bytes
- **Meshes:** 4 (3,845, 778, 3,845, 778 vertices)
- **Materials:** 2
- **Joints:** 7
- **NavLights:** 1
- **Engine Burns:** 1
- **Markers:** 5
- **Collision:** 1

### Our Output
- **Size:** 478,481 bytes (2x larger than HODOR)
- **Meshes:** 4 (4,704 vertices each)
- **Materials:** 2
- **Joints:** 1 (only root joint)
- **NavLights:** 0
- **Engine Burns:** 0
- **Markers:** 0
- **Collision:** 0

### Comparison

| Feature | HODOR | Our Output | Match |
|---------|-------|------------|-------|
| File Size | 232,860 bytes | 478,481 bytes | ❌ |
| Mesh Count | 4 | 4 | ✅ |
| Material Count | 2 | 2 | ✅ |
| Joint Count | 7 | 1 | ❌ |
| NavLight Count | 1 | 0 | ❌ |
| Engine Burn Count | 1 | 0 | ❌ |
| Marker Count | 5 | 0 | ❌ |
| Collision | 1 | 0 | ❌ |

---

## Issues Identified

### Issue 1: Joint Count Mismatch

**HODOR:** 7 joints  
**Our Output:** 1 joint (only root)

**Root Cause:** Our current implementation only creates a single root joint, but HODOR creates multiple joints for complex models.

**Solution:** Implement multi-joint support from DAE/OBJ files.

### Issue 2: File Size Difference

**HODOR:** 232,860 bytes  
**Our Output:** 478,481 bytes (2x larger)

**Root Cause:** Our texture and mesh compression is different from HODOR.

**Solution:** Match HODOR's compression settings.

### Issue 3: Missing Components

**HODOR:** Has navlights, engine burns, markers, collision  
**Our Output:** Missing these components

**Root Cause:** Our current implementation doesn't parse these components from DAE/OBJ files.

**Solution:** Implement parsing for navlights, engine burns, markers, and collision.

---

## Root Cause Analysis

### Why Mesh Count is Correct

The mesh count is correct because:
1. We parsed 4 OBJ files (Root_mesh_lod0.obj, Root_mesh_lod1.obj, Root_mesh_lod2.obj, Root_mesh_lod3.obj)
2. Each OBJ file becomes one mesh
3. This matches HODOR's output (4 meshes)

### Why Joint Count is Wrong

The joint count is wrong because:
1. We only create a single root joint
2. HODOR creates multiple joints for complex models
3. We need to parse joint hierarchy from DAE files

### Why File Size is Different

The file size is different because:
1. Our texture compression is different from HODOR
2. Our mesh compression is different from HODOR
3. We need to match HODOR's compression settings

---

## Next Steps

### Immediate

1. **Investigate joint hierarchy parsing** - How to parse joints from DAE files
2. **Investigate compression settings** - Match HODOR's compression
3. **Test with ter_pharos** - Complete the test

### Short Term

1. **Implement multi-joint support** - Parse joint hierarchy from DAE
2. **Implement navlight, engine burn, marker support** - Parse from DAE
3. **Match compression settings** - Use same compression as HODOR

---

**Document Version:** 1.1  
**Last Updated:** 2026-05-28  
**Status:** Test Working with OBJ Files
