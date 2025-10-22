import ctypes
import os
import sys

# Use the correct path to the shared library
libpath = os.path.join(os.getcwd(), "target", "release", "libparflow_c.so")
print("Trying to load", libpath)

if not os.path.exists(libpath):
    print("Library not found at:", libpath)
    sys.exit(1)

try:
    lib = ctypes.CDLL(libpath)
    
    # Call the C function
    res = lib.run_orchestrator_par()
    print("✅ Result from FFI parallel (sum):", res)
    
    # Test sequential version too
    res_seq = lib.run_orchestrator_seq()
    print("✅ Result from FFI sequential (sum):", res_seq)
    
    if res == 3 and res_seq == 3:
        print("🎉 All C FFI tests passed!")
    else:
        print("❌ Unexpected results")
    
except Exception as e:
    print(f"❌ Error loading or calling library: {e}")
    print("This might be due to missing dependencies. Let's check:")
    
    # Try to check library dependencies
    try:
        import subprocess
        result = subprocess.run(['ldd', libpath], capture_output=True, text=True)
        print("Library dependencies:")
        print(result.stdout)
        if result.stderr:
            print("Errors:", result.stderr)
    except:
        print("Could not run ldd to check dependencies")
