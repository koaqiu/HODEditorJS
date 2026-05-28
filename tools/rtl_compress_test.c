/*
 * RtlCompressBuffer Validation Test
 * 
 * Tests whether the Windows RtlCompressBuffer API produces bytes matching
 * HODOR's compressed POOL data. This is a validation step only — if it
 * matches, we port the ReactOS/Wine implementation to pure Rust.
 *
 * Compile: x86_64-w64-mingw32-gcc -o rtl_compress_test.exe rtl_compress_test.c -lntdll
 * Run:     wine rtl_compress_test.exe <input.bin> <output.bin> [format]
 *   format: 3 = XPRESS (default), 4 = XPRESS_HUFF
 */

#include <windows.h>
#include <stdio.h>
#include <stdlib.h>

/* RtlCompressBuffer prototype from ntdll.dll */
typedef NTSTATUS (WINAPI *RtlCompressBufferFunc)(
    USHORT CompressionFormat,
    PUCHAR UncompressedBuffer,
    ULONG UncompressedBufferLength,
    PUCHAR CompressedBuffer,
    ULONG CompressedBufferLength,
    ULONG UncompressedChunkSize,
    PULONG FinalCompressedSize,
    PVOID WorkSpace
);

/* RtlGetCompressionWorkSpaceSize prototype */
typedef NTSTATUS (WINAPI *RtlGetCompressionWorkSpaceSizeFunc)(
    USHORT CompressionFormat,
    USHORT CompressionEngine,
    PULONG CompressBufferWorkSpaceSize,
    PULONG CompressFragmentWorkSpaceSize
);

#define COMPRESSION_FORMAT_XPRESS     3
#define COMPRESSION_FORMAT_XPRESS_HUFF 4

int main(int argc, char *argv[]) {
    if (argc < 3) {
        fprintf(stderr, "Usage: %s <input.bin> <output.bin> [format]\n", argv[0]);
        fprintf(stderr, "  format: 3=XPRESS (default), 4=XPRESS_HUFF\n");
        return 1;
    }

    USHORT format = COMPRESSION_FORMAT_XPRESS;
    if (argc >= 4) {
        format = (USHORT)atoi(argv[3]);
    }

    /* Load ntdll.dll functions */
    HMODULE ntdll = GetModuleHandleA("ntdll.dll");
    if (!ntdll) {
        fprintf(stderr, "Failed to get ntdll.dll handle\n");
        return 1;
    }

    RtlCompressBufferFunc pRtlCompressBuffer = (RtlCompressBufferFunc)
        GetProcAddress(ntdll, "RtlCompressBuffer");
    RtlGetCompressionWorkSpaceSizeFunc pRtlGetCompressionWorkSpaceSize = 
        (RtlGetCompressionWorkSpaceSizeFunc)
        GetProcAddress(ntdll, "RtlGetCompressionWorkSpaceSize");

    if (!pRtlCompressBuffer || !pRtlGetCompressionWorkSpaceSize) {
        fprintf(stderr, "Failed to get RtlCompressBuffer/RtlGetCompressionWorkSpaceSize\n");
        return 1;
    }

    /* Get workspace size */
    ULONG compressWorkSpaceSize = 0;
    ULONG compressFragmentWorkSpaceSize = 0;
    NTSTATUS status = pRtlGetCompressionWorkSpaceSize(
        format, 0, &compressWorkSpaceSize, &compressFragmentWorkSpaceSize);
    if (status != 0) {
        fprintf(stderr, "RtlGetCompressionWorkSpaceSize failed: 0x%08lx\n", status);
        return 1;
    }
    fprintf(stderr, "Workspace size: %lu bytes\n", compressWorkSpaceSize);

    /* Read input file */
    FILE *fin = fopen(argv[1], "rb");
    if (!fin) {
        fprintf(stderr, "Failed to open input: %s\n", argv[1]);
        return 1;
    }
    fseek(fin, 0, SEEK_END);
    ULONG inputSize = (ULONG)ftell(fin);
    fseek(fin, 0, SEEK_SET);

    PUCHAR inputBuf = (PUCHAR)malloc(inputSize);
    if (!inputBuf) {
        fprintf(stderr, "Failed to allocate input buffer\n");
        fclose(fin);
        return 1;
    }
    fread(inputBuf, 1, inputSize, fin);
    fclose(fin);

    fprintf(stderr, "Input: %s (%lu bytes)\n", argv[1], inputSize);

    /* Allocate workspace and output buffer */
    PVOID workSpace = malloc(compressWorkSpaceSize);
    ULONG maxOutputSize = inputSize + 4096; /* generous margin */
    PUCHAR outputBuf = (PUCHAR)malloc(maxOutputSize);

    if (!workSpace || !outputBuf) {
        fprintf(stderr, "Failed to allocate buffers\n");
        free(inputBuf);
        return 1;
    }

    /* Compress */
    ULONG finalCompressedSize = 0;
    status = pRtlCompressBuffer(
        format,
        inputBuf,
        inputSize,
        outputBuf,
        maxOutputSize,
        0,              /* UncompressedChunkSize (0 = default) */
        &finalCompressedSize,
        workSpace
    );

    if (status != 0) {
        fprintf(stderr, "RtlCompressBuffer failed: 0x%08lx\n", status);
        free(inputBuf);
        free(outputBuf);
        free(workSpace);
        return 1;
    }

    fprintf(stderr, "Output: %lu bytes (%.1f%% of input)\n", 
        finalCompressedSize, 100.0 * finalCompressedSize / inputSize);

    /* Write output file */
    FILE *fout = fopen(argv[2], "wb");
    if (!fout) {
        fprintf(stderr, "Failed to open output: %s\n", argv[2]);
        free(inputBuf);
        free(outputBuf);
        free(workSpace);
        return 1;
    }
    fwrite(outputBuf, 1, finalCompressedSize, fout);
    fclose(fout);

    fprintf(stderr, "Written to: %s\n", argv[2]);

    free(inputBuf);
    free(outputBuf);
    free(workSpace);

    return 0;
}
