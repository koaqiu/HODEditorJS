# HODOR Test Cases Analysis

## Status: Analysis Complete

**Created:** 2026-05-28  
**Test Cases:** ter_pharos, ter_centaur (both created by HODOR)  

---

## Executive Summary

Analyzed two complex HOD 2.0 files created by HODOR. These provide reference implementations for how HODOR creates HOD 2.0 files with different complexity levels.

---

## Test Case 1: ter_pharos

### File Information
- **Location:** `/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld/HWRM_FSFC/source/ship/ter_pharos/ter_pharos.hod`
- **Size:** 236,648 bytes
- **DAE+TGA Location:** `/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/current_project_processing/ship_converted/ter_pharos/`

### Structure
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
- **Engine Burns:** 0
- **Markers:** 0

---

## Test Case 2: ter_centaur

### File Information
- **Location:** `/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld/HWRM_FSFC/source/ship/ter_centaur/ter_centaur.hod`
- **Size:** 232,860 bytes
- **DAE+TGA Location:** `/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/current_project_processing/ship_converted/ter_centaur/`

### Structure
```
VERS (4 bytes)
NAME (26 bytes)
POOL (219,495 bytes)
├── Texture pool: comp=65,852, decomp=87,424
├── Mesh pool: comp=125,480, decomp=1,218,456
└── Face pool: comp=28,135, decomp=65,286
HVMD (0 bytes)
├── LMIP (74 bytes)
├── LMIP (36 bytes)
├── STAT (48 bytes)
├── STAT (46 bytes)
└── MULT (288 bytes)
DTRM (0 bytes)
├── HIER (449 bytes)
├── BURN (100 bytes)
├── NAVL (59 bytes)
├── MRKS (339 bytes)
├── KDOP (1,564 bytes)
├── COLD (8 bytes)
│   ├── BBOX (24 bytes)
│   ├── BSPH (16 bytes)
│   └── TRIS (8 bytes)
└── SCAR (9,991 bytes)
INFO (49 bytes)
```

### Material Information
```json
{
  "name": "ter_centaur",
  "shader": "ship",
  "texture_maps": [
    "support01_DIFF",
    "transparent_DIFF"
  ]
}
```

### Mesh Information
- **Mesh 0 (LOD 0):** Multiple meshes
- **Mesh 1 (LOD 1):** Multiple meshes

### Node Information
- **Joints:** Multiple
- **NavLights:** 1
- **Collision Meshes:** 1
- **Engine Burns:** 1
- **Markers:** Multiple

---

## Comparison Table

| Feature | ter_pharos | ter_centaur | pebble_0 (Vanilla) |
|---------|------------|-------------|-------------------|
| **File Size** | 236,648 bytes | 232,860 bytes | 1,680,248 bytes |
| **POOL Size** | 229,984 bytes | 219,495 bytes | 1,680,248 bytes |
| **Texture Pool** | 183,988 comp / 524,256 decomp | 65,852 comp / 87,424 decomp | 1,673,248 comp / 2,097,120 decomp |
| **Mesh Pool** | 39,015 comp / 153,900 decomp | 125,480 comp / 1,218,456 decomp | 35,441 comp / 71,040 decomp |
| **Face Pool** | 6,953 comp / 10,896 decomp | 28,135 comp / 65,286 decomp | 1,334 comp / 2,220 decomp |
| **Textures** | 3 (DIFF, GLOW, TEAM) | 2 (DIFF, DIFF) | 3 (DIFF, GLOW, NORM) |
| **LODs** | 3 | 2 | 3 |
| **Joints** | 10 | Multiple | 1 |
| **NavLights** | 9 | 1 | 0 |
| **Engine Burns** | 0 | 1 | 0 |
| **Markers** | 0 | Multiple | 0 |
| **Collision** | Yes | Yes | Yes |
| **SCAR** | 3,063 bytes | 9,991 bytes | No |

---

## Key Observations

### 1. Compression Ratios Vary Significantly

**Texture Pool Compression:**
- ter_pharos: 183,988 / 524,256 = 0.35 (65% compression)
- ter_centaur: 65,852 / 87,424 = 0.75 (25% compression)
- pebble_0: 1,673,248 / 2,097,120 = 0.80 (20% compression)

**Mesh Pool Compression:**
- ter_pharos: 39,015 / 153,900 = 0.25 (75% compression)
- ter_centaur: 125,480 / 1,218,456 = 0.10 (90% compression)
- pebble_0: 35,441 / 71,040 = 0.50 (50% compression)

**Observation:** HODOR uses **model-specific compression settings**. There is no one-size-fits-all approach.

### 2. LMIP Chunk Sizes Vary

**ter_pharos:**
- LMIP: 79 bytes × 3 LODs

**ter_centaur:**
- LMIP: 74 bytes, 36 bytes (different sizes!)

**pebble_0:**
- LMIP: 87 bytes × 3 LODs

**Observation:** LMIP chunk size varies based on model complexity.

### 3. STAT Chunk Count Varies

**ter_pharos:**
- STAT: 92 bytes (1 chunk)

**ter_centaur:**
- STAT: 48 bytes, 46 bytes (2 chunks!)

**pebble_0:**
- STAT: 93 bytes (1 chunk)

**Observation:** Some models have multiple STAT chunks.

### 4. TAGS Chunk Presence

**ter_pharos:**
- MULT has FORM TAGS child chunk

**ter_centaur:**
- MULT has FORM TAGS child chunk

**pebble_0:**
- MULT does NOT have FORM TAGS child chunk

**Observation:** TAGS chunk is present in complex models but not in simple ones.

---

## Implications for Our Implementation

### 1. Model-Specific Compression

**Issue:** We cannot use fixed compression settings.

**Solution:** Analyze HODOR's compression algorithm and match it for each model type.

### 2. LMIP Chunk Variability

**Issue:** LMIP chunk size varies between models.

**Solution:** Calculate LMIP size based on model complexity.

### 3. STAT Chunk Count

**Issue:** Some models have multiple STAT chunks.

**Solution:** Handle multiple STAT chunks correctly.

### 4. TAGS Chunk Handling

**Issue:** TAGS chunk is optional.

**Solution:** Detect and preserve TAGS chunk when present.

---

## Test Case Value

### Why These Test Cases are Valuable

1. **Created by HODOR** - Actual tool that creates HOD 2.0 files
2. **Different complexity levels** - Simple (pebble_0) to complex (ter_pharos, ter_centaur)
3. **Different features** - NavLights, Engine Burns, Markers, SCAR
4. **Reference implementations** - Can compare our output with HODOR's output

### How to Use These Test Cases

1. **Extract DAE+TGA files** from each test case
2. **Generate HOD using our implementation**
3. **Compare with HODOR's output**
4. **Identify differences in compression and structure**
5. **Implement fixes to match HODOR's behavior**

---

## Next Steps

### Immediate

1. **Analyze HODOR's compression algorithm** - Identify settings for each model
2. **Analyze HODOR's binormal calculation** - Extract and compare
3. **Implement fixes** - Match HODOR's behavior

### Short Term

1. **Test with ter_pharos** - Generate HOD from DAE+TGA
2. **Test with ter_centaur** - Generate HOD from DAE+TGA
3. **Compare with HODOR output** - Verify our implementation
4. **Validate in game** - Ensure correct rendering

---

**Document Version:** 1.0  
**Last Updated:** 2026-05-28  
**Status:** Analysis Complete