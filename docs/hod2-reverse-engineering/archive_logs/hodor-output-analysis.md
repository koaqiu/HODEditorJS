# HODOR Output Analysis

## Status: Analysis Complete

**Created:** 2026-05-28  
**Test Case:** ter_pharos (complex HOD created by HODOR)  

---

## Executive Summary

Analyzed ter_pharos.hod, a complex HOD 2.0 file created by HODOR. This provides a reference for how HODOR creates HOD 2.0 files with multiple textures, meshes, navlights, and collision data.

---

## ter_pharos.hod Structure (HODOR-generated)

### File Information
- **Location:** `/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld/HWRM_FSFC/source/ship/ter_pharos/ter_pharos.hod`
- **Size:** 236,648 bytes
- **Version:** 512 (HOD 2.0)

### Chunk Structure
```
VERS (4 bytes)
NAME (26 bytes)
POOL (229,984 bytes)
├── Texture pool: comp=183,988, decomp=524,256
├── Mesh pool: comp=39,015, decomp=153,900
└── Face pool: comp=6,953, decomp=10,896
HVMD (0 bytes)
├── LMIP (79 bytes) × 3 LODs
├── STAT (92 bytes)
└── MULT (174 bytes)
DTRM (0 bytes)
├── HIER (583 bytes)
├── NAVL (580 bytes)
├── KDOP (1,588 bytes)
├── COLD (8 bytes)
│   ├── BBOX (24 bytes)
│   ├── BSPH (16 bytes)
│   └── TRIS (8 bytes)
└── SCAR (3,063 bytes)
INFO (49 bytes)
```

### Material Information
```json
{
  "name": "pharos.bmp",
  "shader": "ship",
  "texture_maps": [
    "Pharos_DIFF",
    "Pharos_GLOW",
    "Pharos_TEAM"
  ]
}
```

### Mesh Information
- **Mesh 0 (LOD 0):** 1,488 vertices, 1,488 indices
- **Mesh 1 (LOD 1):** 624 vertices, 624 indices
- **Mesh 2 (LOD 2):** 180 vertices, 180 indices

### Node Information
- **Joints:** 10
- **NavLights:** 9
- **Collision Meshes:** 1
- **Dockpaths:** 0

---

## First Vertex Analysis

### ter_pharos (HODOR)
```
Position: (-4.67896, 7.82711, -0.485433)
Normal: (-1.0, 0.0, 0.0)
UV: (0.52723, -1.93617)
Tangent: (0.0, -0.6033719, -0.7974599)
Binormal: (0.0, -0.79713917, 0.6037957)
```

### pebble_0 (Vanilla)
```
Position: (-0.17662, -1.65612, 2.464357)
Normal: (-0.225789, -0.743329, 0.629668)
UV: (0.5, 0.0)
Tangent: (0.9705058, -0.22769131, 0.0792161)
Binormal: (-0.038397547, 0.65264744, 0.7566881)
```

### pebble_0 (Generated)
```
Position: (-0.17662, -1.65612, 2.464357)
Normal: (-0.22578895, -0.7433288, 0.6296679)
UV: (0.5, 0.0)
Tangent: (0.9705058, -0.22769143, 0.07921616)
Binormal: (0.08448632, 0.6289825, 0.7728151)
```

---

## Key Observations

### 1. Texture Pool Compression

**ter_pharos (HODOR):**
- Texture pool: 183,988 bytes compressed / 524,256 decompressed
- Compression ratio: 0.35 (65% compression)

**pebble_0 (Vanilla):**
- Texture pool: 1,673,248 bytes compressed / 2,097,120 decompressed
- Compression ratio: 0.80 (20% compression)

**pebble_0 (Generated):**
- Texture pool: 1,124,443 bytes compressed / 2,097,120 decompressed
- Compression ratio: 0.54 (46% compression)

**Observation:** HODOR uses different compression settings for different models. ter_pharos has much higher compression (65%) than pebble_0 (20%).

### 2. Mesh Pool Structure

**ter_pharos (HODOR):**
- Mesh pool: 39,015 bytes compressed / 153,900 decompressed
- Compression ratio: 0.25 (75% compression)

**pebble_0 (Vanilla):**
- Mesh pool: 35,441 bytes compressed / 71,040 decompressed
- Compression ratio: 0.50 (50% compression)

**pebble_0 (Generated):**
- Mesh pool: 36,019 bytes compressed / 71,040 decompressed
- Compression ratio: 0.51 (49% compression)

**Observation:** Mesh compression is similar between vanilla and generated, but HODOR's ter_pharos has much higher compression.

### 3. Binormal Values

**ter_pharos (HODOR):**
- Binormal: (0.0, -0.79713917, 0.6037957)

**pebble_0 (Vanilla):**
- Binormal: (-0.038397547, 0.65264744, 0.7566881)

**pebble_0 (Generated):**
- Binormal: (0.08448632, 0.6289825, 0.7728151)

**Observation:** Binormal values are completely different between models, which is expected since they have different geometry.

### 4. TAGS Chunk

**ter_pharos (HODOR):**
- MULT has FORM TAGS child chunk

**pebble_0 (Vanilla):**
- MULT does NOT have FORM TAGS child chunk

**Observation:** HODOR includes TAGS chunk in some models but not others.

---

## Implications for Our Implementation

### 1. Texture Compression

**Issue:** Our generated pebble_0.hod has 33% smaller texture pool than vanilla.

**Possible Causes:**
- Different DXT compression quality settings
- Different compression algorithm parameters
- Different texture format handling

**Solution:** Match HODOR's compression settings for each model type.

### 2. Binormal Calculation

**Issue:** Our generated pebble_0.hod has different binormal values than vanilla.

**Possible Causes:**
- Different cross product order
- Different handedness calculation
- Different tangent space algorithm

**Solution:** Analyze HODOR's binormal calculation algorithm.

### 3. Model-Specific Settings

**Observation:** HODOR uses different compression settings for different models.

**Implication:** We cannot use one-size-fits-all compression settings. We need to match HODOR's settings for each model type.

---

## Test Case Value

### Why ter_pharos is Valuable

1. **Created by HODOR** - This is the actual tool that creates HOD 2.0 files
2. **Complex structure** - Multiple textures, meshes, navlights, collision
3. **Reference implementation** - Can compare our output with HODOR's output
4. **Different compression** - Shows HODOR uses model-specific compression settings

### How to Use This Test Case

1. **Extract ter_pharos.DAE and TGA files**
2. **Run HODOR to create HOD 2.0** (we already have this)
3. **Compare with our generated HOD**
4. **Identify differences in compression and binormal calculation**

---

## Next Steps

### Immediate

1. **Analyze HODOR's binormal calculation** - Extract and compare
2. **Analyze HODOR's texture compression** - Identify settings
3. **Implement fixes** - Match HODOR's behavior

### Short Term

1. **Test with ter_pharos** - Generate HOD from DAE+TGA
2. **Compare with HODOR output** - Verify our implementation
3. **Validate in game** - Ensure correct rendering

---

**Document Version:** 1.0  
**Last Updated:** 2026-05-28  
**Status:** Analysis Complete