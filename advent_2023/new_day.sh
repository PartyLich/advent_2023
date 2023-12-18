#!/bin/bash
# Scaffold files for an AoC day
# doesn't handle any errors, not even remotely robust
# most of the ops seem idempotent though

# day number to create
day=${1?Usage: new_day <day number>}
year=2023

# create and switch to new branch
echo "creating branch: feat/day-$day"
git co -b feat/day-$day main

# create empty test input file
echo "creating test input file: input/$day-t.txt"
touch ./input/$day-t.txt

# fetch actual input
input_url="https://adventofcode.com/$year/day/$(($day + 0))/input"
input_path=./input/$day-1.txt
session_cookie=$(<adventofcode.session)
curl $input_url --cookie "session=$session_cookie" >$input_path

# create module
echo "creating module files: src/day_$day/"
path=./src/day_$day/
mkdir $path
mod_file="$path/mod.rs"
touch $mod_file
cat >>$mod_file <<EOF
//! Solutions to 2023 day $day problems
//! --  --

EOF

# add module to lib
echo "updating lib.rs"
echo "pub mod day_$day;" >>src/lib.rs
echo "formatting"
cargo-fmt

echo "done."
