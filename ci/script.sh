#!/usr/bin/env bash

set -eux

case "$JOB" in
    "test")
        cargo install -f cargo-readme
        cargo test
        ;;
    "bench")
        cargo bench
        ;;
    *)
        echo "Unknown \$JOB = '$JOB'"
        exit 1
esac
