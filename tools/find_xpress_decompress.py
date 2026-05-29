# Ghidra headless script to find Xpress LZ77 decompression code in HomeworldRM.exe
# Usage: analyzeHeadless /tmp/ghidra_project HomeworldRM -import <exe> -postScript find_xpress_decompress.py

from ghidra.app.decompiler import DecompInterface
from ghidra.util.task import ConsoleTaskMonitor

def find_string_refs(program, search_str):
    """Find all references to a string in the binary."""
    results = []
    listing = program.getListing()
    mem = program.getMemory()
    
    # Search for the string in memory
    for block in mem.getBlocks():
        if not block.isInitialized():
            continue
        addr = block.getStart()
        end = block.getEnd()
        
        # Read bytes and search for string
        offset = 0
        while addr.add(offset).compareTo(end) < 0:
            try:
                data = mem.getBytes(addr.add(offset), min(256, end.subtract(addr.add(offset))))
                idx = find_bytes(data, search_str)
                if idx >= 0:
                    found_addr = addr.add(offset + idx)
                    results.append(found_addr)
                    offset += idx + len(search_str)
                else:
                    break
            except:
                break
    
    return results

def find_bytes(haystack, needle):
    """Find needle bytes in haystack."""
    needle_bytes = needle.encode('utf-8')
    for i in range(len(haystack) - len(needle_bytes)):
        match = True
        for j in range(len(needle_bytes)):
            if haystack[i + j] != needle_bytes[j]:
                match = False
                break
        if match:
            return i
    return -1

def decompile_function(func):
    """Decompile a function and return pseudocode."""
    decompiler = DecompInterface()
    decompiler.openProgram(currentProgram)
    
    results = decompiler.decompileFunction(func, 30, monitor)
    if results and results.decompileCompleted():
        return results.getDecompiledFunction().getC()
    return None

def analyze():
    """Main analysis function."""
    program = currentProgram
    print("=" * 80)
    print("Ghidra Xpress Decompression Analyzer")
    print("=" * 80)
    print()
    print("Program: %s" % program.getName())
    print("Language: %s" % program.getLanguageID())
    print()
    
    # Search for compression-related strings
    search_strings = [
        "ArchiveCompressStream",
        "Compress Stream",
        "compression method",
        "compressed data error",
        "ArchiveCompress",
    ]
    
    found_refs = {}
    for s in search_strings:
        refs = find_string_refs(program, s)
        if refs:
            found_refs[s] = refs
            print("Found string '%s' at %d location(s):" % (s, len(refs)))
            for ref in refs:
                print("  0x%s" % ref.toString())
    
    print()
    
    # Find functions that reference these strings
    func_manager = program.getFunctionManager()
    listing = program.getListing()
    
    # Look for functions containing Xpress-like patterns
    # The decompressor typically has:
    # 1. A loop reading 32-bit indicator words
    # 2. Bit shifting operations
    # 3. Match type decoding (checking low bits of a byte)
    
    print("Searching for decompression-like functions...")
    print()
    
    decompiler = DecompInterface()
    decompiler.openProgram(program)
    
    candidates = []
    
    for func in func_manager.getFunctions(True):
        # Check if function name contains compression-related keywords
        name = func.getName().lower()
        if any(kw in name for kw in ['compress', 'decompress', 'xpress', 'lz77', 'lzss']):
            candidates.append(func)
            print("Found named function: %s at 0x%s" % (func.getName(), func.getEntryPoint()))
        
        # Also check functions that reference our found strings
        # (this is slower but catches unnamed functions)
    
    print()
    print("Found %d candidate functions by name" % len(candidates))
    print()
    
    # If we found ArchiveCompressStream, look for its methods
    if "ArchiveCompressStream" in found_refs:
        print("Looking for ArchiveCompressStream class methods...")
        for ref_addr in found_refs["ArchiveCompressStream"]:
            # Find the function containing this reference
            func = func_manager.getFunctionContaining(ref_addr)
            if func:
                print("  Function: %s at 0x%s" % (func.getName(), func.getEntryPoint()))
                candidates.append(func)
    
    # Decompile candidates and look for Xpress patterns
    print()
    print("=" * 80)
    print("Decompiling candidate functions...")
    print("=" * 80)
    
    xpress_funcs = []
    
    for func in candidates:
        print()
        print("--- Function: %s at 0x%s ---" % (func.getName(), func.getEntryPoint()))
        code = decompile_function(func)
        if code:
            # Check for Xpress decompression patterns
            has_indicator = False
            has_bit_shift = False
            has_match_type = False
            
            for line in code.split('\n'):
                line_lower = line.lower()
                if 'indicator' in line_lower or '0x1f' in line_lower or 'bit' in line_lower:
                    has_indicator = True
                if '>>' in line or '<<' in line or '&' in line:
                    has_bit_shift = True
                if '0x03' in line or '0x07' in line or '0b11' in line:
                    has_match_type = True
            
            if has_indicator and has_bit_shift:
                xpress_funcs.append(func)
                print("  ** LIKELY XPRESS DECOMPRESSOR **")
                print()
                # Print first 100 lines of decompiled code
                lines = code.split('\n')
                for i, line in enumerate(lines[:100]):
                    print("  %s" % line)
                if len(lines) > 100:
                    print("  ... (%d more lines)" % (len(lines) - 100))
            else:
                print("  (does not look like Xpress decompressor)")
                # Print first 20 lines for context
                lines = code.split('\n')
                for line in lines[:20]:
                    print("  %s" % line)
                if len(lines) > 20:
                    print("  ... (%d more lines)" % (len(lines) - 20))
    
    print()
    print("=" * 80)
    print("SUMMARY")
    print("=" * 80)
    print()
    print("Candidate functions found: %d" % len(candidates))
    print("Likely Xpress decompressors: %d" % len(xpress_funcs))
    
    for func in xpress_funcs:
        print()
        print("Function: %s" % func.getName())
        print("Address: 0x%s" % func.getEntryPoint())
        print("Parameters:")
        for param in func.getParameters():
            print("  %s %s" % (param.getDataType().getName(), param.getName()))

# Run the analysis
analyze()
