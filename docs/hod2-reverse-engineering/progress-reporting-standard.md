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

Use this format for progress summaries:

```markdown
**Objective**
- One sentence describing the current HODOR replication target.

**Current Status**
- What works now, stated narrowly.
- What is still missing, stated plainly.

**Inputs Used**
- Allowed source inputs used by the current test.
- Explicitly state that `model.json` is not used.

**Completed Since Last Update**
- Concrete code/doc/test changes.

**Validation**
- Commands run.
- Pass/fail counts.
- Any meaningful mismatches.

**Next Target**
- The next small implementation step.

**Risks / Open Questions**
- Anything that could invalidate the result or still needs in-game/HODOR confirmation.
```

## Rules

- Do not describe the pipeline as complete unless it creates HODs from source assets and authored metadata without processed HOD payloads.
- Do not treat authored metadata JSON as cheating; it represents editor-created values.
- Do treat `model.json` as processed-output data and therefore invalid for replication implementation.
- Separate parser/writer round-trip success from HODOR behavior replication.
- Report exact tests run and what those tests compare.
- Prefer small, validated milestones over broad percentage-complete claims.
