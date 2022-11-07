#!/bin/bash


SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
SOURCE_DIR="$SCRIPT_DIR/src"
BUILD_SCRIPT="$SCRIPT_DIR/build.py"

# THE_COMMAND=("$BUILD_SCRIPT")
# "${THE_COMMAND[@]}"


fswatch -o -1 0.1 -r "$SOURCE_DIR" "$BUILD_SCRIPT" | xargs -I{} "$BUILD_SCRIPT"


