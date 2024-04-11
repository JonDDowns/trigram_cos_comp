#!/bin/bash
nrun=5

echo "Rust version 1"
sudo sync; echo 1 > /proc/sys/vm/drop_caches
perf stat -r$nrun -o ./out/rust_v1.txt cargo run -r -- ./ext/example2.csv v1

echo "Rust version 2"
sudo sync; echo 1 > /proc/sys/vm/drop_caches
perf stat -r$nrun -o ./out/rust_v2.txt cargo run -r -- ./ext/example2.csv v2

echo "Rust final version"
sudo sync; echo 1 > /proc/sys/vm/drop_caches
perf stat -r$nrun -o ./out/rust_vfin.txt cargo run -r -- ./ext/example2.csv

echo "R Baseline"
perf stat -r$nrun -o ./out/r_baseline.txt Rscript ./ext/baseline.R
