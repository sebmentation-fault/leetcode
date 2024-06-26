#!/bin/bash

# Ensure the problems directory exists
mkdir -p problems

# Find the highest existing problem number
last_problem=$(find problems/ -type d -regex ".*/problem_[0-9]+" | sort -V | tail -n 1)
if [ -z "$last_problem" ]; then
    # No problems found, starting with problem 1
    new_num=1
else
    last_num=$(basename $last_problem | sed 's/problem_//')
    new_num=$((last_num + 1))
fi

# Create a new problem directory and src directory
new_problem_dir="problems/problem_$new_num"
mkdir -p "$new_problem_dir/src"

# Get the current year
current_year=$(date +"%Y")

# Create a new Cargo.toml file for the problem, including the current year
cat > "$new_problem_dir/Cargo.toml" <<EOF
[package]
name = "problem_$new_num"
version = "0.1.0"
edition = "2018"
description = "Solution for LeetCode Problem $new_num, initially attempted in $current_year."
license = "MIT"
EOF

# Create main.rs with predefined content
cat > "$new_problem_dir/src/main.rs" <<EOF
mod solution;
use solution::Solution;

fn main() {
}
EOF

# Create solution.rs with predefined content
cat > "$new_problem_dir/src/solution.rs" <<EOF
pub struct Solution;

impl Solution {
    pub fn solution() {
    }
}
EOF

# Update the root Cargo.toml to include the new problem
if ! grep -q "problems/problem_$new_num" Cargo.toml; then
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # For macOS, which uses BSD sed
        sed -i '' "/members = \[/a\\
\ \ \ \ \"problems/problem_$new_num\",
" Cargo.toml
    else
        # For Linux and other systems using GNU sed
        sed -i "/members = \[/a \ \ \ \ \"problems/problem_$new_num\"," Cargo.toml
    fi
fi

echo "New problem 'problem_$new_num' has been created and added to the workspace."

# Ask the user if they want to open the new problem now
read -p "Do you want to open the new problem now? [Y/n] " -n 1 -r
echo    # Move to a new line

if [[ $REPLY =~ ^[Yy]$ ]] || [[ -z $REPLY ]]; then
    # Check if the EDITOR environment variable is set and not empty
    if [ -z "$EDITOR" ]; then
        echo "The EDITOR environment variable is not set. Please set it to your preferred editor."
        exit 1
    else
        if [ -z "$TERMINAL" ]; then
            echo "The TERMINAL environment variable is not set. Please set it to your preferred terminal."
            exit 1
        else
            # Use the editor defined in the EDITOR environment variable to open the files
            $TERMINAL -e $EDITOR "$new_problem_dir/src/main.rs" "$new_problem_dir/src/solution.rs" &
        fi
    fi
elif [[ $REPLY =~ ^[Nn]$ ]]; then
    # Exit if the user inputs 'n'
    exit 0
fi
