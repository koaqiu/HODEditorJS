# POOL Chunk Specification - HOD 2.0

## Overview

The POOL chunk is the core data container in HOD 2.0 files, storing compressed textures, meshes, and face indices. This document provides a complete specification of the POOL chunk structure.

---

## POOL Chunk Structure

### Header

```
Offset  Size  Description
0       4     Pool Type (u32, Little-Endian)
              Value: 3518 (0x0DB6)
```

### Stream Layout

The POOL chunk contains three sequential streams:

```
┌─────────────────────────────────────────────────────────────┐
│ Pool Type (4 bytes)                                          │
├─────────────────────────────────────────────────────────────┤
│ Texture Stream                                               │
│   ├── Compressed Size (u32, Little-Endian)                  │
│   ├── Decompressed Size (u32, Little-Endian)                │
│   └── Compressed Data (bytes)                               │
├─────────────────────────────────────────────────────────────┤
│ Mesh Stream                                                   │
│   ├── Compressed Size (u32, Little-Endian)                  │
│   ├── Decompressed Size (u32, Little-Endian)                │
│   └── Compressed Data (bytes)                               │
├─────────────────────────────────────────────────────────────┤
│ Face Stream                                                   │
│   ├── Compressed Size (u32, Little-Endian)                  │
│   ├── Decompressed Size (u32, Little-Endian)                │
│   └── Compressed Data (bytes)                               │
└─────────────────────────────────────────────────────────────┘
```

### Total Header Size

- Pool Type: 4 bytes
- Texture Stream Header: 8 bytes (compressed_size + decompressed_size)
- Mesh Stream Header: 8 bytes (compressed_size + decompressed_size)
- Face Stream Header: 8 bytes (compressed_size + decompressed_size)
- **Total: 28 bytes**

---

## Stream Details

### 1. Texture Stream

**Purpose:** Stores DXT-compressed textures  
**Compression:** Microsoft Xpress LZ77  
**Typical Ratio:** ~4:1

**Structure:**
```
Texture Stream
├── compressed_size: u32 (Little-Endian)
├── decompressed_size: u32 (Little-Endian)
└── compressed_data: bytes[compressed_size]
```

**Data Format:**
- DXT1: 4 bytes per 4x4 block (no alpha)
- DXT5: 8 bytes per 4x4 block (with alpha)
- Multiple textures are concatenated

**Example (pebble_0):**
```
compressed_size: 1,673,248 bytes (vanilla)
decompressed_size: 2,097,120 bytes
compression_ratio: 0.80 (80% of original)
```

### 2. Mesh Stream

**Purpose:** Stores interleaved vertex data  
**Compression:** Microsoft Xpress LZ77  
**Typical Ratio:** ~2:1

**Structure:**
```
Mesh Stream
├── compressed_size: u32 (Little-Endian)
├── decompressed_size: u32 (Little-Endian)
└── compressed_data: bytes[compressed_size]
```

**Data Format:**
- Vertex format: Position (12) + Normal (12) + UV (8) + Tangent (16) + Bitangent (16)
- Total stride: 64 bytes per vertex
- Multiple LOD levels are concatenated

**Example (pebble_0):**
```
compressed_size: 35,441 bytes (vanilla)
decompressed_size: 71,040 bytes
compression_ratio: 0.50 (50% of original)
```

### 3. Face Stream

**Purpose:** Stores triangle indices  
**Compression:** Microsoft Xpress LZ77  
**Typical Ratio:** ~1.5:1

**Structure:**
```
Face Stream
├── compressed_size: u32 (Little-Endian)
├── decompressed_size: u32 (Little-Endian)
└── compressed_data: bytes[compressed_size]
```

**Data Format:**
- 16-bit indices (u16, Little-Endian)
- Sequential triangle lists
- Multiple LOD levels are concatenated

**Example (pebble_0):**
```
compressed_size: 1,334 bytes (vanilla)
decompressed_size: 2,220 bytes
compression_ratio: 0.60 (60% of original)
```

---

## Compression Algorithm

### Microsoft Xpress LZ77

**Type:** LZ77-based compression  
**Block Size:** 256 bytes (typical)  
**Window Size:** 65536 bytes (maximum)

**Key Features:**
- Literal bytes (uncompressed)
- Match copies (backreferences)
- Variable-length encoding

**Implementation:** `parser/src/xpress.rs`

### Compression Settings

**Current Implementation:**
- Hash chain length: 64 (maximum matches to check)
- Minimum match length: 3 bytes
- Maximum match length: 258 bytes
- Maximum offset: 65535 bytes

**Comparison with RODOH:**
- Different compression patterns (see testing_diff results)
- May use different hash chain settings
- May use different compression levels

---

## Pool Type Identifier

### Value: 3518 (0x0DB6)

**Location:** First 4 bytes of POOL chunk  
**Format:** u32, Little-Endian  
**Importance:** Must be set correctly for game engine to parse

**Verification:**
```rust
let pool_type = u32::from_le_bytes([0x06, 0x0D, 0x00, 0x00]); // 0x0DB6 = 3518
```

---

## Data Alignment

### Texture Stream
- No specific alignment required
- DXT blocks are naturally aligned (16 bytes)

### Mesh Stream
- Vertices are naturally aligned (64 bytes each)
- No padding between vertices

### Face Stream
- 2-byte alignment required (u16 indices)
- Padded with 0 if odd length

---

## Compression Statistics

### pebble_0 (Vanilla)

| Stream | Compressed | Decompressed | Ratio | Xpress Stats |
|--------|------------|--------------|-------|--------------|
| Texture | 1,673,248 | 2,097,120 | 0.80 | inds=32299, lits=704301, matches=296968 |
| Mesh | 35,441 | 71,040 | 0.50 | inds=890, lits=25523, matches=2046 |
| Face | 1,334 | 2,220 | 0.60 | inds=38, lits=1158, matches=7 |

### pebble_0 (Roundtrip)

| Stream | Compressed | Decompressed | Ratio | Xpress Stats |
|--------|------------|--------------|-------|--------------|
| Texture | 1,122,994 | 2,097,120 | 0.54 | inds=15456, lits=131478, matches=347650 |
| Mesh | 35,739 | 71,040 | 0.50 | inds=881, lits=24851, matches=2456 |
| Face | 1,322 | 2,220 | 0.60 | inds=38, lits=1146, matches=7 |

### pebble_0 (From Assets)

| Stream | Compressed | Decompressed | Ratio | Xpress Stats |
|--------|------------|--------------|-------|--------------|
| Texture | 1,124,432 | 2,097,120 | 0.54 | inds=15503, lits=133322, matches=347249 |
| Mesh | 18,156 | 71,040 | 0.26 | inds=371, lits=9088, matches=2390 |
| Face | 1,322 | 2,220 | 0.60 | inds=38, lits=1146, matches=7 |

---

## Key Observations

### 1. Texture Compression Variance

**Observation:** Vanilla texture compression is 33% larger than generated  
**Hypothesis:** RODOH uses different DXT compression quality settings  
**Impact:** Affects file size but not visual quality

### 2. Mesh Compression Variance

**Observation:** From-assets mesh compression is 44-49% smaller than vanilla  
**Hypothesis:** Different vertex ordering or padding  
**Impact:** May affect rendering performance

### 3. Face Compression Consistency

**Observation:** Face compression is nearly identical across all versions  
**Analysis:** Face indices are simple sequential data, compression is consistent  
**Impact:** No issues expected

### 4. Xpress Compression Patterns

**Observation:** Significant differences in Xpress statistics  
**Analysis:** Different hash chain settings or compression levels  
**Impact:** Affects file size but not functionality

---

## Implementation Notes

### Creating POOL Chunk

```rust
fn generate_pool_data(compiled_meshes: &[CompiledMesh], comp_tex: &[u8], 
                      decomp_tex_len: u32, pool_type: u32) -> Vec<u8> {
    let mut pool_buf = Vec::new();
    let mut cursor = Cursor::new(&mut pool_buf);
    
    // Pool type identifier
    cursor.write_u32::<LittleEndian>(pool_type)?; // 3518 = 0x0DB6
    
    // Texture stream
    cursor.write_u32::<LittleEndian>(comp_tex.len() as u32)?;
    cursor.write_u32::<LittleEndian>(decomp_tex_len)?;
    cursor.write_all(comp_tex)?;
    
    // Mesh stream
    let comp_mesh = xpress::compress_or_raw(&decomp_mesh);
    cursor.write_u32::<LittleEndian>(comp_mesh.len() as u32)?;
    cursor.write_u32::<LittleEndian>(decomp_mesh.len() as u32)?;
    cursor.write_all(&comp_mesh)?;
    
    // Face stream
    let comp_face = xpress::compress_or_raw(&decomp_face);
    cursor.write_u32::<LittleEndian>(comp_face.len() as u32)?;
    cursor.write_u32::<LittleEndian>(decomp_face.len() as u32)?;
    cursor.write_all(&comp_face)?;
    
    Ok(pool_buf)
}
```

### Reading POOL Chunk

```rust
fn parse_pool_chunk(chunk: &IffChunk) -> Result<PoolData, String> {
    let mut cursor = Cursor::new(&chunk.data);
    
    let pool_type = cursor.read_u32::<LittleEndian>()?;
    
    // Texture stream
    let comp_tex_len = cursor.read_u32::<LittleEndian>()? as usize;
    let decomp_tex_len = cursor.read_u32::<LittleEndian>()? as usize;
    let mut comp_tex = vec![0u8; comp_tex_len];
    cursor.read_exact(&mut comp_tex)?;
    
    // Mesh stream
    let comp_mesh_len = cursor.read_u32::<LittleEndian>()? as usize;
    let decomp_mesh_len = cursor.read_u32::<LittleEndian>()? as usize;
    let mut comp_mesh = vec![0u8; comp_mesh_len];
    cursor.read_exact(&mut comp_mesh)?;
    
    // Face stream
    let comp_face_len = cursor.read_u32::<LittleEndian>()? as usize;
    let decomp_face_len = cursor.read_u32::<LittleEndian>()? as usize;
    let mut comp_face = vec![0u8; comp_face_len];
    cursor.read_exact(&mut comp_face)?;
    
    Ok(PoolData { pool_type, comp_tex, decomp_tex_len, ... })
}
```

---

## Validation Checklist

When creating or modifying POOL chunks:

- [ ] Pool type is 3518 (0x0DB6)
- [ ] All stream headers are present (8 bytes each)
- [ ] Compressed sizes match actual data lengths
- [ ] Decompressed sizes are correct
- [ ] Xpress compression is valid (decompressable)
- [ ] Face stream is 2-byte aligned
- [ ] Total size matches expected size

---

## References

### Source Code
- `parser/src/xpress.rs` - Xpress compression implementation
- `parser/src/compiler.rs` - `generate_pool_data` function
- `parser/src/hod.rs` - POOL chunk parsing

### Test Data
- `testing/pebble_0/pebble_0_vanilla.hod` - Reference POOL chunk
- `testing/pebble_0/pebble_0_roundtrip.hod` - Generated POOL chunk

### Tools
- `cargo run --bin dump_pool` - Dump POOL stream sizes
- `cargo run --bin testing_diff` - Compare POOL chunks

---

**Document Version:** 1.0  
**Last Updated:** 2026-05-27  
**Status:** Complete