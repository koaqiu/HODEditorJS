# RODOH/HODOR DAE-to-HOD 2.0 Conversion Analysis

## Overview

RODOH (Reverse Of DAE to HOD) converts DAE + TGA files into HOD 2.0 format for Homeworld Remastered. This document analyzes the conversion process using ter_pharos as a case study.

## Input Files (ter_pharos)

### DAE File Structure
```
ter_pharos.DAE
├── <asset>
│   └── <contributor> authoring_tool="RODOH Apr 16 2015 17:44:57"
├── <library_images>
│   ├── IMG[Pharos_DIFF]-image (Pharos_DIFF.TGA)
│   ├── IMG[Pharos_TEAM]-image (Pharos_TEAM.TGA)
│   ├── IMG[Pharos_GLOW]-image (Pharos_GLOW.TGA)
│   ├── IMG[Pharos_SPEC]-image (Pharos_SPEC.TGA)
│   └── IMG[Pharos_STRP]-image (Pharos_STRP.TGA)
├── <library_materials>
│   └── MAT[pharos.bmp]_SHD[ship]
├── <library_effects>
│   └── MAT[pharos.bmp]_SHD[ship]-fx
├── <library_geometries>
│   ├── MULT[Root_mesh]_LOD[0] (306 positions, 303 normals, 249 UVs, 496 triangles)
│   ├── MULT[Root_mesh]_LOD[1] (139 positions, 146 normals, 121 UVs, 208 triangles)
│   ├── MULT[Root_mesh]_LOD[2] (42 positions, 61 normals, 42 UVs, 60 triangles)
│   └── COL[Root] (124 vertices, 60 triangles)
└── <library_visual_scenes>
    ├── ROOT_COL → COL[Root] (collision mesh)
    ├── ROOT_INFO
    │   ├── UVSets[1]
    │   └── Class[MultiMesh]_Version[512]
    └── ROOT_LOD[0]
        ├── NAVL[Navlight_B1] through NAVL[Navlight_R3] (9 navlights)
        └── MULT[Root_mesh]_LOD[0]
```

### Texture Files
```
Pharos_DIFF.TGA   → Diffuse texture (color)
Pharos_TEAM.TGA   → Team color mask
Pharos_GLOW.TGA   → Glow/emission texture
Pharos_SPEC.TGA   → Specular texture
Pharos_STRP.TGA   → Stripe mask
```

## Output HOD 2.0 File Structure

### Chunk Hierarchy
```
ter_pharos.hod (236648 bytes)
├── VERS (4 bytes) - Version: 512
├── NAME (26 bytes) - "Homeworld2 Multi Mesh File"
├── POOL (229984 bytes) - COMPRESSED DATA
│   ├── Texture Pool: compressed=183988, decompressed=524256 bytes
│   ├── Mesh Pool: compressed=39015, decompressed=153900 bytes
│   └── Face Pool: compressed=6953, decompressed=10896 bytes
├── HVMD (0 bytes) - Visual Data Container
│   ├── LMIP (79 bytes) - LOD 0
│   ├── LMIP (79 bytes) - LOD 1
│   ├── LMIP (79 bytes) - LOD 2
│   ├── STAT (92 bytes) - Statistics
│   └── MULT (174 bytes) - Mesh Container
├── DTRM (0 bytes) - Transform Data Container
│   ├── HIER (583 bytes) - Hierarchy
│   ├── NAVL (580 bytes) - Navigation Lights
│   ├── KDOP (1588 bytes) - Collision Tree
│   ├── COLD (8 bytes) - Collision Data
│   │   ├── BBOX (24 bytes) - Bounding Box
│   │   ├── BSPH (16 bytes) - Bounding Sphere
│   │   └── TRIS (8 bytes) - Collision Triangles
│   └── SCAR (3063 bytes) - Battle Scars
└── INFO (49 bytes) - File Information
```

## Key Conversion Differences

### 1. Mesh Data Storage

**DAE File (Uncompressed):**
```xml
<geometry id="MULT[Root_mesh]_LOD[0]-lib">
  <mesh>
    <source id="MULT[Root_mesh]_LOD[0]-POSITION">
      <float_array count="918">-4.678960 7.827110 -0.485433...</float_array>
    </source>
    <source id="MULT[Root_mesh]_LOD[0]-Normal0">
      <float_array count="909">-1.000000 0.000000 0.000000...</float_array>
    </source>
    <source id="MULT[Root_mesh]_LOD[0]-UV0">
      <float_array count="498">0.527230 -1.936170...</float_array>
    </source>
    <triangles count="496">
      <p>0 0 0 1 0 1 2 0 2...</p>
    </triangles>
  </mesh>
</geometry>
```

**HOD 2.0 (Compressed in POOL):**
- Vertex data is interleaved: [Position(3f), Normal(3f), UV(2f), Tangent(4f)]
- Stride: 64 bytes per vertex (26 floats × 4 bytes)
- All mesh data compressed using Microsoft Xpress algorithm
- Face indices stored separately in face pool

### 2. Material/Texture Handling

**DAE File:**
```xml
<image id="IMG[Pharos_DIFF]-image" name="IMG[Pharos_DIFF]_FMT[DXT1]">
  <init_from>Pharos_DIFF.TGA</init_from>
</image>
<material id="MAT[pharos.bmp]_SHD[ship]">
  <instance_effect url="#MAT[pharos.bmp]_SHD[ship]-fx" />
</material>
```

**HOD 2.0:**
- Textures compressed to DXT1/DXT5 format
- Stored in texture pool within POOL chunk
- Material reference: "pharos.bmp" with shader "ship"
- Texture slots: DIFF, GLOW, TEAM (3 textures)

### 3. LOD System

**DAE File:**
```
ROOT_LOD[0] → MULT[Root_mesh]_LOD[0] (496 triangles, 1488 indices)
ROOT_LOD[1] → MULT[Root_mesh]_LOD[1] (208 triangles, 624 indices)
ROOT_LOD[2] → MULT[Root_mesh]_LOD[2] (60 triangles, 180 indices)
```

**HOD 2.0:**
- LMIP chunks define LOD levels
- BMSH chunks contain mesh data for each LOD
- HOD mesh parts store interleaved vertices plus separate face indices
- The stable HODOR structural check currently verifies per-material material index, index count, and vertex count

### 4. Navigation Lights

**DAE File:**
```xml
<node name="NAVL[Navlight_B1]" id="NAVL[Navlight_B1]">
  <translate sid="translate">0.000000 3.290000 -3.480000</translate>
  <node name="SUB_PARAMS" id="SUB_PARAMS">
    <node name="Sz[1]" id="Sz[1]" />
    <node name="Col[0,0,1]" id="Col[0,0,1]" />
  </node>
</node>
```

**HOD 2.0:**
- NAVL chunk (580 bytes) stores all 9 navlights
- Position, size, color, phase, frequency encoded in binary

### 5. Collision Data

**DAE File:**
```xml
<geometry id="COL[Root]-lib">
  <mesh>
    <source id="COL[Root]-POSITION">
      <float_array count="372">-0.386489 3.274360 -3.795890...</float_array>
    </source>
    <triangles count="60">0 1 2...</triangles>
  </mesh>
</geometry>
```

**HOD 2.0:**
- COLD chunk with BBOX, BSPH, TRIS sub-chunks
- KDOP chunk for collision tree (1588 bytes)
- Much more complex collision structure

## Conversion Process

### Step 1: DAE Parsing
1. Parse COLLADA XML structure
2. Extract mesh geometry (vertices, normals, UVs)
3. Extract material definitions
4. Extract texture references
5. Extract node hierarchy (joints, navlights, etc.)

### Step 2: Texture Processing
1. Read TGA files referenced in DAE
2. Convert to DXT1/DXT5 format based on SHADERS.MAP
3. Apply shader channel mapping:
   - DIFF = R G B 1 (diffuse texture)
   - GLOW = G G G G (glow map from green channel)
   - TEAM = 1 1 1 r (team mask from red channel)
4. Compress textures using DXT compression

### Step 3: Mesh Processing
1. Interleave vertex data: Position + Normal + UV + Tangent
2. Convert to binary format (little-endian)
3. Calculate tangent/bitangent for normal mapping
4. Organize into LOD levels
5. Compress mesh data using Microsoft Xpress

### Step 4: Face Processing
1. Extract triangle indices from DAE
2. Convert to 16-bit indices
3. Compress face data using Microsoft Xpress

### Step 5: POOL Chunk Creation
1. Create texture pool (compressed textures)
2. Create mesh pool (compressed vertex data)
3. Create face pool (compressed indices)
4. Combine into single POOL chunk with headers

### Step 6: HOD 2.0 Assembly
1. Write VERS (version 512)
2. Write NAME ("Homeworld2 Multi Mesh File")
3. Write POOL chunk
4. Create HVMD container:
   - LMIP chunks for LODs
   - STAT chunk (statistics)
   - MULT chunk (mesh references)
5. Create DTRM container:
   - HIER chunk (hierarchy)
   - NAVL chunk (navigation lights)
   - KDOP/COLD chunks (collision)
   - SCAR chunk (battle scars)
6. Write INFO chunk

## Key Technical Details

### POOL Chunk Structure
```
POOL Header
├── Texture Pool
│   ├── Compressed size: 183988 bytes
│   ├── Decompressed size: 524256 bytes
│   └── DXT-compressed texture data
├── Mesh Pool
│   ├── Compressed size: 39015 bytes
│   ├── Decompressed size: 153900 bytes
│   └── Interleaved vertex data
└── Face Pool
    ├── Compressed size: 6953 bytes
    ├── Decompressed size: 10896 bytes
    └── 16-bit triangle indices
```

### Vertex Format (64 bytes per vertex)
```
Offset  Size  Description
0       12    Position (X, Y, Z) - float32 × 3
12      12    Normal (X, Y, Z) - float32 × 3
24      8     UV (U, V) - float32 × 2
32      16    Tangent (X, Y, Z, W) - float32 × 4
48      16    Bitangent (X, Y, Z, W) - float32 × 4
```

### Compression Algorithm
- **Microsoft Xpress**: LZ77-based compression
- **Block size**: 256 bytes
- **Compression ratio**: ~4:1 for textures, ~4:1 for meshes

## Validation

### Mesh Count Verification
- DAE: 3 mesh LODs (Root_mesh LOD0, LOD1, LOD2)
- HOD: 3 BMSH chunks in MULT container
- ✓ Match

### Geometry Count Verification
- DAE LOD0: 496 triangles, 1488 indices, 1 material part
- DAE LOD1: 208 triangles, 624 indices, 1 material part
- DAE LOD2: 60 triangles, 180 indices, 1 material part
- ✓ Match for material grouping and index counts in the current source-asset replication test

### Vertex Joining Status
- DAE unique position/UV/normal tuple counts are lower than the current direct OBJ-built vertex counts.
- HODOR BMSH vertex counts match the current OBJ-built vertex counts for the checked fixtures.
- Do not treat DAE unique tuple counts as the HOD vertex-buffer target; HODOR expands or preserves more face-corner tuples during DAE-to-HOD conversion.

### Texture Count Verification
- DAE: 5 textures (DIFF, TEAM, GLOW, SPEC, STRP)
- HOD: 3 textures (DIFF, GLOW, TEAM)
- ✓ Match (SPEC and STRP not used in ship shader)

## Conclusion

RODOH/HODOR performs a complex conversion from DAE+TGA to HOD 2.0:
1. **Mesh data** is interleaved, converted to binary, and compressed
2. **Textures** are converted to DXT format and compressed
3. **Structure** is reorganized from COLLADA XML to IFF-like binary chunks
4. **Compression** uses Microsoft Xpress algorithm for ~4:1 ratio
5. **LOD system** is preserved with LMIP chunks
6. **Materials** are mapped to Homeworld shaders via SHADERS.MAP

The resulting HOD 2.0 file is ~50% smaller than the original DAE+TGA files due to compression.
