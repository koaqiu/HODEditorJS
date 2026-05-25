# Decisions

- **Joint Counting for Unique Names**: Counted existing joints starting with the respective prefix (`RepairPoint`, `CapturePoint`, `Hardpoint_`) to generate unique default names.
- **Engine Burn Limiter**: Placed the limit check at the very beginning of the `engine_nozzle` block in `handleAddNode` to block addition and alert the user if 6 or more engine burns already exist.
- **Whole Number Coordinate Increments**: Changed the step attribute and wheel delta-step multipliers inside translation and coordinate inputs to 1.0/1 to allow users to scrub and increment by whole numbers instead of small decimals.
- **Model Diagnostics Panel Placement**: Placed the Alerts and Warnings Panel at the bottom of the hierarchy tree panel, visible only when the "Hierarchy" tab is active, to provide real-time structural feedback without cluttering the UI.
- **Animation Player Layout and Keyframing**: Redesigned the floating Animation Player panel into a clean two-row layout to accommodate new controls. Implemented local matrix decomposition to record precise local position, rotation, and scale keyframes for selected joints.
