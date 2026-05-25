# HODEditorJS - Agent Starting Point & Handover Guide

Welcome to the **Homeworld Remastered HOD Editor** development workspace. This file serves as the definitive starting point, design architecture reference, and technical roadmap for any incoming AI agent or developer.

---

## 1. Project Mission & Core Purpose
This tool is a modern, cross-platform desktop application built with **Tauri**, **React**, **TypeScript**, and a custom high-performance **Rust parser (`hwr_hod_parser`)**. It is designed to inspect, edit, and save Homeworld Remastered (HWRM) `.hod` ship mesh and skeleton files—supporting both legacy **HOD 1.0** and modern **HOD 2.0** file formats, with a focus on migrating legacy files to modern formats.

---

## 2. Directory Architecture & External Reference Assets
* **/parser/**: A standalone, pure Rust library crate (`hwr_hod_parser`) that parses binary `.hod` files.
  * `src/hod.rs`: Implements visual hierarchy parsers, POOL chunk decompression, mesh attributes, markers, and joints hierarchy structure.
  * `src/xpress.rs`: Decompresses MS-XCA compressed chunk streams utilizing a custom LZ77-variant layout.
* **/src-tauri/**: The Rust Tauri desktop bridge.
  * `src/lib.rs`: Manages native OS file dialogs, IPC bridges for file loading/saving, and a persistent diagnostic logger writing to `hwr_hod_editor.log`.
* **/src/**: The frontend React + TypeScript + Three.js + Tailwind UI web application.
  * `src/App.tsx`: Handles file load states, the legacy-to-modern migration warning banners, and primary view state.
  * `src/components/Viewport.tsx`: Implements the interactive WebGL Canvas using Three.js and camera orbit controls.
  * `src/components/HierarchyTree.tsx`: Renders the visual hierarchy sidebar tree (joints, meshes, and LODs) with interactive real-time visibility toggles.

### 📚 External Reference Tools
When designing or debugging the editor UI features, refer to the following native UI/editor implementations in the mod tools workspace:
* **CFHodEd**: `/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/mod-tools/CFHodEd/`
  * *Purpose*: The gold-standard WinForms/C# visualizer and editor for HOD 2.0. Excellent for researching layout of NavLights, Engine Glows, Dockpaths, and LOD mesh assignments.
* **DAEnerys**: `/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/mod-tools/DAEnerys/`
  * *Purpose*: A `.DAE` (Collada) import/export pipeline tool for HWRM asset authoring, useful for understanding bone weight skinning layout.

### 📁 Sample `.hod` Files for Development & Testing
Use these specific files when loading, validating, or running parser tests:
* **HOD 1.0 (Legacy format)**:
  * `/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/uncompressed_bigs/freespace_remastered/ship/ter_orion/ter_orion.hod`
* **HOD 2.0 (Modern HWRM format - Mothership)**:
  * `/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/uncompressed_bigs/HWRM/ship/hgn_mothership/hgn_mothership.hod`
* **HOD 2.0 (Modern HWRM format - Orion)**:
  * `/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld/HWRM_FSFC/source/ship/ter_orion/ter_orion.hod`

---

## 3. Breakthrough Discoveries & Critical Fixes

### 🌟 HOD 2.0 Joint Identifier Mask (`(first_val & 0xFFFFFF00) == 0xFFFFFF00`)
* **The Issue**: Legacy code checked HOD 2.0 files with a hardcoded value: `if first_val == 0xFFFFFF2C`. However, official HOD 2.0 files utilize various joint formats starting with other signatures (e.g. `0xFFFFFF40` on the Hgn Mothership). This caused non-matching files to fall back to a HOD 1.0 joint-count loop, reading garbage signatures as billion-iteration joint loops, immediately crashing with an EOF error.
* **The Fix**: Changed the validation to a bitwise mask: `if (first_val & 0xFFFFFF00) == 0xFFFFFF00`. This reliably identifies all variations of HOD 2.0 joint chunk headers.

### 🛑 Minimum Stream-Length Guards in `parse_joints`
* **The Issue**: Multi-LOD models and official assets have trailing, unaligned padding bytes at the end of hierarchy (`HIER`) blocks. This caused naive EOF checks to parse garbage data.
* **The Fix**: Added a check requiring `remaining < 44` bytes inside `parse_joints` to safely terminate the loop since a valid joint block requires at least 44 bytes.

### 📐 12-byte Positions/Normals & Skinning Space Offset
* **The Issue**: Vertex positions and normals in Homeworld skinned meshes are packed as 3-float vectors (12 bytes) rather than 4-float vectors (16 bytes). Reading them incorrectly created a severe 4-byte offset drift, corrupting adjacent attributes (such as bone indices, weights, and UV coordinates).
* **The Fix**: Updated `read_vertex` to parse positions and normals as strictly 12 bytes and handle trailing skinning parameters dynamically.

### 🛡️ Circular Reference Guards in Hierarchies
* **The Issue**: Corrupted or modified HOD files can contain circular parent-child joint loops, causing the recursive Three.js coordinate solvers and React tree renderers to overflow the stack and freeze the JS runtime.
* **The Fix**: Integrated a `visited: Set<string>` tracker inside both `Viewport.tsx` (for global joint matrix calculations) and `HierarchyTree.tsx` (for tree rendering) to stop recursive traversal immediately upon detecting a circular link.

### ⚡ Safe Linear Index Spreading
* **The Issue**: Using `Math.max(...part.indices)` on large, high-poly ship meshes crashed the JavaScript virtual machine with a stack-overflow exception.
* **The Fix**: Replaced the spread operator with a safe, simple linear iteration loop to find the maximum vertex index bounds.

### 🎥 Precise Origin Orbiting & Zero Panning Smoothness (Momentum Disabled)
* **The Issue**: Camera orbit controls previously rotated around the model's computed bounding-box center with high momentum/damping. This felt slippery to modders and made precise alignment to grid markings challenging.
* **The Fix**: Disabled camera orbit damping (`enableDamping = false`) and fixed the camera rotation focus exactly to the grid origin `(0, 0, 0)`, ensuring immediate snap-stops on drag end and seamless spatial rotation alignment.

### 🔌 Custom XPress LZ77 Decompression Match Offset Fix
* **The Issue**: Standard MS-XCA (XPress) decompressors copy sliding-window matches from `output_idx - offset - 1`. However, Homeworld's custom offset variant copies from exactly `output_idx - offset` (1 byte difference). Using standard offsets introduced a cumulative 1-byte byte drift, corrupting all vertex coordinates starting at Vertex 1 or 2, collapsing them to `(0,0,0)` on the frontend, and rendering empty space.
* **The Fix**: Updated `parser/src/xpress.rs` to copy matches using `output_idx - offset` (`checked_sub(offset)`). This completely eliminates alignment drift, parsing and rendering all high-poly assets and pebbles flawlessly.

### 💡 DTRM Sub-chunk Expansion & Real-Time Animators (Phase 1, 2, 3, 5 Complete)
* **The Issue**: Advanced ship attachments—such as NavLights (`NAVL`), Dockpaths (`DOCK`), Engine Burns (`BURN`), Engine Glows (`GLOW`), Engine Shapes (`ETSH`), and Collision hulls (`COLD`)—were previously ignored by the parser and invisible in the 3D viewport.
* **The Fix**: 
  - **Rust Backend**: Extended `parser/src/hod.rs` and `parser/src/iff.rs` to decode and serialize all 6 sub-chunks natively. Designed a self-detecting boundary scanner for `COLD` container padding to maintain 100% classic HOD v1 and remastered HOD v2 cross-version parsing.
  - **Real-Time Pulsing NavLights**: Programmed a time-delta animation loop in `src/components/Viewport.tsx` that pulses NavLight helpers based on their exact frequency and phase, complete with an interactive HTML5 RGB color picker.
  - **Toggeable Visualizers**: Integrated checkboxes in the toolbar to dynamically toggle NavLights, Dockpaths, Engine Burns, and Collision meshes in the WebGL scene.
  - **Direct 3D Gizmos**: Connected `TransformControls` to NavLights, Markers, and individual Dockpath points for drag-moving in 3D, mapping coordinate changes back to local joint matrices.
  - **Collision Box Auto-Creator**: Added automatic bounding box/sphere generation on load for HOD files missing collision hulls.
  - **Big-Data Settings Folder Picker**: Built a local-storage persisted config option pointing to uncompressed Big folder `keeper.txt` roots.

---

## 4. Application Log Files & Diagnostic Tracking
To prevent silent failures, any critical operations, user commands, frontend exceptions, and backend panics are persisted inside `hwr_hod_editor.log`.

* **Dynamic OS Locations**:
  * **Development Log Location**: `/run/host/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/src-tauri/target/debug/hwr_hod_editor.log`
* **Diagnostic Subsystems Captured**:
  * **Native Panic Hooks**: Any unhandled Rust thread panic captures file, line number, and error payload before cleanly logging.
  * **Frontend IPC Bridge**: Connects runtime Three.js render exceptions, UI warnings, and file loading actions via Tauri's custom `log_event` hook.
  * **HOD Metadata Dumps**: Records parsed node totals, mesh counts, and decompressed buffer metrics on every file import.

---

## 5. Current Build & Run Commands

Ensure you run these from the project root directory:

### Run Tauri Desktop Application
```bash
npm run tauri dev
```

### Run Web Frontend Build
```bash
npm run build
```

### Compile & Test Rust Parser Standalone
```bash
cargo check --manifest-path parser/Cargo.toml
cargo test --manifest-path parser/Cargo.toml
```

---

## 6. Immediate Next Steps & Roadmaps (From TODO)

When resuming development, focus on completing the final stages of the pipeline:

1. **Companion Logic Editor (`.events`, `.mad`, `.madstate`)**:
   - Parse companion `.events` Lua files to map marker visual triggers (sounds, particle effects) directly in the viewport.
   - Design a dedicated Lua text editor tab with syntax highlighting to edit events side-by-side with the 3D canvas.
2. **OBJ Import Wizard & Texture Compressors**:
   - Provide UI buttons to import custom `.OBJ` mesh files as ship LODs.
   - Automatically compress localized `_DIFF.TGA` textures to DXT format and package them inside the binary output `POOL` chunk on save.
3. **Animations Track Previewer (`KEYF`)**:
   - Render and animate multi-key skeletal joints and keyframe tracks directly inside the WebGL viewport by integrating `.mad` state definition controllers.
4. **Collision Hull Mesh Handlers**:
   - Support importing/exporting simplified collision geometries.
5. **Errors & Warnings Linter Panel**:
   - Highlight missing Root joints, orphan bones, and invalid weapon/nozzle naming prefixes.

---

## 7. Recent Advanced Implementations

### 📐 Multi-Joint Assembly Templates
- **Parent-Child Skeletal Linkage**: When adding special group nodes like Hardpoints, Capture Points, Repair Points, Salvage Points, Weapon Gimbals, or Weapon Turrets, the editor automatically instantiates and nests their entire multi-joint component structures rather than just a single joint.
- **Primary Axis Translation Offsets**: To guarantee directionality, nested children of assembly templates are positioned at `+5.0` local coordinate translation offsets along standard primary axes (e.g. `+5.0y` for Directions, `+5.0z` for Muzzles and Rests).

### 🏷️ 0-Indexed Hash Naming Pre-Fillers
- Modals pre-fill naming suggestions using modern 0-indexed integer counting with no underscores for hash suffix nodes, automatically pre-filling strings like `NavLight0` or `marker0` to perfectly align with Homeworld Remastered requirements.

### 📁 Collapsible Sidebar Groups & Integrity Warnings
- **Visual Group Folders**: Custom point-groups render as collapsible unified folder nodes in the skeletal hierarchy list to protect their child bones from accidental edits.
- **getWarnings() Integrity Solver**: Checks the structural correctness of weapon and attachment groups on the fly. Missing or misaligned child joint warnings are surfaced in real-time.

### 🩹 Self-Healing Repair Tool & Mass-Renaming
- **One-Click Group Restorer**: Selecting an incomplete point-group inside the inspector displays missing bones in dashed-red cards with a single-click "Repair Structure" button. Clicking this automatically generates, parents, and snaps the missing bones back to defaults.
- **Cascade Group Renaming**: Renaming any point-group root cascades the name change automatically to all child joints, markers, and meshes associated with that group.

### 🕹️ Drag-and-Drop NavLight Reparenting
- NavLights in both the tree hierarchy and root lists are fully draggable and droppable. Dropping a NavLight on a joint node automatically updates the parent of its corresponding backing joint in the HOD mesh.

