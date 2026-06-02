# 03 Hierarchy Tree Interactions

## Scope

This spec covers hierarchy rendering, selection, search, collapse, visibility, drag/drop, and context-menu behavior.

Primary sources:

- `src/components/HierarchyTree.tsx`
- `src/App.tsx` for `onReParentNode`
- `src/components/Viewport.tsx` for visibility effects

## Rendered Node Types

The hierarchy tree renders:

- Root and root-level joints.
- Child joints.
- Weapon group folders.
- Markers.
- Mesh LOD parts.
- Nav lights.
- Engine burns.
- Engine glows.
- Engine shapes.
- Collision hulls.
- Dockpaths and dockpoints.

Pure navlight position joints may be hidden from the general joint list unless they have meaningful children/attachments. Weapon assembly subnodes are hidden from the normal root/general joint list and shown under their weapon group folder.

## Selection

Clicking a tree row sets `selectedNode` with the row's logical type and name. Selection drives Inspector and Viewport behavior.

Weapon group rows use type `weapon_group`. Weapon subnodes rendered inside the group remain type `joint` for selection and viewport/inspector compatibility.

## Search And Collapse

Search filters tree rows using direct names and recursive descendant matches. Collapsible rows use `collapsedJoints` state. Weapon group collapse keys are prefixed with `weapon_group:`.

## Visibility

Eye toggles update `visibleMeshes` keys. Toggling a joint propagates visibility to descendant keys. Toggling a weapon group propagates visibility to all component joints and their descendants.

Visibility keys use prefixes such as `joint:`, `marker:`, `navlight:`, `engine_burn:`, `engine_glow:`, `engine_shape:`, `collision:`, `dockpath:`, and `weapon_group:`.

## Drag And Drop

Tree rows set drag data with node name and node type. Drops call `onReParentNode` from props.

`App.tsx` owns actual reparenting behavior. Current app-side reparenting supports joints, markers, meshes, engine burns, and navlights. Agents must check `App.tsx` before claiming support for other dropped types.

Root is not draggable.

## Context Menu

Right-click opens a portal-based context menu at cursor coordinates. The menu contains Rename and, when allowed, Delete for standard tree nodes.

For the Textures list in the Materials tab, the context menu provides options to:
- Toggle Y-Flip (inverts `legacy_storage_y_flipped` and instantly updates the WebGL `flipY` state for preview).
- Remove Texture.

Weapon assembly subnodes must not open the right-click menu. This is enforced by the shared protected-subnode guard in `HierarchyTree.tsx`.

The weapon assembly/group row itself must keep its context menu so the assembly can be renamed or deleted as a group.
