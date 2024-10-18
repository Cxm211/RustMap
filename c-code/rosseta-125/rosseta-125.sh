#!/bin/bash

# Define the log file
LOGFILE="rosseta_125_run_C_log.txt"

# Clear the log file if it already exists
> "$LOGFILE"

# Loop through each subdirectory
for dir in */; do
    # Check if main.c exists in the directory
    if [[ -f "${dir}main.c" ]]; then
        echo "Running main.c in $dir" | tee -a "$LOGFILE"
        
        # Navigate into the directory
        cd "$dir"
        
        # Compile main.c and check for errors
        gcc main.c -o main
        if [ $? -eq 0 ]; then
            # Run the compiled program and log the output
            ./main | tee -a "../$LOGFILE"
        else
            echo "Compilation failed in $dir" | tee -a "../$LOGFILE"
        fi
        
        # Navigate back to the parent directory
        cd ..
    else
        echo "No main.c found in $dir" | tee -a "$LOGFILE"
    fi
done

echo "Script execution completed." | tee -a "$LOGFILE"
