#!/bin/bash

# Define the paths for the source folder and the log folder
src_folder="/root/rustmap/feasibility_study/rossta_code_gpt/failed_11_cases"
log_folder="/root/rustmap/feasibility_study/rossta_code_gpt/rust_errors"

# Ensure the log folder exists
mkdir -p "$log_folder"

# Iterate through all .rs files in the source folder
for file in "$src_folder"/*.rs; do
    # Extract the filename (without the path)
    filename=$(basename "$file")
    
    # Construct the path for the log file
    log_file="$log_folder/${filename%.rs}.log"

    # Compile the Rust file and redirect the output to the log file
    rustc "$file" &> "$log_file"
done

# Initialize the combined log file
combined_log="$log_folder/combined_errors.log"
> "$combined_log"

# Append each log file's content to the combined log file
for log in "$log_folder"/*.log; do
    echo "$(basename "$log" .log)" >> "$combined_log"
    echo >> "$combined_log"
    cat "$log" >> "$combined_log"
    echo >> "$combined_log"
done

# Count the number of log files
log_count=$(find "$log_folder" -type f -name "*.log" | wc -l)

# Output the total number of log files
echo "Total number of log files: $log_count"
