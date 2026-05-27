# This is the main test file
#
# This file run the compiler over every single test (files with `.par` extension) on the directory,
# and then compares the output with the expected one (files with `.c` extensions)
#
# To run these tests, build the project to get a `parlan` executable, then move the executable into 
# this directory, finally run this file

# note: this is a very simple test suite, and will probably change soon to be more robust

import subprocess, filecmp

tests = [
    "glb_vars",
    "loops",
    "ifs",
    "functions",
    
]

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

run_tests()