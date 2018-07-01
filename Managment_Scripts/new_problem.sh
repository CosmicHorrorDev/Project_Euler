#!/usr/bin/bash

NEW_PROBLEM=~/Programming/Repos/project_euler/Problems/Problem_$1

# echo $NEW_PROBLEM
mkdir -p $NEW_PROBLEM/Python

cargo new --lib --vcs none "$NEW_PROBLEM/Rust/problem_$1"
cp ~/.config/custom_scripts/template.py "$NEW_PROBLEM/Python/problem_$1.py"

