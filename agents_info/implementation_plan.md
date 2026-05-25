# Implementation Plan: Animation Editor — Full UI (Phase 3.5)

## Background

The backend pipeline for animations is complete — the Rust parser loads `.mad` companion files and embedded `KEYF` tracks, and the data flows into `HODModel.animations`. The user can see the "Animations" tab in the sidebar and the loaded animation name appears there.

**What the user sees today:**
- ✅ Animations tab in the sidebar lists animation names, track count, total keyframes, joint channel names
- ✅ A slim timeline bar floats at the bottom of the viewport with Play/Stop, a scrubber, an animation dropdown, and buttons: "New Anim", "Add Track...", "Add Keyframe", "Compile .MAD"
- ✅ A pulsing green badge at the top says "N Animations Loaded — Use Timeline Below ↓"

**What the user CANNOT see/do:**
- ❌ The viewport timeline bar is not visible — it's there but completely hidden because its `bottom: 16px` position is **clipped by the parent container's `overflow: hidden`** or sits behind the inspector panel on narrow layouts
- ❌ No visual timeline showing keyframes as markers on a horizontal ruler (like a real NLE/DCC)
- ❌ Cannot delete animations or rename them from the sidebar
- ❌ Cannot delete keyframes or edit keyframe values (time, position, rotation)
- ❌ Cannot delete animation tracks
- ❌ No loop toggle or playback speed controls
- ❌ The "Compile .MAD" button outputs a plain-text stub, not the real binary IFF `.mad` format
- ❌ No visual graph/curve editor for animation channels
- ❌ No indication of which joint is animating while playing (highlighted joint)

---

## Root Cause: Timeline Bar Invisible

The floating timeline panel at `bottom: 16px` inside the viewport `div` with `overflow: hidden` is likely clipped. The fix is to either:
1. Move the animation panel **outside** the viewport container (into `App.tsx` as a dedicated horizontal dock below the viewport), or
2. Change the viewport container to `overflow: visible` (but this breaks the clip mask for 3D canvas).

**Chosen approach**: Extract the animation panel from `Viewport.tsx` and render it as a **dedicated animation dock row** in `App.tsx`, sandwiched between the viewport and the inspector bottom edge. This guarantees it's always visible and avoids overflow clipping.

---

## Proposed Changes

### Phase 3.5-A: Make the Timeline Panel Always Visible

#### [MODIFY] [App.tsx](file:///run/media/system/Data/SteamLibrary/steamapps/common/Homeworld%20347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/src/App.tsx)
- Add `showAnimBar: boolean` state (true when `model?.animations?.length > 0`).
- Render a new `<AnimationDock>` component **below** the `<Viewport>` row in the main layout flex column, instead of inside the viewport.
- Pass it: `model`, `selectedAnimIdx`, `setSelectedAnimIdx`, `isPlaying`, `setIsPlaying`, `currentTime`, `setCurrentTime`, `onModelChange`.

#### [NEW] [AnimationDock.tsx](file:///run/media/system/Data/SteamLibrary/steamapps/common/Homeworld%20347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/src/components/AnimationDock.tsx)
A dedicated horizontal bar that spans the full width below the 3D viewport. Contains:

**Left cluster:**
- ⏮ Rewind to 0 button
- ▶/⏸ Play/Pause toggle button
- ⏹ Stop (reset to 0) button
- 🔁 Loop toggle button (loops playback when enabled)
- Speed selector: `0.25x`, `0.5x`, `1.0x`, `2.0x`

**Center cluster:**
- Animation name dropdown (`<select>` mapped to `selectedAnimIdx`)
- Time readout: `0.00s / 4.00s`
- ➕ New Animation button (opens create modal)
- 🗑 Delete Animation button (with confirmation)

**Right cluster:**
- Add Track dropdown (`<select>` of joints not yet in the active anim)
- Add Keyframe button (requires a joint to be selected in the hierarchy)
- 💾 Compile .MAD button (calls existing `handleCompileToMAD`)

**Timeline ruler row (below controls):**
- A full-width horizontal ruler showing:
  - Time ticks (0.0s, 0.5s, 1.0s … up to `duration`)
  - A draggable **playhead** red line at `currentTime`
  - For the active animation: one row per track, with **diamond keyframe markers** clickable to select/delete

Remove the old animation panel JSX from `Viewport.tsx` entirely.

---

### Phase 3.5-B: Sidebar Animations Tab — Edit Controls

#### [MODIFY] [HierarchyTree.tsx](file:///run/media/system/Data/SteamLibrary/steamapps/common/Homeworld%20347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/src/components/HierarchyTree.tsx)

Extend the Animations tab to add:
1. **Per-animation rename**: an inline text field (clicking animation name switches to edit mode).
2. **Per-animation delete**: a `🗑` icon button on hover next to the animation name.
3. **Per-track delete**: a `×` icon button on each joint channel row.
4. **Per-keyframe delete**: right-click or a `×` button on each `0.00s` keyframe badge.
5. **Keyframe value inspector**: clicking a keyframe badge expands an inline read-only table showing `time`, `pos [x, y, z]`, `rot [x, y, z, w]`.

All edit operations call `onModelChange(updatedModel)` to mutate through the standard state pipeline.

Props to add to `HierarchyTreeProps`:
```ts
onModelChange?: (m: HODModel) => void;
currentTime?: number;
```

---

### Phase 3.5-C: Playback Improvements

#### [MODIFY] [Viewport.tsx](file:///run/media/system/Data/SteamLibrary/steamapps/common/Homeworld%20347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/src/components/Viewport.tsx)
- Remove the old animation panel JSX (moved to `AnimationDock`).
- Keep the internal `evaluateAnimation` logic and the `isPlaying`/`currentTime` render-loop update.
- Accept new props: `isPlaying`, `setIsPlaying`, `currentTime`, `setCurrentTime`, `playbackSpeed`, `loopPlayback` from `App.tsx` (lifted up).
- **Highlight animated joint**: When `isPlaying`, change the material color of any joint sphere in `jointsGroup` whose name appears in the active animation's tracks to a brighter green.

#### [MODIFY] [App.tsx](file:///run/media/system/Data/SteamLibrary/steamapps/common/Homeworld%20347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/src/App.tsx)
- Lift `isPlaying`, `currentTime`, `playbackSpeed`, `loopPlayback` state up from `Viewport` into `App.tsx`.
- Pass them down into both `<Viewport>` and `<AnimationDock>`.

---

### Phase 3.5-D: Compile .MAD — Real Binary Output (Stretch Goal)

#### [MODIFY] [hod.rs](file:///run/media/system/Data/SteamLibrary/steamapps/common/Homeworld%20347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/parser/src/hod.rs)
- Add a Tauri command `compile_mad_from_anims` that accepts the `HODModel` JSON, and writes a real binary `.mad` IFF file using the same serialization path as `save_edits` → MAD writer.
- The "Compile .MAD" button in `AnimationDock` calls this command via `invoke("compile_mad_from_anims", ...)` instead of the current plain-text dump.

---

## Verification Plan

### Manual
1. Load `ter_fenris.hod` — the AnimationDock should appear below the viewport with the "radar" animation selected.
2. Press Play — the radar mesh rotates in the viewport while the playhead scrubs along the ruler.
3. Press Stop — mesh resets, playhead returns to 0.
4. Enable Loop — animation loops automatically.
5. Click a keyframe diamond — the sidebar highlights the corresponding track and time.
6. Delete an animation via the `🗑` button — the dropdown and sidebar update immediately.
7. Create a new animation via "New Animation" — it appears in the dropdown.
8. "Compile .MAD" produces a valid binary `.mad` file loadable in Homeworld Remastered.

### Build
- `npm run build` must produce 0 TypeScript errors.
- `cargo build` in `parser/` must produce 0 errors.

---

## Priority Order

1. **Phase 3.5-A** (AnimationDock visibility fix) — **highest priority**, user can't see the timeline at all
2. **Phase 3.5-B** (Sidebar edit controls) — needed for delete/rename workflows
3. **Phase 3.5-C** (Playback improvements) — quality of life
4. **Phase 3.5-D** (Binary .MAD compile) — currently outputs a text stub; needs real binary
