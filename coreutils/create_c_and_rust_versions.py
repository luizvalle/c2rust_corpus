import argparse
import shutil
import traceback
import logging
import json
import sys
import os
import re

from typing import List, Dict, Sequence
from dataclasses import dataclass


@dataclass(frozen=True)
class Deps:
    """Dependencies for an object file."""
    source_file: str
    header_files: Sequence[str]


logging.basicConfig(
    stream=sys.stderr,  # Log messages to stderr
    level=logging.DEBUG,  # Set the logging level
    format="%(asctime)s - %(levelname)s - %(message)s"  # Format
)

_LOGGER = logging.getLogger("logger")

_MAKEFILE = """\
CFLAGS=-I./include -g
SRCS := \$(wildcard *.c)
OBJS := \$(SRCS:.c=.o)

.PHONY: all
all: clean $PROG_NAME

$PROG_NAME: \$(OBJS)
	gcc $^ -o \$@

%.o: %.c
	gcc \$(CFLAGS) -c $< -o \$@

.PHONY: clean
clean:
	rm -rf \$(OBJS) $PROG_NAME
"""


def _parse_arguments() -> Dict[str, str]:
    parser = argparse.ArgumentParser(description="Create C and Rust versions of"
                                     " a Coreutils program")

    parser.add_argument("--program_name",
                        type=str,
                        help="Name of the executable (i.e. echo, cat, etc.)",
          required=True)
    parser.add_argument("--corpus_dir",
                        type=str,
                        help="The parent directory that will contain the C and Rust versions",
                        default=os.getcwd())
    parser.add_argument("--coreutils_dir",
                        type=str,
                        help="The root directory for coreutils.",
                        required=True)

    args = parser.parse_args()

    return vars(args)


def _copy_files(filepaths: List[str], dest_dir: str) -> None:
    for filepath in filepaths:
        shutil.copy(filepath, dest_dir)


def _get_deps(filepath: str) -> Deps:
    directory = os.path.dirname(filepath)
    filename, _ = os.path.splitext(os.path.basename(filepath))
    deps_filepath = os.path.join(directory, ".deps", f"{filename}.Po")
    if (not os.path.exists(deps_filepath)
            or not os.path.isfile(deps_filepath)):
        _LOGGER.error(f"Could not find the dependency file: '{deps_filepath}'")
        sys.exit(1)
    with open(deps_filepath, 'r') as deps_file:
        lines = deps_file.readlines()
    target_line_idx = next(
        (i for i, line in enumerate(lines) if ':' in line), None)
    if target_line_idx is None:
        _LOGGER.error(f"Could not find a target in '{deps_filepath}'")
        sys.exit(1)
    dependencies = lines[target_line_idx].split(':')[1].split()
    dependencies = [dep.strip('\\') for dep in dependencies]
    dependencies = [dep for dep in dependencies if dep]
    for line in lines[target_line_idx + 1:]:
        if ':' in line:
            break  # Stop if a new target line is encountered
        dependencies.extend(line.split())
    main_source_file = os.path.join()


def main():
    args = _parse_arguments()
    _LOGGER.info(
        f"Received the following arguments:\n{json.dumps(args, indent=4)}")

    if not os.path.exists(args["coreutils_dir"]):
        _LOGGER.error(
            f"The coreutils directory does not exist: {args['coreutils_dir']}")
        sys.exit(1)

    out_prog_dir = os.path.join(args["corpus_dir"], args["program_name"])
    out_c_dir = os.path.join(out_prog_dir, "c")
    out_rust_dir = os.path.join(out_prog_dir, "rust")

    if os.path.exists(out_prog_dir):
        try:
            shutil.rmtree(out_prog_dir)
        except Exception as e:
            _LOGGER.error(
                f"Could not remove the directory:\n{traceback.format_exc()}")
            sys.exit(1)

    try:
        os.makedirs(out_c_dir)
        os.makedirs(out_rust_dir)
    except Exception as e:
        _LOGGER.error(
            f"Could not create the output directories:\n{traceback.format_exc()}")
        sys.exit(1)

    _LOGGER.info(f"Created the '{out_c_dir}' and '{out_rust_dir}' directories")

    c_makefile_path = os.path.join(out_c_dir, "Makefile")
    try:
        with open(c_makefile_path, 'w') as makefile:
            makefile.write(_MAKEFILE)
    except Exception as e:
        _LOGGER.error(
            f"Could not create the Makefile '{c_makefile_path}':\n"
            f"{traceback.format_exc()}")
        sys.exit(1)

    _LOGGER.info(f"Created the Makefile: '{c_makefile_path}'")

    main_source_filepath = os.path.join(
        args["coreutils_dir"], "src", f"{args['program_name']}.c")

    _get_deps(main_source_filepath)


if __name__ == "__main__":
    main()
