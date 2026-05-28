# Path 2: RtlCompressBuffer Validation

## Purpose

Test whether the Homeworld Remastered game engine uses the Windows `RtlCompressBuffer` API for POOL stream compression. This is a **validation step only** — if it matches, we port the ReactOS/Wine implementation to pure Rust for cross-platform support.

## Status

**Started:** 2026-05-29  
**Agent:** OpenCode Agent  
**Phase:** BLOCKED — Wine does not implement XPRESS compression (format 3)

## Result

Wine's `RtlGetCompressionWorkSpaceSize` returns `0xc000025f` (STATUS_UNSUPPORTED_COMPRESSION) for format 3 (XPRESS). Wine/staging 10.15 does not implement XPRESS compression.

This means we **cannot validate** via Wine whether the game engine uses the Windows API. 

**Conclusion:** Path 2 is blocked on Linux. Options:
1. Use a real Windows machine to run the test (if available)
2. Move to Path 3 (byte-matching) or Path 1 (Ghidra)
3. Look for an open-source XPRESS compressor implementation (ReactOS, etc.)

## Files Created (still useful)

| File | Purpose |
|------|---------|
| `tools/rtl_compress_test.c` | C program calling RtlCompressBuffer (ready for Windows) |
| `parser/src/bin/extract_pool_streams.rs` | Extracts decompressed pools from HOD files |
| `testing/ter_centaur/rtl_test/decomp_*.bin` | Decompressed pool data |
| `testing/ter_centaur/rtl_test/hodor_comp_*.bin` | HODOR's compressed bytes for comparison |

## Workflow

```
Write C test program
        ↓
Compile with MinGW
        ↓
Run under Wine
        ↓
Compare output with HODOR bytes
        ↓
   ┌────┴────┐
 Match     No match
   ↓         ↓
Port to    Move to
Rust       Path 1/3
(Path 4)
```

## Files

| File | Purpose |
|------|---------|
| `tools/rtl_compress_test.c` | C program calling RtlCompressBuffer |
| `tools/rtl_compress_test.exe` | Compiled Windows binary |
| `tools/rtl_compress_test.sh` | Wine runner script |
| `testing/ter_centaur/rtl_mesh_pool.bin` | Compressed mesh pool output |
| `testing/ter_centaur/rtl_face_pool.bin` | Compressed face pool output |
| `testing/ter_centaur/rtl_tex_pool.bin` | Compressed texture pool output |

## Step 1: Check Prerequisites

```bash
# Check MinGW
which x86_64-w64-mingw32-gcc

# Check Wine
which wine
wine --version
```

## Step 2: Write Test Program

The C program needs to:
1. Read decompressed data from a file
2. Call `RtlCompressBuffer(COMPRESSION_FORMAT_XPRESS, ...)`
3. Write compressed output to a file
4. Print compressed size for comparison

## Step 3: Compile

```bash
x86_64-w64-mingw32-gcc -o tools/rtl_compress_test.exe tools/rtl_compress_test.c -lntdll
```

## Step 4: Extract Decompressed Data

Use a Rust binary to extract the decompressed mesh/face/texture pools from `ter_centaur_hodor.hod` into raw binary files.

## Step 5: Run Under Wine

```bash
wine tools/rtl_compress_test.exe <input.bin> <output.bin>
```

## Step 6: Compare

Compare the output bytes with HODOR's compressed bytes using `xpress_compare` or direct byte comparison.

## Decision Matrix

| Result | Action |
|--------|--------|
| Bytes match HODOR exactly | Game uses Windows API → Port to Rust (Path 4) |
| Bytes are close but differ | Game uses similar algorithm → Investigate differences |
| Bytes are completely different | Game uses custom implementation → Move to Path 1/3 |
| API call fails | API not available or different parameters → Move to Path 1/3 |

## Progress Log

| Date | Step | Result | Notes |
|------|------|--------|-------|
| 2026-05-29 | Setup | ✅ Complete | Created C program, extracted decompressed pools |
| 2026-05-29 | Install MinGW/Wine | ✅ Complete | Installed via distrobox esp-dev |
| 2026-05-29 | Compile C program | ✅ Complete | Compiled with MinGW, minor macro warnings |
| 2026-05-29 | Run under Wine | ❌ BLOCKED | Wine does not implement XPRESS compression (format 3) |
| 2026-05-29 | Decision | PIVOT | Moving to Path 3 (byte-matching) |
