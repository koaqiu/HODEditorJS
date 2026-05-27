# 01 App Shell And File Flow

## Scope

This spec covers top-level app state, toolbar actions, model loading, saving, new model creation, and file import/export entry points.

Primary sources:

- `src/App.tsx`
- `src/components/Toolbar.tsx`
- `src/components/Viewport.tsx`
- `src-tauri/src/lib.rs` for invoked backend commands

## Shell State

`App.tsx` owns the loaded `model`, selected node, dirty state, loading/saving flags, status/error messages, selected animation index, viewport visibility, transform mode, and file path state.

When no model is loaded, panels should show empty/no-model states. When a model is loaded, the hierarchy, inspector, viewport, toolbar actions, animation dock, and relevant overlays operate from the shared model state.

## Toolbar Actions

`Toolbar.tsx` exposes these top-level actions:

- Open HOD.
- Import DAE.
- Save.
- Save As.
- New HOD.

Save buttons must reflect disabled/loading state from app state. Toolbar should call callbacks passed from `App.tsx`; it should not own file persistence logic.

## New HOD

New HOD creation initializes a minimal model with a `Root` joint and empty editable collections. It should reset selection and mark the app state appropriately through `App.tsx`.

## Loading HOD And DAE

HOD loading is initiated from `App.tsx` and backed by Tauri commands. Loading should populate the model, clear stale selection, update file path/status, and handle errors through the app error state.

DAE import follows the app import flow and should produce/update a model through the existing app callbacks rather than bypassing app state.

## Saving

Save uses the current file path when available. Save As prompts for a destination. Both flows should keep `isSaving`, status, and dirty state coherent.

Agents must not add save side effects to lower-level components unless routed through existing app callbacks.

## Auto Collision Creation

When loading v2 models without collision hull data, `App.tsx` may auto-create default collision bounds from ship geometry. This is part of current load-time UI behavior.

## Viewport Shell Controls

`App.tsx` owns render mode and layer toggles passed to `Viewport.tsx`. Viewport controls must update app state through callbacks, not local-only duplicated state, when the setting affects global rendering.
