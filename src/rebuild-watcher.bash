#!/bin/bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
SOURCE_DIR="$SCRIPT_DIR"
BUILD_SCRIPT="$SCRIPT_DIR/example_builder.py"

# THE_COMMAND=("$BUILD_SCRIPT")
# "${THE_COMMAND[@]}"


# fswatch -o -1 0.1 -r "$SOURCE_DIR" "$BUILD_SCRIPT" | xargs -I{} "$BUILD_SCRIPT"
fswatch -o -1 0.1 -r . | xargs -I{} "$BUILD_SCRIPT"


