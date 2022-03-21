import subprocess
import os
from typing import Optional

COMPILER = "target/debug/ghuloum.exe"
DRIVER = "build/driver"


def run_program(source: str) -> Optional[str]:
    """
    Takes the argument `source` and feeds it to the compiler, and if the
    compilation succeeds, runs the driver program
    """
    # enable terminal colors on Windows
    if os.name == "nt":
        os.system("color")

    script_name = os.path.split(__file__)[1]

    compiler_proc = subprocess.run(
        [COMPILER, "--src", source], capture_output=True, text=True
    )

    if compiler_proc.returncode != 0:
        print(("[%s] compilation failed with message:") % script_name)
        print(compiler_proc.stdout)
        return None

    make_proc = subprocess.run(
        ["make"], capture_output=True, text=True
    )

    if make_proc.returncode != 0:
        print(("[%s] makefile failed with message:") % script_name)
        print(make_proc.stdout)
        return None

    driver_proc = subprocess.run([DRIVER], capture_output=True, text=True)
    driver_output = driver_proc.stdout

    if driver_proc.returncode != 0:
        print(("[%s] driver failed with message:") % script_name)
        print(driver_proc.stdout)
        return None

    return driver_output.strip()
