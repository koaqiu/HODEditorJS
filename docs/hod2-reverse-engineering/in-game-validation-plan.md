# In-Game Validation Plan

## Status: Ready for Execution

**Created:** 2026-05-28  
**Priority:** HIGH  
**Estimated Time:** 1-2 hours  

---

## Executive Summary

This plan outlines the steps to validate that generated pebble_0 HOD files work correctly in Homeworld Remastered. Previous tests showed vertex explosion issues, which should be fixed by our Gram-Schmidt orthogonalization implementation.

---

## Background

### Previous Issues

From `progress_log.md`:
- **Issue:** Vertex explosion/distortion in-game
- **Symptom:** Vertices went to infinity
- **Root Cause:** Normal.W component (tangent handedness) was 0.0
- **Fix:** Gram-Schmidt orthogonalization (implemented in Phase 2)

### Current Status

**Implemented Fixes:**
1. ✅ Gram-Schmidt orthogonalization for tangent calculation
2. ✅ Handedness calculation for binormal direction
3. ✅ SHADERS.MAP integration for shader selection
4. ✅ Validation suite passing (3/3 tests)

**Expected Result:** Generated HOD files should now render correctly in game.

---

## Test Files

### Files to Test

**Location:** `testing/pebble_0/`

1. **pebble_0_vanilla.hod** - Original vanilla HOD (reference)
2. **pebble_0_roundtrip.hod** - Vanilla → parse → generate
3. **pebble_0_from_assets.hod** - OBJ + TGA + JSON → generate

### File Sizes

| File | Size | Description |
|------|------|-------------|
| pebble_0_vanilla.hod | 1,680,248 bytes | Original |
| pebble_0_roundtrip.hod | 1,173,862 bytes | Roundtrip |
| pebble_0_from_assets.hod | 1,175,305 bytes | From assets |

---

## Testing Procedure

### Step 1: Prepare Test Environment

**1.1 Backup Original Files**
```bash
# Backup vanilla HOD
cp /run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/uncompressed_bigs/HWRM/pebble/pebble_0/pebble_0.hod \
   /run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/uncompressed_bigs/HWRM/pebble/pebble_0/pebble_0.hod.backup
```

**1.2 Copy Generated Files**
```bash
# Copy from_assets HOD to game directory
cp /run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/testing/pebble_0/pebble_0_from_assets.hod \
   /run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/uncompressed_bigs/HWRM/pebble/pebble_0/pebble_0.hod
```

### Step 2: Test Vanilla HOD

**2.1 Load Game**
- Launch Homeworld Remastered
- Start a new game or load a saved game

**2.2 Navigate to Pebble**
- Access the pebble_0 object
- Observe rendering

**2.3 Expected Result**
- Pebble renders correctly
- No vertex explosion
- Textures display properly
- Normal mapping works

### Step 3: Test Roundtrip HOD

**3.1 Copy Roundtrip HOD**
```bash
cp /run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/testing/pebble_0/pebble_0_roundtrip.hod \
   /run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/uncompressed_bigs/HWRM/pebble/pebble_0/pebble_0.hod
```

**3.2 Load Game**
- Restart Homeworld Remastered (or reload)
- Navigate to pebble_0

**3.3 Expected Result**
- Pebble renders identically to vanilla
- No visual differences
- All textures correct

### Step 4: Test From-Assets HOD

**4.1 Copy From-Assets HOD**
```bash
cp /run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/testing/pebble_0/pebble_0_from_assets.hod \
   /run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/uncompressed_bigs/HWRM/pebble/pebble_0/pebble_0.hod
```

**4.2 Load Game**
- Restart Homeworld Remastered
- Navigate to pebble_0

**4.3 Expected Result**
- Pebble renders correctly
- No vertex explosion
- Textures display properly
- Normal mapping works

---

## Validation Checklist

### Visual Validation

- [ ] Pebble renders without vertex explosion
- [ ] Pebble shape is correct (not distorted)
- [ ] Textures display correctly
- [ ] Normal mapping works (lighting looks correct)
- [ ] No visual artifacts

### Technical Validation

- [ ] Game loads without crashes
- [ ] No error messages in log
- [ ] Object is selectable/interactive
- [ ] No performance issues

### Comparison Validation

- [ ] Vanilla HOD renders correctly
- [ ] Roundtrip HOD matches vanilla
- [ ] From-assets HOD renders correctly

---

## Expected Issues & Solutions

### Issue 1: Vertex Explosion

**Symptom:** Vertices go to infinity, object disappears or distorts  
**Cause:** Normal.W component is 0.0  
**Solution:** Gram-Schmidt orthogonalization (already implemented)

### Issue 2: Textures Missing

**Symptom:** Object renders white or gray  
**Cause:** Texture files not found  
**Solution:** Ensure TGA files are in correct location

### Issue 3: Normal Mapping Wrong

**Symptom:** Lighting looks incorrect  
**Cause:** Tangent/binormal calculation error  
**Solution:** Verify tangent calculation (already implemented)

### Issue 4: Game Crash

**Symptom:** Game crashes on load  
**Cause:** HOD file corruption  
**Solution:** Verify HOD structure with validation suite

---

## Success Criteria

### Primary Success

- [ ] From-assets HOD renders correctly in game
- [ ] No vertex explosion
- [ ] Textures display properly
- [ ] Normal mapping works

### Secondary Success

- [ ] Roundtrip HOD matches vanilla
- [ ] No visual differences
- [ ] No performance impact

### Tertiary Success

- [ ] All pebble variants work (pebble_0, pebble_1, pebble_2)
- [ ] No crashes
- [ ] Ready for HOD 1.0 testing

---

## Risk Assessment

### Risk 1: Vertex Explosion Persists

**Probability:** Low (Gram-Schmidt should fix it)  
**Mitigation:** Check Normal.W component in generated HOD

### Risk 2: Textures Not Loading

**Probability:** Low (textures are copied correctly)  
**Mitigation:** Verify file paths in game

### Risk 3: Game Crash

**Probability:** Low (validation suite passes)  
**Mitigation:** Check HOD structure with diagnostic tools

---

## Post-Validation Steps

### If Validation Passes

1. ✅ Document success in PROGRESS.md
2. ✅ Proceed to HOD 1.0 testing
3. ✅ Expand to more complex models

### If Validation Fails

1. ⏳ Analyze error messages
2. ⏳ Check HOD structure with diagnostic tools
3. ⏳ Compare with vanilla HOD byte-by-byte
4. ⏳ Fix issues and re-test

---

## Tools for Debugging

### Diagnostic Binaries

```bash
# Dump HOD structure
cargo run --bin dump_chunks -- <path_to_hod>

# Compare two HOD files
cargo run --bin compare_hods -- <file1> <file2>

# Dump pool data
cargo run --bin dump_pool -- <path_to_hod>

# Analyze compression
cargo run --bin testing_diff
```

### Log Analysis

**Game Log Location:**
```
%USERPROFILE%\Documents\My Games\Homeworld\HomeworldRM\bin\Release\bin32\log.txt
```

**Look for:**
- Error messages
- Warning messages
- Loading messages

---

## Timeline

### Immediate (Today)

1. **Prepare test environment** (10 minutes)
2. **Test vanilla HOD** (5 minutes)
3. **Test roundtrip HOD** (5 minutes)
4. **Test from-assets HOD** (5 minutes)
5. **Document results** (10 minutes)

**Total:** 35 minutes

### Short Term (This Week)

1. **Test pebble_1 and pebble_2** (30 minutes)
2. **Test with more complex models** (1 hour)
3. **Document findings** (30 minutes)

---

**Plan Version:** 1.0  
**Last Updated:** 2026-05-28  
**Status:** Ready for Execution