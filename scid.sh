#!/bin/sh

#cargo build --release
SCRIPT_DIR="$(dirname $0)"
RUST_BACKTRACE=full "$SCRIPT_DIR/target/release/xanadu" 2>>"$SCRIPT_DIR"/xanadu.err

