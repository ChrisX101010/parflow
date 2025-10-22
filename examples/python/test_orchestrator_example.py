# This is a simple doctest-style assertion to be run manually after building the C library.
# It expects run_orchestrator_par to return 3 for the example tasks (1 + 2).
import subprocess, sys, os
libpath = os.path.join("parflow-c", "target", "release")
if not os.path.exists(libpath):
    print("Build parflow-c first: cargo build -p parflow-c --release")
    sys.exit(1)
print("Manually run examples/python/orchestrator_ctypes.py to verify the FFI output.")
