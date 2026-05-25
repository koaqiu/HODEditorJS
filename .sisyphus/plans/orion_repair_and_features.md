# WORK PLAN: HWRM 2.0 HOD Editor Enhancements and Serialization Fixes

## 1. Draggable NavLights and Weapon Assemblies
- **Files to Modify**: `src/components/Viewport.tsx`, `src/App.tsx`
- **Detailed Changes**:
  - In `src/components/Viewport.tsx` (inside `getLocalPosition` function):
    - Replace the `selectedType === "navlight"` block with:
      ```typescript
      } else if (selectedType === "navlight") {
        const joint = model?.joints.find(j => j.name.toLowerCase() === selectedName.toLowerCase());
        parentName = joint?.parent_name || "";
      ```
    - Add a `selectedType === "weapon_group"` block:
      ```typescript
      } else if (selectedType === "weapon_group") {
        const positionJointName = `${selectedName}_Position`;
        const joint = model?.joints.find(j => j.name.toLowerCase() === positionJointName.toLowerCase());
        parentName = joint?.parent_name || "";
      ```
  - In `src/App.tsx` (inside `handleNodeTransform` function):
    - Add a `type === "weapon_group"` block:
      ```typescript
      } else if (type === "weapon_group") {
        const positionJointName = `${name}_Position`;
        const updatedJoints = model.joints.map((joint) => {
          if (joint.name.toLowerCase() === positionJointName.toLowerCase()) {
            const m = joint.local_transform.m.map(row => [...row]);
            m[3][0] = pos.x;
            m[3][1] = pos.y;
            m[3][2] = pos.z;
            return { ...joint, local_transform: { m } };
          }
          return joint;
        });
        updateModel({ ...model, joints: updatedJoints });
      }
      ```

## 2. Dynamic Node Templates (Repair Point, Capture Point, Hardpoint, Salvage Point, Weapon Gimbals, Weapon Turrets)
- **Files to Modify**: `src/components/HierarchyTree.tsx`
- **Detailed Changes**:
  - Update `addNodeType` state types list to include `"repair_point_template"`, `"capture_point_template"`, `"hardpoint_template"`, `"salvage_point_template"`.
  - In the `useEffect` node naming generator, add default prefix pre-filling with unique numbering counts starting at index 0:
    - `"repair_point_template"` -> `RepairPoint${count}`
    - `"capture_point_template"` -> `CapturePoint${count}`
    - `"hardpoint_template"` -> `Hardpoint_${count}`
    - `"salvage_point_template"` -> `SalvagePoint${count}`
    - `"weapon_template"` -> `Weapon_${count}`
  - Ensure counter logic removes underscores for indices and begins numbering at 0 (e.g. `NavLight0`, `marker0`, `EngineNozzle0`).
  - In `handleAddNode`, implement multi-joint assembly templates with precise parent-child nested relations and scaled +5.0 offsets:
    - **Hardpoint**: `Hardpoint_name_Position` -> `_Direction` (+5.0y), `_Rest` (+5.0z) under `Position`.
    - **Capture Point**: `CapturePoint#` -> `Heading` (+5.0z), `Left` (+5.0z), `Up` (+5.0y) under `CapturePoint#`.
    - **Repair Point**: `RepairPoint#` -> `Heading` (+5.0z), `Left` (+5.0x), `Up` (+5.0y) under `RepairPoint#`.
    - **Salvage Point**: `SalvagePoint#` -> `Heading` (+5.0z), `Left` (+5.0z), `Up` (+5.0y) under `SalvagePoint#`.
    - **Weapon Turret**: `Weapon_name_Position` -> `_Direction` (+5.0y), `_Latitude` (+5.0z), `_Barrel` (+5.0y under Latitude) -> `_Muzzle` (+5.0y under Barrel), `_Rest` (+5.0z) under `Position`.
    - **Weapon Gimbal**: `Weapon_name_Position` -> `_Direction` (+5.0y), `_Muzzle` (+5.0z), `_Rest` (+5.0z) under `Position`.

## 3. Numeric Wheel Integer Scrubbing
- **Files to Modify**: `src/components/Inspector.tsx`
- **Detailed Changes**:
  - Change default step to `1` in mouse wheel handlers `handleNumericWheel` or numeric coords.
  - Modify wheel delta-step multipliers inside translations inputs to increment by `1.0` (whole number) instead of `0.05` decimals per wheel scroll move.

## 4. Default Panel Layout Width Expansion
- **Files to Modify**: `src/App.tsx`
- **Detailed Changes**:
  - Change `sidebarWidth` state initialization from `280` to `364` (a 30% wider default sidebar view) to accommodate deeply nested joint names.

## 5. Alerts and Warnings Panel
- **Files to Modify**: `src/App.tsx` or `src/components/HierarchyTree.tsx`
- **Detailed Changes**:
  - Build a beautiful dynamic alerts card panel highlighting structural warnings (missing weapon joints, engine burn count limits, missing collision/navlights, etc.).

## 6. Ship Animations Creation UI
- **Files to Modify**: `src/components/Viewport.tsx`
- **Detailed Changes**:
  - In the floating Animation Player panel, add:
    - A **Create New Animation** modal trigger button that appends a fresh `HODAnimation` object with name and duration to `model.animations`.
    - An **Add Track** joint-mapping dropdown to attach selected joints to the animation tracks.
    - An **Add Keyframe** button next to the slider scrubbing interface to record local positions, rotations, and scales of the active selected joint directly to keyframes.

## 7. Fix HOD 1.0 Mesh Loading (Chunk Trimming)
- **Files to Modify**: `parser/src/hod.rs`
- **Detailed Changes**:
  - Trim `sub_chunk.id` in `match sub_chunk.id.trim()` on line 320 and 426.
  - Trim child IDs to allow precise string matching of `"MULT"`, `"GOBG"`, `"STAT"`, `"LMIP"`, and `"BMSH"` joints even if they contain trailing spaces or nulls.

## 8. Engine Burns Sizing Limiter
- **Files to Modify**: `src/components/HierarchyTree.tsx`
- **Detailed Changes**:
  - In `handleAddNode`, check `model.engine_burns.len() >= 6`.
  - Block addition of new engine burns or nozzle templates if 6 already exist, showing an alert warning to the user.

## 9. Verification & Diagnostics Loop
- Ensure both TypeScript and Rust build processes run perfectly:
  - Frontend: `npx tsc --noEmit`
  - Backend: `cargo check`

## 10. Fix HIER Chunk Order inside DTRM Container to Prevent Game Crashes
- **Files to Modify**: `parser/src/hod.rs`
- **Detailed Changes**:
  - In `save_edits` function, move the newly serialized `hier_chunk` creation and insertion to be the **very first child** added to `new_children` inside the `DTRM` chunk.
  - This ensures `FORM:HIER` is always written first in `DTRM` chunk payload, preventing out-of-order joint referencing crashes in the game engine.

## 11. Multi-Joint Grouping & Naming Conventions
- **Files to Modify**: `src/components/HierarchyTree.tsx`, `src/components/Inspector.tsx`
- **Detailed Changes**:
  - Add unified grouping node rendering for all 6 template types inside `HierarchyTree.tsx` (using crosshair, shield, radio, wrench, and box icons for Weapons, Capture, Hardpoints, Repair, and Salvage points, respectively).
  - Implement full validation rules for all 6 template types inside `getWarnings()` (raising specific warnings for missing components or incorrect parenting structures).
  - Extend the selection inspector and rename tools in `Inspector.tsx` to handle all 5 point group types (Weapons, Hardpoints, Capture, Repair, and Salvage points), displaying structural completion checkmarks, missing joint card lists in red-dashed borders, and an automated "Repair Structure" button designed specifically for each component type (automatically recreating any missing sub-joints under correct parent-child relationships and snapping coordinates back to standard +5.0 offsets).
