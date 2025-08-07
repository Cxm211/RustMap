#!/usr/bin/env bash
# Analyze cognitive complexity for four RustMap projects

# List of project folders
PROJECTS=(bzip2_rs_gpt bzip2-c2rust/rust rosetta_code_gpt rossta-c2rust)


# Loop through each project
for proj in "${PROJECTS[@]}"; do
  echo " Analyzing $proj"

  rust-code-analysis-cli \
    -m \
    -p "$proj" \
    --pr \
    -O json \
    --output results/

  echo " JSON saved to results/${proj}"
  echo
done
