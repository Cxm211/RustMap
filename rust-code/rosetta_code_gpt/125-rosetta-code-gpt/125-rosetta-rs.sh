#!/bin/bash

# Define the log file
LOGFILE="rust_run_log.txt"

# Clear the log file if it already exists
> "$LOGFILE"

# Loop through each Rust file in the current directory
for file in *.rs; do
    # Check if the file exists
    if [[ -f "$file" ]]; then
        echo "Compiling and running $file" | tee -a "$LOGFILE"
        
        # Compile the Rust file and check for errors
        rustc "$file"
        if [ $? -eq 0 ]; then
            # Extract the filename without extension for execution
            filename="${file%.*}"
            
            # Run the compiled program and log the output
            "./$filename" | tee -a "$LOGFILE"
            
            # Remove the compiled binary to clean up
            rm "./$filename"
        else
            echo "Compilation failed for $file" | tee -a "$LOGFILE"
        fi
    else
        echo "$file not found" | tee -a "$LOGFILE"
    fi
done

echo "Script execution completed." | tee -a "$LOGFILE"
