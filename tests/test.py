# This is the main test file
#
# This file run the compiler over every single test (files with `.par` extension) on the directory,
# and then compares the output with the expected one (files with `.c` extensions)
#
# To run these tests, build the project to get a `parlan` executable, then move the executable into 
# this directory, finally run this file

# note: this is a very simple test suite, and will probably change soon to be more robust

import subprocess, filecmp
from pathlib import Path

tests = [
    ["glb_vars", False],
    ["loops", False],
    ["ifs", False],
    ["functions", False],
    #["type_checker_fail", True]
    #["semantic_checker_fail", True]
    
]

curr_dir = Path(__file__).resolve().parent
exe_path = curr_dir.parent / "target" / "release" / "parlan.exe"

def run_test(test, should_fail):
    try:
        command = [str(exe_path)] + [curr_dir / f"{test}.par", "-o", curr_dir / "out.c", "-emit-c"]
        out = subprocess.run(command, capture_output=True, text=True, check=True)

        is_correct = filecmp.cmp(curr_dir / f"{test}.c", curr_dir / "out.c", shallow=False)
        if not is_correct:
            print(f"test: {test}.par -> failed")
            return 1
        print(f"test: {test}.par -> passed")
        return 0
    except subprocess.CalledProcessError as e:
        if not should_fail:
            print(f"fail compiling {test}.par:\nstdout: \n{e.stdout}\nstderr: \n{e.stderr}")
            exit(1)
        else:
            print(f"test: {test}.par -> passed (expected to fail)")

def run_tests(): 
    global tests

    for test in tests:
        out = subprocess.run(["./parlan", f"{test}.par", "-o", "out.c", "-emit-c"], capture_output=True, text=True)
        if out.stdout != "" or out.stderr != "":
            print(f"failed, error while compiling {test}.par:\nstdout: \n{out.stdout}\nstderr: \n{out.stderr}")
            break
        is_correct = filecmp.cmp(f"{test}.c", "out.c", shallow=False)
        print(
            f"test: {test}.par -> passed" if is_correct else
            f"test: {test}.par -> failed"
        )

for test in tests:
    out = run_test(test[0],test[1])
    if out == 1:
        exit(1)