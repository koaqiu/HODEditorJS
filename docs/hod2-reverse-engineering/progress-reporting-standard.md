# Progress Reporting Standard

## Objective

All progress updates for HODOR replication must stay anchored to the overall project objective:

Create HOD 2.0 files from source assets and editor-authored metadata, then validate the generated HOD against HODOR output.

## Allowed Inputs

The replication pipeline may use these as source inputs:

- `*.obj` for geometry, LODs, face order, object names, and `usemtl` material assignment.
- `*.mtl` for source material names and texture references.
- `*.tga` for source texture image data.
- `materials.json` for editor-authored HOD material definitions when those values are authored in the editor.
- `joints.json` for editor-authored hierarchy data.
- `navlights.json` for editor-authored navlight data.
- `markers.json` for editor-authored marker data.
- `engine_burns.json` for editor-authored engine burn data.
- `collision_meshes.json` for editor-authored collision data.

## Forbidden Inputs

The replication pipeline must not use these as implementation source data:

- `model.json`.
- Processed HOD mesh payloads extracted from HODOR output.
- Processed HOD texture payloads extracted from HODOR output.
- Any data copied from parsed HODOR output to bypass OBJ/MTL/TGA ingestion.

HODOR-generated HOD files are allowed only as an oracle for parsing, comparison, and validation.

## Standard Update Format

Use this format for progress summaries in `PROGRESS.md`:

```markdown
**Objective**
- One sentence describing the current feature or fix target.

**Current Status**
- What works now, stated narrowly.
- What is still missing, stated plainly.

**Completed Since Last Update (Successes)**
- Concrete code/doc/test changes.
- Exact file paths and line number logic modified.
- Include exact success messages or metric improvements.

**What Failed / Remains Broken**
- Specific error messages, file:line locations.
- Root cause analysis if known. Never leave broken code without logging the error.

**Caveats & Quirks Found**
- Any strange behavior, unexpected limitations, or non-standard workarounds used to satisfy the game engine or UI framework.
- Ensure these are also permanently logged in `docs/hod2-reverse-engineering/hod2_reverse_engineering_knowledge_base.md`.

**Validation & Test Results**
- Commands run (e.g. `cargo run --bin verify_lossless`).
- Pass/fail counts.
- Any meaningful mismatches or UI screenshots taken.

**Commit Checkpoint**
- Note the Git Commit hash or message used to checkpoint this work session.

**Next Target**
- The next small implementation step.
```

## Rules

- Do not describe the pipeline as complete unless it creates HODs from source assets and authored metadata without processed HOD payloads.
- Do not treat authored metadata JSON as cheating; it represents editor-created values.
- Do treat `model.json` as processed-output data and therefore invalid for replication implementation.
- Separate parser/writer round-trip success from HODOR behavior replication.
- Report exact tests run and what those tests compare.
- Prefer small, validated milestones over broad percentage-complete claims.
- **Commit Checkpoints**: Before ending any session, run `git add . && git commit -m "..."` to save your work, and document the checkpoint in your final update.
