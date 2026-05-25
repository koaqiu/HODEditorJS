---
name: hod-binary-layout
description: Use when reading, writing, or planning modifications to Homeworld Remastered HOD binary sub-chunks (NAVL, DOCK, BURN, GLOW, COLD, ETSH).
---

# HOD Binary Layouts Reference

This file serves as the definitive reference sheet for parsing and serializing the sub-chunks of the `"DTRM"` chunk form container in Homeworld Remastered HOD v2.0.

---

## 1. NAVL Chunk (NavLights)
NavLights are defined inside a single `"NAVL"` chunk containing a count and sequential blocks.

- **Structure**:
  - `count`: `u32` (LittleEndian)
  - Repeat `count` times:
    - `name`: String (Prepend length as `u32` + UTF-8 bytes)
    - `section`: `u32`
    - `size`: `f32`
    - `phase`: `f32`
    - `frequency`: `f32`
    - `style`: String (Prepend length as `u32` + UTF-8 bytes)
    - `color`: Vector3 (`x`, `y`, `z` as `f32`)
    - `_unused`: `f32` (always write `1.0f`)
    - `distance`: `f32`
    - `sprite_visible`: `u8` (0 or 1)
    - `high_end_only`: `u8` (0 or 1)

---

## 2. BURN Chunk (Engine Burns)
Each Engine Burn is stored as a separate `"BURN"` chunk.

- **Structure**:
  - `name`: String (Prepend length as `u32` + UTF-8 bytes)
  - `parent_name`: String (Prepend length as `u32` + UTF-8 bytes) (Maps to an EngineNozzle joint)
  - `num_divisions`: `i32` (usually `5`)
  - `num_flames`: `i32` (usually `1`)
  - `vertices`: Array of Vector3 (`x`, `y`, `z` as `f32`) of size `num_divisions * num_flames` (usually 5 * 1 * 12 = 60 bytes)

---

## 3. GLOW Chunk (Engine Glows)
Engine Glows are FORM chunks container holding two nested child chunks: `"INFO"` and `"BMSH"`.

- **FORM Parent**: `"GLOW"` (Form)
  - **Child Chunk 1**: `"INFO"` (Default)
    - `name`: String (Prepend length as `u32` + UTF-8 bytes)
    - `parent_name`: String (Prepend length as `u32` + UTF-8 bytes)
    - `lod`: `i32`
  - **Child Chunk 2**: `"BMSH"` (Normal)
    - A standard basic mesh chunk parsed using the same mesh parsing logic.

---

## 4. ETSH Chunk (Engine Shapes)
Engine Shapes represent the physical mesh volume of the engine exhaust.

- **Structure**:
  - `name`: String (Prepend length as `u32` + UTF-8 bytes)
  - `parent_name`: String (Prepend length as `u32` + UTF-8 bytes)
  - `mesh`: Standard basic mesh format.

---

## 5. COLD Chunk (Collision Meshes)
Collision Meshes represent the bounding boxes/spheres and simplified collision hulls.

- **Structure**:
  - `name`: String (Prepend length as `u32` + UTF-8 bytes) (Maps to the joint it is parented under)
  - `min_extents`: Vector3 (`x`, `y`, `z` as `f32`)
  - `max_extents`: Vector3 (`x`, `y`, `z` as `f32`)
  - `center`: Vector3 (`x`, `y`, `z` as `f32`)
  - `radius`: `f32`
  - `mesh`: Simplified collision mesh hulls.

---

## 6. DOCK Chunk (Dockpaths)
Dockpaths contain path markers for ship docking.

- **Structure**:
  - `count`: `u32`
  - Repeat `count` times:
    - `name`: String (Prepend length as `u32` + UTF-8 bytes)
    - `parent`: String (Prepend length as `u32` + UTF-8 bytes)
    - `num_points`: `i32`
    - Repeat `num_points` times:
      - `position`: Vector3 (`x`, `y`, `z` as `f32`)
      - `rotation`: Matrix4 (rotation matrix)
      - `tolerance`: `f32`
      - `max_speed`: `f32`
