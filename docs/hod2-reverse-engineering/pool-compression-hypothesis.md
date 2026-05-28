# In-Game Vertex Explosion: POOL Compression Hypothesis & Experiment

## 1. Problem Statement
The generated `ter_centaur` HOD 2.0 file compiled from source assets exhibits a severe "vertex explosion" / spikiness artifact in-game, where a large portion of the vertices are pinned/stretched back to the origin `(0, 0, 0)`.

## 2. The Pool Compression Hypothesis
Our custom Microsoft Xpress compression implementation (`compress` in `parser/src/xpress.rs`) successfully round-trips with our own decompressor in Rust, achieving a 100% success rate on structural parsing and lossless round-trip tests.

However, the game engine's custom/outdated in-engine Xpress decompressor might fail or abort midway through the compressed mesh stream if it encounters a match pattern it does not support or decode correctly.

* **Impact**: When the game's decompressor fails or stops early, it exits without decompressing the remaining portion of the mesh vertex pool.
* **Symptom**: The unpopulated/undecompressed region of the buffer remains filled with default `0` bytes.
* **Visual Result**: All vertices in that undecompressed region have position coordinates of `(0.0, 0.0, 0.0)`. This causes the classic "spikiness" where those vertices are stretched to the 3D origin.

---

## 3. The Bypass Experiment
To isolate this hypothesis, we have temporarily **disabled/bypassed Xpress compression globally**.

HOD 2.0 POOL chunks store both compressed and decompressed sizes. If `compressed_size == decompressed_size`, the game engine recognizes the stream as uncompressed/raw and completely skips the decompression path, copying the bytes directly.

### Changes Made:
1. **Bypassed Compression globally**: In `parser/src/xpress.rs`, temporarily modified `compress_or_raw` to always return the uncompressed input buffer:
   ```rust
   pub fn compress_or_raw(input: &[u8]) -> Vec<u8> {
       input.to_vec()
   }
   ```
2. **Regenerated Fixtures**: Re-ran the replication suite:
   ```bash
   cargo run --bin test_hodor_replication
   ```
   This regenerated `ter_centaur_generated.hod` with:
   * **Texture Pool**: compressed = 87424, decompressed = 87424 (Raw)
   * **Mesh Pool**: compressed = 1183616, decompressed = 1183616 (Raw)
   * **Face Pool**: compressed = 37704, decompressed = 37704 (Raw)

---

## 4. How to Interpret the Test Results
Once the generated uncompressed HOD is loaded in-game:

* **Scenario A (Spikiness Disappears)**:
  If the mesh renders perfectly without spikiness, **the compression hypothesis is 100% correct**. The issue is a slight match-pattern mismatch between our Xpress compressor and the game engine's decompressor. We can then refine the compressor to emit simpler/safer LZ77 match headers that are fully compatible.
  
* **Scenario B (Spikiness Persists)**:
  If the mesh is still exploded even with raw/uncompressed pools, **the compression is not the cause**. The origin pinning is instead caused by incorrect/mismatched vertex element strides, missing joint index weights, or transformations in the serialized buffer itself.
