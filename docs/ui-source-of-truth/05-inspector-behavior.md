# 05 Inspector Behavior

## Scope

This spec covers the right-side inspector for selected nodes, assembly repair, field editing, and import/export controls exposed from the inspector.

Primary sources:

- `src/components/Inspector.tsx`
- `src/components/Viewport.tsx` for model types
- `src/App.tsx` for transform/update callbacks

## Empty State

When no node is selected or no model is loaded, Inspector should show the corresponding empty state rather than editing stale data.

## Assembly Inspector

Inspector recognizes special group selections for weapon, hardpoint, capture, repair, and salvage assemblies.

For assemblies, Inspector shows completion state, missing component rows, and a repair action when required joints are missing. Repair recreates missing joints with default offsets and parent relationships from current code.

Assembly rename behavior must preserve related joint references and avoid duplicate names.

## Joint And Marker Editing

Joint and marker editing updates transform-related fields through the model update callbacks. Transform edits should remain synchronized with viewport gizmo behavior.

Marker edits update marker position/rotation data and parent association according to current Inspector code.

## NavLight Editing

NavLight editor fields include size, phase, frequency, style, distance, color, and enabled/toggle fields currently implemented in `Inspector.tsx`.

NavLight UI edits must update navlight data and any associated joint data through `onModelChange` or the relevant callback path.

## Dockpath And Dockpoint Editing

Dockpath/dockpoint inspector supports editing dockpoint position and properties such as tolerance and max speed according to current code.

Dockpoint selection names are derived from dockpath name and point index.

## Collision Editing

Collision inspector supports editing center/radius/extents and selecting or calculating from a source mesh when current code exposes that action.

Collision mesh parent access may be optional in imported/malformed data; UI code should preserve null-safe handling.

## Mesh Editing And OBJ Flow

Mesh inspector supports editing mesh name, LOD, parent, and materials according to current code. OBJ import/export controls are exposed from Inspector and route through app/Tauri callbacks.

## Engine Nodes

Inspector contains editors for engine burn/glow/shape data. Agents must verify exact fields in `Inspector.tsx` before adding or documenting new engine-specific controls.

## Materials And Textures

Material and texture editing lives in Inspector and related hierarchy material import/export controls. Do not duplicate material state outside model updates.
