#! /usr/bin/env python3

import subprocess
import select
import sys
import shutil
import os


def run(command):
    """
    Run a subprocess and print stdout and stderr in real-time

    Args:
        command (list): Command and arguments as a list (e.g., ["ls", "-la"])
    """
    try:
        print(f"Running '{command}'")
        process = subprocess.Popen(
            command.split(" "),
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )

        # Use select to read from stdout and stderr without blocking
        while True:
            reads = [process.stdout.fileno(), process.stderr.fileno()]
            ret = select.select(reads, [], [])

            for fd in ret[0]:
                if fd == process.stdout.fileno():
                    line = process.stdout.readline()
                    if line:
                        sys.stdout.buffer.write(line)
                        sys.stdout.buffer.flush()
                if fd == process.stderr.fileno():
                    line = process.stderr.readline()
                    if line:
                        sys.stderr.buffer.write(line)
                        sys.stderr.buffer.flush()

            if process.poll() is not None:
                break

        return process.wait()

    except Exception as e:
        print(f"Error running subprocess: {e}")


def edit_cargo_toml(*, wasm: bool):
    # read Cargo.toml
    with open("Cargo.toml", "r") as file:
        cargo_toml = file.read()

    replace_strs = [
        'surf = { version = "2.3.2" }',
        'surf = { version = "2.3.2", default-features = false, features = ["wasm-client"] }',
    ]

    if not wasm:
        replace_strs.reverse()

    cargo_toml = cargo_toml.replace(replace_strs[0], replace_strs[1])

    # write back to Cargo.toml
    with open("Cargo.toml", "w") as file:
        file.write(cargo_toml)


try:
    build_mode = "rust"

    if len(sys.argv) > 1:
        build_mode = sys.argv[1]

    if build_mode == "wasm":
        edit_cargo_toml(wasm=True)

    if build_mode == "wasm":
        run(
            "wasm-pack build --target web --out-dir ./consumers/web/pkg -- --color always"
        )

        # Remove the generated .gitignore file from the pkg directory
        os.remove("consumers/web/pkg/.gitignore")
    else:
        run("cargo --color always build --release")

    if build_mode == "py":
        run(
            "cargo --color always run --bin uniffi-bindgen generate --library target/release/libplayground.dylib --language python --out-dir consumers/python"
        )

        extension = None

        # Determine what the extension of the library is
        extensions = ("dylib", "so", "dll")
        for ext in extensions:
            if os.path.exists(f"target/release/libplayground.{ext}"):
                extension = ext
                break

        copy_args = [
            f"target/release/libplayground.{extension}",
            f"consumers/python/libplayground.{extension}",
        ]

        # Copy the library file using Python's shutil
        shutil.copy2(*copy_args)

        print(f"Copied {copy_args[0]} to {copy_args[1]}")


finally:
    # Ensure we always reset the Cargo.toml to the original state
    edit_cargo_toml(wasm=False)
