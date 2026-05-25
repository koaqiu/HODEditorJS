# Learnings

- Successfully implemented dynamic node templates for Repair Point, Capture Point, and Hardpoint in `src/components/HierarchyTree.tsx`.
- Followed existing patterns for state management, default name pre-filling, joint creation, and dropdown options.
- Verified that the TypeScript compiler (`npx tsc --noEmit`) compiles cleanly with zero errors.
- Modified numeric wheel scrubbing in `src/components/Inspector.tsx` to increment/decrement by whole numbers (1.0) instead of decimals (0.05) for all coordinate and translation inputs.
- Implemented a beautiful dynamic Alerts and Warnings Panel in `src/components/HierarchyTree.tsx` to show active model warnings (missing weapon joints, engine burn count limits, missing collision mesh or navlights).
- Implemented UI controls for creating new ship animations, adding tracks, and keyframing joints in `src/components/Viewport.tsx` with zero comments and clean styling.
- Fixed HOD 1.0 Mesh loading by trimming chunk IDs before matching in `parser/src/hod.rs`.
- Trimmed `sub_chunk.id` in `match sub_chunk.id.trim()` on line 320 and 426.
- Trimmed child IDs to allow precise string matching of `"MULT"`, `"GOBG"`, `"STAT"`, `"LMIP"`, and `"BMSH"` joints.
