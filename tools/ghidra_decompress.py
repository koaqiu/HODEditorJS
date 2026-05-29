# Focused Ghidra headless script to decompile specific functions
from ghidra.app.decompiler import DecompInterface
from ghidra.util.task import ConsoleTaskMonitor

def run():
    decompiler = DecompInterface()
    decompiler.openProgram(currentProgram)
    
    fm = currentProgram.getFunctionManager()
    
    # Target addresses to decompile
    targets = [
        0x00783cc2,  # ArchiveCompressStream decompression dispatcher
        0x00783dbd,  # Small dispatcher
        0x00783df2,  # Read method (250 bytes)
        0x0077d171,  # References ArchiveCompressStream
        0x0077d2e8,  # References ArchiveCompressStream
    ]
    
    for addr_val in targets:
        addr = currentProgram.getAddressFactory().getDefaultAddressSpace().getAddress(addr_val)
        func = fm.getFunctionAt(addr)
        if func is None:
            func = fm.getFunctionContaining(addr)
        
        if func:
            print("=" * 80)
            print("Function: %s at 0x%s (size=%d)" % (func.getName(), func.getEntryPoint(), func.getBody().getNumAddresses()))
            print("=" * 80)
            
            results = decompiler.decompileFunction(func, 60, monitor)
            if results and results.decompileCompleted():
                code = results.getDecompiledFunction().getC()
                print(code)
            else:
                print("  Decompilation failed")
        else:
            print("No function at 0x%x" % addr_val)
    
    # Also search for any function that contains Xpress-like bit patterns
    # Look for functions with 'and eax, 0x3' or 'and eax, 0x7' (match type checks)
    print()
    print("=" * 80)
    print("Searching for functions with Xpress match-type patterns...")
    print("=" * 80)
    
    found = 0
    for func in fm.getFunctions(True):
        if func.getBody().getNumAddresses() > 500 and func.getBody().getNumAddresses() < 5000:
            # Medium-sized functions (likely decompression)
            results = decompiler.decompileFunction(func, 10, monitor)
            if results and results.decompileCompleted():
                code = results.getDecompiledFunction().getC()
                # Check for Xpress patterns
                if ('& 3' in code or '& 7' in code or '& 0x3' in code or '& 0x7' in code) and \
                   ('>>' in code or '<<' in code) and \
                   ('while' in code or 'for' in code):
                    print()
                    print("CANDIDATE: %s at 0x%s (size=%d)" % (func.getName(), func.getEntryPoint(), func.getBody().getNumAddresses()))
                    print("-" * 40)
                    # Print first 80 lines
                    lines = code.split('\n')
                    for line in lines[:80]:
                        print(line)
                    if len(lines) > 80:
                        print("... (%d more lines)" % (len(lines) - 80))
                    found += 1
                    if found >= 3:
                        break

run()
