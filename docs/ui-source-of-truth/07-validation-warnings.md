# 07 Validation Warnings

## Scope

This spec covers current hierarchy diagnostics, warnings, and repair-related UI signals.

Primary sources:

- `src/components/HierarchyTree.tsx`
- `src/components/Inspector.tsx`
- `TODO.md` only as secondary context

## Hierarchy Diagnostics

`HierarchyTree.tsx` computes warnings through `getWarnings()` and renders them in the diagnostics panel.

Current warning/info categories include:

- Weapon group missing required joints.
- Engine burn count at or above the current warning threshold.
- Missing collision mesh data.
- Missing navlight data.

Do not claim broader validation coverage unless current code implements it.

## Assembly Repair Signals

Inspector shows assembly completion/incompletion state and missing component rows for supported assemblies. Repair actions recreate missing required joints for the selected assembly family.

Diagnostics and repair UI should agree on which components are required. If they disagree, inspect both `HierarchyTree.tsx` and `Inspector.tsx` before changing behavior.

## Warning Severity

Current diagnostics are user guidance and integrity checks. They do not block saving by themselves unless save logic explicitly enforces a condition.

## Non-Goals

These docs do not assert complete HOD validation, parser-level validation, game-runtime validation, or binary layout correctness. Those belong in parser/binary specs and tests.
