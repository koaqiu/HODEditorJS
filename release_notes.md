## What's Changed
* **Viewport Gizmo Dragging**: Added the ability to seamlessly move and rotate joints directly in the 3D viewport using a visual gizmo.
* **Keyframe Dragging**: Dragging keyframed joints in the viewport now smoothly updates the keyframe without jitter or "fighting" the animation timeline.
* **EngineNozzle Gizmo Fix**: Fixed an issue where `EngineNozzle` joints could not be manipulated with the gizmo.
* **TransformControls Fixes**: Fixed several crash scenarios when ending a drag operation that rebuilt the scene.

## Reverse Engineering Milestones
* Dynamic Shader parameter support (e.g. `_SCORCHED`, `_ENV0`)
* NavLight and EngineNozzle implicit duplicate resolution inside HOD 2.0 chunk mappings
* Generic fallback routines for Weapon Assemblies parsing (`Latitude`, `Rest`, `Muzzle` resolution)
