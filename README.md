# Homeworld Remastered HOD Editor (`HODEditorJS`)

A modern, high-performance, cross-platform visual editor for Homeworld Remastered (HWRM) `.hod` ship skeleton and mesh files. Built with **Tauri**, **React**, **TypeScript**, **Three.js**, and a custom standalone **Rust parser (`hwr_hod_parser`)**.

This tool is designed to inspect, edit, and save `.hod` files, with a dedicated focus on migrating legacy **HOD 1.0** files to modern **HOD 2.0** formats, adding standard nodes, repairing skeleton structures, editing collision hulls, and managing ship animations.

---

## 📖 Table of Contents
1. [Key Features Overview](#-key-features-overview)
2. [Deep-Dive Feature Highlights](#-deep-dive-feature-highlights)
    - [HOD 1.0 / 2.0 Loading & 2.0 Saving](#1-hod-10--20-loading--20-saving)
    - [Animation Editing (WIP)](#2-animation-editing-work-in-progress---wip)
    - [Standard Node Templates](#3-standard-node-templates-hwrm-hod-20)
    - [TargetBox Editing](#4-targetbox-editing)
    - [Import / Export of .OBJ Models](#5-import--export-of-obj-model-files)
    - [Import / Export of Materials](#6-import--export-of-materials--textures)
    - [Model Auto-Fixing & Sanitization](#7-model-auto-fixing--sanitization)
3. [Architecture Overview](#%EF%B8%8F-architecture-overview)
4. [Installation & Run Guide](#-installation--run-guide)
5. [Development & Testing](#-development--testing)
6. [License](#-license)

---

## 🌟 Key Features Overview

* **Multi-Version HOD Support**: Full binary decompression, inspection, and visualization of both HOD 1.0 (legacy) and HOD 2.0 (remastered) formats.
* **Modern WebGL Viewport**: Powered by Three.js with zero-damping precise origin orbiting, adjustable grids, customizable axis helpers, and fully interactive 3D gizmos (`TransformControls`).
* **Attachment visualizers (DTRM chunks)**: Toggeable visual scene layers for NavLights (`NAVL`), Dockpaths (`DOCK`), Engine Burns (`BURN`), Engine Glows (`GLOW`), Engine Shapes (`ETSH`), and Collision hulls (`COLD`).
* **Self-Healing Skeleton Tool**: Automated repair of broken joint structures conforming to HWRM standards with a single click.
* **Cascading Renames**: Propagation of name modifications across child joints, markers, and meshes associated with point-groups to prevent broken game references.

---

## 🔍 Deep-Dive Feature Highlights

### 1. HOD 1.0 / 2.0 Loading & 2.0 Saving

Our parser delivers high-performance and robust binary decoding, overcoming several legacy engine boundaries:

* **HOD 1.0 Legacy Handling**:
  * **Two-Pass Sequential Parser**: HOD 1.0 ships (such as `shi_cain.hod`) write material data (`STAT` blocks) *before* their texture descriptors (`LMIP` blocks). This project implements a two-pass reader: Pass 1 pre-loads and caches all texture data, and Pass 2 parses material mapping blocks and maps them to resolved textures, preventing empty materials in the viewport.
  * **Recursive Mipmap Calculator**: Legacy files do not list sub-mipmap sizes in their headers, causing numeric overflow crashes in native parsers. This project automatically computes the dimension stack of all subsequent mipmap sublevels mathematically (halving dimensions iteratively) to render legacy texture hierarchies cleanly.
* **MS XPress LZ77 Compressor Bug Resolution**:
  Modders previously found that saving a HOD 2.0 file and launching it in-game caused corrupted meshes, texture tearing, or 90-degree offsets. We resolved three critical bugs in the compression algorithm inside `parser/src/xpress.rs`:
  1. **31-Bit Indicators Alignment**: Standard XPress bitstreams group indicator blocks every 32 bits, but the game engine parses them every 31 bits (setting `indicator_bit = 0` on the 31st iteration). The compressor has been corrected to group indicator blocks every 31 bits to avoid data-stream drift.
  2. **Offset Bit Shift Bug**: Fixed a byte-copy bug during 2-byte match serialization, applying a left shift of 6 bits (`((best_offset & 3) << 6) | 0b10`) instead of direct bitwise ORing, which had corrupted reference offsets.
  3. **Direct Distance Alignment**: Adjusted distance matching to copy from `output_idx - offset` rather than `output_idx - offset - 1`, matching Homeworld's custom decompressor variant and eliminating 1-byte accumulation offsets.
* **Absolute Bone Rotation & Scale Preservation**:
  Although the frontend maps 3D transforms to a 4x4 matrix, the Homeworld engine reads raw floating-point attributes (`px, py, pz, rx, ry, rz, sx, sy, sz` for joints, Euler `rx, ry, rz` for markers) directly from `HIER` and `MRKS` chunks. Since matrix-to-Euler decomposition is lossy and non-unique, converting to matrices and back caused the ship and its bones to rotate 90 degrees left or tilt 30 degrees in-game. 
  
  Our solution tracks these raw float vectors on load (`position`, `rotation`, and `scale`). On file save, unmodified joints/markers bypass lossy matrix decomposition, preserving original binary floats exactly and ensuring a **100% byte-for-byte identical roundtrip**.

---

### 2. Animation Editing (Work-in-Progress - WIP)

* **Mad Serialization & Decoding**: Supports decoding binary `.mad` companion animation files (Euler YXZ channels mapped to quaternion rotations, position offsets, and scale coordinates) and legacy HOD 1.0 embedded keyframe markers (`MRKR` -> `KEYF` -> `ANIM` IFF chunks).
* **Smooth Radial LERP/SLERP**: Translates between Three.js quaternions and Euler YXZ radians matching the Homeworld engine's coordinate rotation system. 
* **Timeline State Sync**: React playback loops and gizmo keyframe additions are bound to the parent model controller. Saving the HOD automatically triggers background commands to write or patch the companion `.mad` animation files.
* **In-Viewport Rendering (WIP Info)**:
  * While keyframes are exported and serialized properly in the file structure, the WebGL renderer handles animations as a Work-in-Progress. 
  * Geometry animation is propagated via **Mesh Delta Transforms** (`userData.parentJointName` and original baked world matrices). As joints animate via LERP/SLERP interpolation on the timeline, delta matrices are calculated on-the-fly to rotate and translate matching mesh components (e.g. rotating radar dishes, pivoting turrets) in real time.
  * A pulsing **Animation Status Badge** ("N Animations Loaded — Use Timeline Below ↓") guides users to the timeline controls at the bottom of the editor when an animated HOD is loaded.

---

### 3. Standard Node Templates (HWRM HOD 2.0)

Creating multi-joint setups is simplified using template presets. Instead of hand-building complex skeletal structures bone-by-bone, selecting a template in `HierarchyTree` adds nested parent-child families with mathematically accurate coordinate translation offsets of `+5.0` units:

| Template Type | Hierarchy & Offsets Structure |
| :--- | :--- |
| **Hardpoint** | `Hardpoint_name_Position` (Pivot) <br> ├── `_Direction` (offset `+5.0` on Local Y: `[0, 5, 0, 1]`) <br> └── `_Rest` (offset `+5.0` on Local Z: `[0, 0, 5, 1]`) |
| **Capture Point** | `CapturePoint#` (Base) <br> ├── `Heading` (offset `+5.0` on Local Z: `[0, 0, 5, 1]`) <br> ├── `Left` (offset `+5.0` on Local Z: `[0, 0, 5, 1]`) <br> └── `Up` (offset `+5.0` on Local Y: `[0, 5, 0, 1]`) |
| **Repair Point** | `RepairPoint#` (Base) <br> ├── `Heading` (offset `+5.0` on Local Z: `[0, 0, 5, 1]`) <br> ├── `Left` (offset `+5.0` on Local X: `[5, 0, 0, 1]`) <br> └── `Up` (offset `+5.0` on Local Y: `[0, 5, 0, 1]`) |
| **Salvage Point** | `SalvagePoint#` (Base) <br> ├── `Heading` (offset `+5.0` on Local Z: `[0, 0, 5, 1]`) <br> ├── `Left` (offset `+5.0` on Local Z: `[0, 0, 5, 1]`) <br> └── `Up` (offset `+5.0` on Local Y: `[0, 5, 0, 1]`) |
| **Weapon Turret** | `Weapon_name_Position` <br> ├── `_Direction` (offset `+5.0` on Local Y: `[0, 5, 0, 1]`) <br> ├── `_Rest` (offset `+5.0` on Local Z: `[0, 0, 5, 1]`) <br> └── `_Latitude` (offset `+5.0` on Local Z: `[0, 0, 5, 1]`) <br> &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;└── `_Barrel` (offset `+5.0` on Local Y) <br> &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;└── `_Muzzle` (offset `+5.0` on Local Y) |
| **Weapon Gimbal** | `Weapon_name_Position` <br> ├── `_Direction` (offset `+5.0` on Local Y: `[0, 5, 0, 1]`) <br> ├── `_Muzzle` (offset `+5.0` on Local Z: `[0, 0, 5, 1]`) <br> └── `_Rest` (offset `+5.0` on Local Z: `[0, 0, 5, 1]`) |

* **0-Indexed Naming Conversions**: Pre-fills suggested joint names utilizing 0-indexed counting structures (e.g. `NavLight0`, `marker0`, `EngineGlow0_LOD0`) with no underscores for hash suffix nodes, matching game engine rules.
* **Unified Group Folding**: Collapses Hardpoints, Weapons, and Capture Points into single folder nodes in the tree sidebar, shielding required structural components from accidental manual deletions.
* **Self-Healing Integrity Engine**: An integrated `getWarnings()` validator scans active groups in real-time. Missing child bones appear as red-dashed status cards in the Point-Group Inspector. Clicking **"Repair Structure"** instantly instantiates, parents, and aligns missing child joints back to default offsets.
* **Draggable Reparenting**: Full drag-and-drop hierarchy sorting in the sidebar. Reparenting items like NavLights updates the backing physical joint arrays seamlessly, updating world coordinate orientations.

---

### 4. TargetBox Editing

* **3D Collision Volume Visualizer**: Dedicated panel that generates an adjustable, wireframe 3D bounding box over the ship model.
* **Adjustable Dimensions & Margins**: Allows users to interactively resize width, height, length, and coordinate offsets.
* **One-Click Code Generator**: Instantly calculates and outputs the exact Lua formatting block corresponding to the TargetBox coordinates, ready for direct copy/paste into the ship's `.ship` configuration script.

---

### 5. Import / Export of .OBJ Model Files

* **LOD Mesh Injection**: Allows importing custom Wavefront `.OBJ` mesh files to replace or inject into specific Level of Detail (LOD) nodes in the ship mesh list.
* **Vertex Attribute Reconstruction**: Seamlessly handles position packing, normal re-generation, and texture UV coordinate mapping.
* **Exporter Pipeline**: Exports any selected LOD mesh from the model hierarchy as an `.OBJ` file, allowing creators to extract game models and tweak geometries in external tools like Blender or Maya.

---

### 6. Import / Export of Materials & Textures

* **Shader Settings Manager**: Full inspector tools to edit HOD 2.0 and HOD 1.0 materials, allowing custom shader assignment, rendering flags, and parameter edits.
* **TGA Diffuse Pipeline**: Supports referencing a primary diffuse texture (`_DIFF.TGA`), which loads and links automatically alongside corresponding specular, normal, or glow maps.
* **Standalone Material Portability**: Export material templates as standalone config files and import them onto other HOD models to achieve look-and-feel parity across entire fleets.

---

### 7. Model Auto-Fixing & Sanitization

To bypass annoying errors or manual hex-editing, the editor features built-in self-cleaning pipelines:

* **Astronomic Coordinate Outlier Sanitizer**: Skinned mesh vertex streams often contain dummy bone anchors with extreme coordinate values (uninitialized floats between $10^{11}$ and $10^{38}$ units). These outlier coordinate bounds blew up Three.js camera auto-focus algorithms, forcing the camera to zoom billions of units out and rendering the ship as an invisible, sub-pixel dot. On import, coordinates exceeding a magnitude of `10,000.0` units are safely sanitized to `0.0`.
* **Automatic Collision Hull Generation**: If a loaded HOD file lacks standard `COLD` sub-chunks, the editor calculates the exact geometry dimensions and automatically generates bounding box and bounding sphere collision bounds to prevent game crashes.
* **Circular Link Set Guards**: Scans parent relationships during tree parsing. If a circular parent-child joint reference is identified, parsing terminates recursively to prevent browser stack overflow crashes.

---

## 🛠️ Architecture Overview

The editor uses a decoupled high-performance architecture:

```
┌────────────────────────────────────────────────────────┐
│                     FRONTEND (UI)                      │
│            React + TypeScript + Tailwind CSS            │
│  - Timeline & Playback Controllers                     │
│  - Sidebar Joint Tree Visualizers                      │
├────────────────────────────────────────────────────────┤
│                     VIEWPORT (3D)                      │
│                       Three.js                         │
│  - Real-Time Pulse Animations (NavLights)              │
│  - Direct Transform Gizmos & Orbit Controls            │
├────────────────────────────────────▲───────────────────┤
│                                    │ IPC (Tauri)       │
│                                    ▼                   │
├────────────────────────────────────────────────────────┤
│                 TAURI DESKTOP BRIDGE                   │
│                       Rust API                         │
│  - Native File Dialogs                                 │
│  - Diagnostic Logger (hwr_hod_editor.log)              │
├────────────────────────────────────▲───────────────────┤
│                                    │ Rust FFI          │
│                                    ▼                   │
├────────────────────────────────────────────────────────┤
│                  STANDALONE PARSER                     │
│                hwr_hod_parser (Rust)                   │
│  - MS XPress 31-bit LZ77 Decompressor                  │
│  - HOD 1.0 & 2.0 IFF Binary Reader                     │
│  - Absolute Euler and Float Serializers                │
└────────────────────────────────────────────────────────┘
```

---

## 🚀 Installation & Run Guide

### Prerequisites
Make sure you have the following installed on your machine:
* [Node.js](https://nodejs.org/) (v18.x or newer)
* [Rust & Cargo](https://rustup.rs/) (v1.75.0 or newer)
* Platform-specific Tauri compile dependencies (visit [Tauri's Prerequisites Guide](https://v2.tauri.app/start/prerequisites/) for your OS)

### 1. Clone & Install Dependencies
Clone the repository and install Node.js modules from the project root:
```bash
git clone https://github.com/your-username/HODEditorJS.git
cd HODEditorJS
npm install
```

### 2. Run in Development Mode
Launches the interactive Tauri desktop application in hot-reloading development mode:
```bash
npm run tauri dev
```

### 3. Build Production Executable
Compiles the React frontend and packages a production-optimized native desktop executable (installer/app-bundle) for your operating system:
```bash
npm run build
npm run tauri build
```

---

## 🧪 Development & Testing

You can run isolated compilation checks and unit tests for the standalone Rust parser crate:

```bash
# Check parser syntax & dependencies
cargo check --manifest-path parser/Cargo.toml

# Run binary parser unit tests & LZ77 roundtrip verifications
cargo test --manifest-path parser/Cargo.toml
```

---

## 📄 License

This software is distributed under the **Creative Commons Attribution-NonCommercial 4.0 International (CC BY-NC 4.0)** license.

* **You are free to**: Share, copy, redistribute, adapt, remix, transform, and build upon the material.
* **Under the following terms**:
  * **Attribution**: You must give appropriate credit, provide a link to the license, and indicate if changes were made.
  * **Non-Commercial**: **You may NOT use the material for commercial purposes (the software cannot be sold).**

Read the full licensing terms in the accompanying `LICENSE` file.
