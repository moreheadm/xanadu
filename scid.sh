#!/bin/sh

#cargo build --release
RUST_BACKTRACE=1 "$(dirname $0)/target/release/xanadu" 2>/home/moreheadmax/xanadu.err

