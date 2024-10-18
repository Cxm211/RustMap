To further verify the unsafety categorization, please check the 
#  Unsafety Analysis for bzip2-rustmap-gpt, which-rustmap-gpt and rossta-rustmap-gpt RQ2


for Figure 22 Rosseta Code, Figure 23 GNU which and Figure 25 Bzip2 unsafety categorization
- for C2Rust: we use [C2Rust](https://github.com/immunant/c2rust)
- for CRUSTS : we use [In Rust We Trust – A Transpiler from Unsafe C to Safer Rust](https://ieeexplore.ieee.org/document/9793767) from [CRustS - Transpiling Unsafe C code to Safer Rust](https://github.com/yijunyu/crusts)
- for Laertes: we use [aliasing-limit-23](10.5281/zenodo.7714175) from *[Aliasing Limits on Translating C to Safe Rust](https://dl.acm.org/doi/abs/10.1145/3586046)* 
- for rustmap: we use GPT-4 to generate result, which located in feasibility folder to generate the result
- for unsafe-counter: We will use [unsafe-counter](10.5281/zenodo.5442253) from [Translating C to safer Rust](https://dl.acm.org/doi/abs/10.1145/3485498)



 In order to execute the instructions below, we recommend to the unsafe-counter docker image use [zenodo doi](10.5281/zenodo.10421039)  


the core step is below:
```bash
 cargo run --release --target=x86_64-unknown-linux-gnu --bin unsafe-counter -- project 
```


After running the below commands, we mainly focus on two rows, each generate the data requires in figure 22, 23, and 25
```bash
Benchmark,Statistic,ReadFromUnion,MutGlobalAccess,InlineAsm,ExternCall,RawPtrDeref,UnsafeCast,Alloc
(unknown),Occurrences, ........,........,........,........,........,........
```

 ###  5.0.1. Figure 22 which GNU unsafety categorization
```bash
# rosseta-c2rust
# rosseta-crust
# rosseta-laertes

for project in "rossta-crust" "rossta-c2rust" "rossta-laertes"; do

  # Create or empty the log file for each project
  log_file="/root/lab/laertes/${project}/t_rosta.log"
  > $log_file

  # Iterate through all the folders under /root/lab/laertes/test-inputs/ for each project
  for dir in /root/lab/laertes/test-inputs/${project}/*; do
    if [ -d "$dir" ]; then  # Ensure this is a directory
      # Check if src/main.rs file exists
      if [ -f "$dir/src/main.rs" ]; then
        echo "----" >> $log_file
        echo "Processing: $dir/src/main.rs" >> $log_file
        
        # Enter the /root/lab/unsafe-counter directory
        pushd /root/lab/unsafe-counter > /dev/null
        # Run the cargo command and append the output to the log file
        cargo run --release --target=x86_64-unknown-linux-gnu --bin unsafe-counter -- "$dir/src/main.rs" >> $log_file 2>&1
        # Return to the previous directory (if needed, uncomment the next line)
        # popd > /dev/null
        
      else
        echo "Warning: $dir/src/main.rs not found." >> $log_file
      fi
    fi
  done

done


# manual categorization for rosseta-rustmap

```


###  5.0.2. Figure 23 which GNU unsafety categorization
```bash
# which-c2rust
cargo run --release --bin unsafe-counter -- ../laertes/test-inputs/bzip2-c2rust/rust/c2rust-lib.rs 

# which-crusts
cargo run --release --bin unsafe-counter -- ../laertes/test-inputs/bzip2-crust/rust/lib.rs 

# which-laertes
cargo run --release --bin unsafe-counter -- ../laertes/test-inputs/bzip2-laertes/rust/c2rust-lib.rs 

# manual categorization for which-rustmap
```


###  5.0.3. Figure 25 Bzip2 unsafety categorization
```bash

# bzip2-c2rust
cargo run --release --bin unsafe-counter -- ../laertes/test-inputs/bzip2-c2rust/rust/c2rust-lib.rs 

# bzip2-crusts
cargo run --release --bin unsafe-counter -- ../laertes/test-inputs/bzip2-crust/rust/lib.rs 

# bzip2-laertes
cargo run --release --bin unsafe-counter -- ../laertes/test-inputs/bzip2-laertes/rust/c2rust-lib.rs 

# manual categorization for bzip2-rustmap
```
