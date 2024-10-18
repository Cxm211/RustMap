
## rosseta-c2rust
## rosseta-crust
## rosseta-laertes

for project in "rossta-crust" "rossta-c2rust" "rossta-laertes"; do

  ## Create or empty the log file for each project
  log_file="/root/lab/laertes/${project}/t_rosta.log"
  > $log_file

  ## Iterate through all the folders under /root/lab/laertes/test-inputs/ for each project
  for dir in /root/lab/laertes/test-inputs/${project}/*; do
    if [ -d "$dir" ]; then  ## Ensure this is a directory
      ## Check if src/main.rs file exists
      if [ -f "$dir/src/main.rs" ]; then
        echo "----" >> $log_file
        echo "Processing: $dir/src/main.rs" >> $log_file
        
        ## Enter the /root/lab/unsafe-counter directory
        pushd /root/lab/unsafe-counter > /dev/null
        ## Run the cargo command and append the output to the log file
        cargo run --release --target=x86_64-unknown-linux-gnu --bin unsafe-counter -- "$dir/src/main.rs" >> $log_file 2>&1
        ## Return to the previous directory (if needed, uncomment the next line)
        ## popd > /dev/null
        
      else
        echo "Warning: $dir/src/main.rs not found." >> $log_file
      fi
    fi
  done

done
