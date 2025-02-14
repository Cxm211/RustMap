# Key Code and Dataset Catalogue 
## Code
- [C code for bzip2](c-code/bzip2-1.0.8)
- [C code for bzip2 after handling macros](c-code/bzip2-1.0.8-macro)
- [Preprocessed C code for bzip2 after handling macros](c-code/bzip2-1.0.8-i)
- [C code for rosseta](c-code/rosseta-125)
- [RustMap Generated bzip2 ](rust-code/bzip2_rs_gpt)
- [RustMap Generated rosseta code](rust-code/rosetta_code_gpt)
- [C2Rust Generated bzip2](rust-code/bzip2-c2rust)
## Dataset
- [bzip2 coverage test](key_datasets/merged_coverage_test_bzip2.csv)
- [rosseta coverage test](key_datasets/merged_coverage_test_bzip2.csv)
- [bzip2 complexity consolidated data](key_datasets/bzip2_complexity.csv)
- [rosseta complexity individual data](key_datasets/rosseta-readability-stats.csv)
- [rosseta complexity consolidated data data](key_datasets/rosseta-consolidated-stats.csv)  

## Example of Macro Handling
- [Sample Translations of Numerical Macros](README_example.md#sample-translations-of-numerical-macros)
- [Sample Translations of Complex Macros](README_example.md#sample-translations-of-complex-macros)
- [Sample Translations of Unhandled Macros](README_example.md#sample-translations-of-unhandled-macros)

## prompt templates
- [Prompt to Translate Macro in C](prompt-templates/macro-translation.txt)
- [Prompt to Generate Rust from C](prompt-templates/prompt_for_LLM_to_generate_code.txt)
- [Prompt to Solve Inconsistency Error](prompt-templates/prompt_to_solve_inconsistency_error.txt)


# Detailed README.md
- [Key Code and Dataset Catalogue](#key-code-and-dataset-catalogue)
  - [Code](#code)
  - [Dataset](#dataset)
  - [prompt templates](#prompt-templates)
- [Detailed README.md](#detailed-readmemd)
- [1. Introducation](#1-introducation)
  - [1.1. Version Introduction](#11-version-introduction)
- [2. Scaffolding Boilerplate Generation (example of bzip2)](#2-scaffolding-boilerplate-generation-example-of-bzip2)
  - [2.1. Handle Macros](#21-handle-macros)
  - [2.2. Dynamically applied Runtime Function Call Graph](#22-dynamically-applied-runtime-function-call-graph)
  - [2.3. Generate Function Static Call Graph](#23-generate-function-static-call-graph)
    - [2.3.1. Use Static cflow](#231-use-static-cflow)
    - [2.3.2. Generate RustMap Scaffolding](#232-generate-rustmap-scaffolding)
- [3. Prompts](#3-prompts)
  - [3.1. Prompt for handling macros](#31-prompt-for-handling-macros)
  - [3.2. Prompt for directly applying LLM to translate](#32-prompt-for-directly-applying-llm-to-translate)
  - [3.3. Prompt to Resolve Compilation Error](#33-prompt-to-resolve-compilation-error)
  - [3.4. Prompt to resolve Inconsistency error.](#34-prompt-to-resolve-inconsistency-error)
- [4. Functional Test](#4-functional-test)
  - [4.1. Functional Test of bzip2](#41-functional-test-of-bzip2)
    - [4.1.1. Steps to build translated bzip2](#411-steps-to-build-translated-bzip2)
    - [4.1.2. Functional Test](#412-functional-test)
    - [4.1.3. Verification of RustMap bzip2 compress by uncompressing `.bz2`](#413-verification-of-rustmap-bzip2-compress-by-uncompressing-bz2)
  - [4.2. Functional Test of Rosetta Code](#42-functional-test-of-rosetta-code)
    - [4.2.1 Batch Execution of Rosetta Code](#421-batch-execution-of-rosetta-code)
    - [4.1.2. Verification of Rosetta running result compared to original C](#412-verification-of-rosetta-running-result-compared-to-original-c)
- [5. Count of Macros Definition and Usage](#5-count-of-macros-definition-and-usage)
- [6. Coverage Test](#6-coverage-test)
  - [6.1. Table 1.1: Calculating Coverage Test Ratio of Custom Test Case to bzip2 Test Suite](#61-table-11-calculating-coverage-test-ratio-of-custom-test-case-to-bzip2-test-suite)
    - [6.1.1. Measuring Coverage for bzip2's Original Test Suite](#611-measuring-coverage-for-bzip2s-original-test-suite)
    - [6.1.2. Measuring Coverage for Custom Test Suite: bzip2 Rust Compress and Decompress Functions](#612-measuring-coverage-for-custom-test-suite-bzip2-rust-compress-and-decompress-functions)
    - [6.1.3. Measuring Coverage for Custom Test Suite: bzip2 Rust Decompress Function](#613-measuring-coverage-for-custom-test-suite-bzip2-rust-decompress-function)
    - [6.1.4. Combined Coverage Results for bzip2](#614-combined-coverage-results-for-bzip2)
  - [6.2. Table 1.2 Rosetta Coverage Test Generation](#62-table-12-rosetta-coverage-test-generation)
- [7. Macro Handling](#7-macro-handling)

# 1. Introducation
The purpose of this artifact is to reproduce the experimental results we achieved in the paper "RustMap: Towards Project-Scale C to Rust Migration via LLM."

We will demonstrate how to perform the Coverage Test, Complexity Test, Functionality Test of bzip2 and 125 Rosetta code, and the Unsafety Test.



## 1.1. Version Introduction
- [bzip2-1.0.8](https://gitlab.com/bzip2/bzip2/-/tree/bzip2-1.0.8?ref_type=tags)
- [rosetta code](https://rosettacode.org/)

> The following operational instructions can be executed in the Docker image https://hub.docker.com/r/cxm211/rustmap-new
# 2.  Scaffolding Boilerplate Generation (example of bzip2)
As we write in paper, we will do Scaffolding pre-processing before applying LLM to translate the C code.

## 2.1. Handle Macros
We will handle the macros in the C code first. The detailed process is in paper, here we will show the [prompt template to handle the macros](prompt-templates/macro-translation.txt). 

The c code after handling macros is in `c-code/bzip2-1.0.8-macro` and the preprocessed c code is in `c-code/bzip2-1.0.8-i`. The c code after handling macros will be used to generate the dynamic call graph. We will use the preprocessed c code to generate the static call graph and translate the C code to Rust code.


## 2.2. Dynamically applied Runtime Function Call Graph
As we write in paper, we try to run original C program and observe the calling relationship.


Add the -pg option to CFLAGS and LDFLAGS: This is used to enable Gprof profiling during compilation and linking.
We have added `-pg` flag to Makefile in `./bzip2-1.0.8-macro/Makefile` this will help to generate 

1. in Makefile add -pg, this is used to enable Gprof profiling during compilation and linking.
```bash
CFLAGS=-Wall -Winline -O2 -g $(BIGFILES) -pg
LDFLAGS+= -pg
```
make sure the flag `-pg` in CFLAGS and LDFLAGS in `c-code/bzip2-1.0.8-macro/Makefile`

2. Run the program to generate the `gmon.out` file
```bash
cd c-code/bzip2-1.0.8-macro
make clean
make
./bzip2 -1 < testcases/sample1.ref > testcases/sample1.rb2
```

3. Generate the analysis report gprof.out using Gprof
```bash
gprof bzip2 gmon.out > gprof.out
```
4. Generate the Dynamic call graph `call_graph.png` using Gprof2Dot and dot:
```bash

gprof2dot -f prof gprof.out | dot -Tpng -o dynamic_call_graph.png

```

## 2.3. Generate Function Static Call Graph 


### 2.3.1. Use Static cflow

When using the cflow tool for a C project, it's generally recommended to have only one main function in the project. cflow is designed to analyze function call relationships in C programs and generates a call graph. If there are multiple main functions, cflow might face difficulties, as the main function typically serves as the entry point of a program. For projects with multiple main functions, like those containing independent sub-projects, you might need to run cflow separately for each part or adjust the project structure for effective analysis. In summary, having a single main function is the best practice for using cflow, unless you have specific needs and strategies to handle multiple instances. 
 
```bash
python3 cflow_generation.py /root/rustmap/c-code/bzip2-1.0.8-i
```
You can find 'c-code/bzip2-1.0.8-i/scg.dot' and 'c-code/bzip2-1.0.8-i/scg.svg' .


### 2.3.2. Generate RustMap Scaffolding
```bash
python3 extract.py /root/rustmap/c-code/bzip2-1.0.8-i
```
The Scaffolding is generated and saved to [scaffolding.json](c-code/bzip2-1.0.8-i/scaffolding.json)



# 3. Prompts
## 3.1. Prompt for handling macros
```text
You will be provided with C code that contains some simple and complex macros. Your task is to convert these macros into non-macro equivalent code, ensuring the functionality and logic remain the same. You must replace the macro definitions with standard C code (such as inline functions or constant variables), but the way they are called or invoked should not change in the code. All code that is not part of a macro should remain unchanged. Generate compliable and complete code. 
Requirements:
1. For simple macros (e.g., constant values), replace them with appropriate const variables with same names.
2. For macros with arguments (complex macros), replace them with functions while ensuring that return values, function name and functionality remain unchanged. If the parameter types of the converted function are unclear, refer to the later code for clarification. If the type remains unclear, do not convert the macro and highlight the uncertainty.
3. If a macro is defined or undefined within a conditional compilation directive (e.g., #if, #ifdef, #ifndef), it should remain unchanged for all statements.
4. Be mindful of any operator precedence or edge cases that could occur with macro expansions, and ensure the logic is preserved without altering the original functionality.
5. If a macro has been replaced and is followed by an #undef directive, remove the #undef statement.
6. If the macro applies to multiple data types, create multiple versions of the function for each applicable data type (using function overloading or type-specific function names as needed in C).
7. If there are any uncertainties about how the macro should be converted or any potential ambiguity, ask for clarification before making the conversion.
8. After converting the macros, output the entire modified code with both the converted macros and the unchanged non-macro code.
9. If any undefined function or macros appears in future conversations, refer to the conversation history for context.

```

## 3.2. Prompt for directly applying LLM to translate
```bash

If the C code references other vital functions or structures, ask mefirst and wait for my provided input.(ASK ME first)Convert the given code to idiomatic Rust, keeping its function. Useminimal unsafe traits. Don't translate unknown variables or functions, and avoid assumptions.(ASK ME frst)

(1) If a variable inside the function is modified, add the mut specifier.
(2) Distinguish between mutable and immutable references by stor-ing intermediate values.
(3) If necessary, add lifetime annotations.
(4) Add clear comments for all numeric types and pay attention to type conversions, especially between usize and others.
(5) Be cautious of potential out-of-bound errors in the C code.
(6)Use the Rust standard library as much as possible.
(7)When performing arithmetic operations, be mindful of potentialoverflow or underflow.


(...Here is C Code to be translated...)


I must reiterate: if you encounter unfamiliar variables or functionsduring translation, you must ask me and wait for my provided inputbefore translating.(ASK ME FIRST)


```
## 3.3. Prompt to Resolve Compilation Error
Normally we will after our test generate compilation error, we will directly put the compilation error into ChatGPT-4
See the examples of compilation errors in the directory: `prompt-templates/compilation-errors` We have listed some of compilation from our RustMap Translation of Rosetta Code



## 3.4. Prompt to resolve Inconsistency error. 
 please generate Rust code fragment here to have
  consistent states as the C code above
```bash
// C code fragment with its before- and after-states:
{
    (before-state of C)
    (...Here is the C code fragment that have
        caused errors in the translated Rust...)
    (after-state of C)
}

// Rust code generation:
{
    (before-state in Rust)
    /** please generate Rust code fragment here to have
        consistent states as the C code above
      */
}
```
<!-- Please check the detailed Example in `./prompt-templates/inconsistency-solution` -->

# 4. Functional Test

## 4.1. Functional Test of bzip2
<!-- > The following operational instructions can be executed in the Docker image https://hub.docker.com/repository/docker/cxm211/rustmap/general
>
>  -->
### 4.1.1. Steps to build translated bzip2
```bash
cd /root/rustmap/rust-code/bzip2_rs_gpt
cargo build
```
It will generate executable binary in `/root/rustmap/rust-code/bzip2_rs_gpt/target/debug/bzip2_rs_gpt` this path


<!-- ### 4.1.2. test cases generations bzip2
```bash
cd /root/rustmap/rust-code/bzip2_tests
python3 random-test-case-generation.py
```
This step will generate five small-scale text files: `random_1_chars.txt`, `random_10_chars.txt`, `random_100_chars.txt`, `random_1000_chars.txt`, `random_5000_chars.txt`
Then we will use bzip2-rust binary to test the results. -->


### 4.1.2. Functional Test
```bash
cd /root/rustmap/rust-code/bzip2_testcases/

# compress test cases and record its processing time
{ 
  echo "Running random_1_chars.txt"; time /root/rustmap/rust-code/bzip2_rs_gpt/target/debug/bzip2_rs_gpt random_1_chars.txt;
  echo "Running random_10_chars.txt"; time /root/rustmap/rust-code/bzip2_rs_gpt/target/debug/bzip2_rs_gpt random_10_chars.txt;
  echo "Running random_100_chars.txt"; time /root/rustmap/rust-code/bzip2_rs_gpt/target/debug/bzip2_rs_gpt random_100_chars.txt;
  echo "Running random_1000_chars.txt"; time /root/rustmap/rust-code/bzip2_rs_gpt/target/debug/bzip2_rs_gpt random_1000_chars.txt;
  echo "Running random_5000_chars.txt"; time /root/rustmap/rust-code/bzip2_rs_gpt/target/debug/bzip2_rs_gpt random_5000_chars.txt;
} 2>&1 | tee output.log


```

you may will the generation time in here: /root/rustmap/rust-code/bzip2_testcases/timings.txt


### 4.1.3. Verification of RustMap bzip2 compress by uncompressing `.bz2`
```bash
cd /root/rustmap/rust-code/bzip2_testcases/
bzip2recover random_1_chars.txt.bz2
bzip2 -d rec00001random_1_chars.txt.bz2

bzip2recover random_10_chars.txt.bz2
bzip2 -d rec00001random_10_chars.txt.bz2

bzip2recover random_100_chars.txt.bz2
bzip2 -d rec00001random_100_chars.txt.bz2

bzip2recover random_1000_chars.txt.bz2
bzip2 -d rec00001random_1000_chars.txt.bz2

bzip2recover random_5000_chars.txt.bz2
bzip2 -d rec00001random_5000_chars.txt.bz2


# Perform diff comparison
diff random_1_chars.txt rec00001random_1_chars.txt > diff_random_1_chars.txt
diff random_10_chars.txt rec00001random_10_chars.txt > diff_random_10_chars.txt
diff random_100_chars.txt rec00001random_100_chars.txt > diff_random_100_chars.txt
diff random_1000_chars.txt rec00001random_1000_chars.txt > diff_random_1000_chars.txt
diff random_5000_chars.txt rec00001random_5000_chars.txt > diff_random_5000_chars.txt


```


## 4.2. Functional Test of Rosetta Code
### 4.2.1 Batch Execution of Rosetta Code
Since there are attached testcases in original Roseta Code, we will directly execute the translated Roseta Code and Compare the result with original C Roseta Code
```bash
cd /root/rustmap/c-code/rosseta-125/
chmod +x ./rosetta-125.sh
bash ./rosetta-125.sh > original_rosetta_result.log

cd /root/rustmap/rust-code/rosetta_code_gpt/125-rosetta-code-gpt/
chmod +x ./125-rosetta-rs.sh
bash ./125-rosetta-rs.sh > rustmap_rosetta_result.log
```

### 4.2.2. Verification of Rosetta running result compared to original C
```bash
diff /root/rustmap/c-code/rosseta-125/original_rosetta_result.log /root/rustmap/rust-code/rosetta_code_gpt/125-rosetta-code-gpt/rustmap_rosetta_result.log > diff_rosetta_result.log
```


# 5. Count of Macros Definition and Usage

> The following operational instructions can be executed in the Docker image https://hub.docker.com/repository/docker/cxm211/rustmap-new/general
> 
![](paper_pic/Macros-Counts.jpg)

We count the number of macros declaration and usage based on original bzip2 C, which located in `c-code/bzip2-1.0.8`



# 6. Coverage Test
We try to show the process to generate the Coverage Test Statistics in this section
![](./paper_pic/Coverage-Test-Result-bzip2-Rosetta.jpg)

> The following operational instructions can be executed in the Docker image https://hub.docker.com/repository/docker/cxm211/rustmap-new/general

## 6.1. Table 1.1: Calculating Coverage Test Ratio of Custom Test Case to bzip2 Test Suite
we have added `coverage` flags in `Makefile` under `c-code/bzip2-1.0.8

![](paper_pic/Table-coverage-bzip2.jpg)

### 6.1.1. Measuring Coverage for bzip2's Original Test Suite
```bash
cd /root/rustmap/c-code/bzip2-1.0.8
# this command generate 
make 
```
View the result in `/root/rustmap/c-code/bzip2-1.0.8/out/index.html` for Table 1 bzip2


### 6.1.2. Measuring Coverage for Custom Test Suite: bzip2 Rust Compress and Decompress Functions
```bash
make cleancoverage
./bzip2 -k testcases/compress_test.txt
lcov --capture --directory . --output-file compress_coverage.info
genhtml compress_coverage.info --output-directory compress_out
```
View the Result in `bzip2-1.0.8/compress_out/bzip2-1.0.8/index.html`

### 6.1.3. Measuring Coverage for Custom Test Suite: bzip2 Rust Decompress Function
```bash
make cleancoverage
./bzip2 -k -d testcases/decompress_test.txt.bz2
lcov --capture --directory . --output-file decompress_coverage.info
genhtml decompress_coverage.info --output-directory decompress_out
```
View the Result in `bzip2-1.0.8/decompress_out/bzip2-1.0.8/index.html`


### 6.1.4. Combined Coverage Results for bzip2
Once you combine the result, you use Original Test Suite to divide  
 combined get the result like below:
View the Combined  Result in `bzip2-1.0.8/compress_out/bzip2-1.0.8/index.html`



## 6.2. Table 1.2 Rosetta Coverage Test Generation
```bash
# This script iterates through all subdirectories in the current directory, compiles and runs the first C file found in each subdirectory, collects code coverage data, and finally generates an HTML coverage report.
bash /root/rustmap/c-code/rosetta-125/gcc-Rosetta-code.sh
```
View the result in `/root/rustmap/c-code/rosetta-125/coverage_report/index.html` for Table 1 Rosetta Code
![](paper_pic/Table-Coverage-Rosetta.jpg)

<!-- 
# 7. Cogntive Complexity Test

> The following operational instructions can be executed in the Docker image `rustmap-feasibility-study.tar.gz` available on [zenodo](https://zenodo.org/records/10433166). The operational code is present in both this repository and the Zenodo Docker image.


## 7.1. bzip2 Complexity Test
```bash
# Generate Cognitivie Complexity comparative study result for Rustmap and C2Rust for each folder
cargo run -- /root/rustmap/cognitive-complex-test/src/comparision/bzip2

cargo run -- /root/rustmap/cognitive-complex-test/src/comparision/blocksort

cargo run -- /root/rustmap/cognitive-complex-test/src/comparision/bzlib

cargo run -- /root/rustmap/cognitive-complex-test/src/comparision/compress

cargo run -- /root/rustmap/cognitive-complex-test/src/comparision/decompress

cargo run -- /root/rustmap/cognitive-complex-test/src/comparision/huffman
```
View the initial result in `/root/rustmap/cognitive-complex-test/result/bzip2-complexity-init-result.csv`


We merge the result as image below
![](cognitive-complex-test/result/bzip2-merged-result.pdf)



View Final Result after merge
![](cognitive-complex-test/result/bzip2-Complexity-Final-Result.pdf)


## 7.2. Rosetta Code Complexity Test
In `cognitive-complex-test/src`

1. Generate RustMap Rosetta Code Complexity Result
```bash
# Rename `main-Rosetta-gpt` and replace it as `main.rs`
cargo run -- /root/rustmap/rust-code/Rosetta_code_gpt

```

2. Generate C2Rust Rosetta Code Complexity Result
```bash
#  Rename `main-Rosetta-c2rust` and replace it as `main.rs`
cargo run --  /root/rustmap/unsafety-analysis-for-rust/test-inputs/Rosetta-c2rust-readability
```


View the initial result in `/root/rustmap/cognitive-complex-test/result/Rosetta-complexity-init-result.csv`


View Final Result after merge
![](paper_pic/Roseta-Code-Complexity.pdf)  




### 7.2.1. Drawing Violin Graph for both bzip2 and Rosetta code
![](./paper_pic/violin-diagram.jpg)
This `.ipynb` file reads two CSV files containing complexity score data for RustMap and C2Rust, generates violin plots for these data, and performs statistical comparisons using the Wilcoxon test and Cliff's Delta test.

The first `Book1.csv` contains complexity scores for bzip2 RustMap and C2Rust. The second `Book2.csv` contains complexity scores for Rosetta Code RustMap and C2Rust.

#### 7.2.1.1. How to Execute:

1. Ensure that Jupyter Notebook is installed. If not, install it using the following command:

```bash
pip install notebook
```


2. Open a terminal or command prompt and navigate to the directory containing the violin_plot.ipynb file. For example:

```bash
/root/rustmap/violin_plot.ipynb
```

3. Start Jupyter Notebook
```bash
jupyter notebook
```

4. The Jupyter Notebook interface will automatically open in your browser. Find and click on the `violin_plot.ipynb` file.

5. Run each code cell one by one (click on each cell and press Shift+Enter) to ensure all code executes correctly and generates the violin plots and statistical comparison results.







# 8. Unsafety Analysis for bzip2-rustmap-gpt and Rosetta-rustmap-gpt

The below operation is based on docker image *rustmap-unsafety-evaluation.tar.gz* in Zenodo.
The Code is of operation is both on README.md and Zenodo docker image.

- for C2Rust: we use [C2Rust](https://github.com/immunant/c2rust)
- for CRUSTS : we use [In Rust We Trust – A Transpiler from Unsafe C to Safer Rust](https://ieeexplore.ieee.org/document/9793767) from [CRustS - Transpiling Unsafe C code to Safer Rust](https://github.com/yijunyu/crusts)
- for Laertes: we use [aliasing-limit-23](10.5281/zenodo.7714175) from *[Aliasing Limits on Translating C to Safe Rust](https://dl.acm.org/doi/abs/10.1145/3586046)* 
- for unsafe-counter: We will use [unsafe-counter](https://zenodo.org/records/10433166) from [Translating C to safer Rust](https://dl.acm.org/doi/abs/10.1145/3485498)



 In order to execute the instructions below, we recommend to the unsafe-counter docker image use [zenodo doi](https://zenodo.org/records/10433166)  


the core step is below:
```bash
 cargo run --release --target=x86_64-unknown-linux-gnu --bin unsafe-counter -- project 
```


After running the below commands, we mainly focus on two rows, each generate the data requires in Unsafe Test
```bash
Benchmark,Statistic,ReadFromUnion,MutGlobalAccess,InlineAsm,ExternCall,RawPtrDeref,UnsafeCast,Alloc
(unknown),Occurrences, ........,........,........,........,........,........
```
See Table in ![](./paper_pic/Unsafety-Code.jpg)


View the shell script in `/root/rustmap/unsafety-analysis-for-rust/test-inputs/Rosetta-code-catgorization.sh`



###  8.0.1. Bzip2 unsafety categorization
```bash

# bzip2-c2rust
cargo run --release --bin unsafe-counter -- ../laertes/test-inputs/bzip2-c2rust/rust/c2rust-lib.rs 

# bzip2-crusts
cargo run --release --bin unsafe-counter -- ../laertes/test-inputs/bzip2-crust/rust/lib.rs 

# bzip2-laertes
cargo run --release --bin unsafe-counter -- ../laertes/test-inputs/bzip2-laertes/rust/c2rust-lib.rs 

# manual categorization for bzip2-rustmap
```



# 9. Code Rewrite Pattern Samples

## 9.1. Global Variable Lazy Static
We have demonstrated the origianl C code and its Rust rewrite in the directory `code_patterns/global_variables_lazy_static`

## 9.2. Pointer Aliasing without Endiness Concern 
See the code under `/root/rustmap-clone/code_patterns/pointer_aliasing/raw_pointer_rewrite` to illustrate both pointer aliasing without endiness concern and with endiness concern


## 9.3. Pointer Aliasing with Endiness Concern 
See the code under `/root/rustmap-clone/code_patterns/pointer_aliasing/endianness_concern` to illustrate both pointer aliasing without endiness concern and with endiness concern
The Rust code above addresses the endianness concern in the C code. Here is a detailed explanation of how the C and Rust codes implement this functionality:

To view the code in details in the folder `/root/rustmap-clone/code_patterns/pointer_aliasing/endianness_concern`

### 9.3.1. C Code Implementation:

In the C code, data is manipulated directly using memory pointers. The specific steps are as follows:

1. **Define Structures and Types**:
    ```c
    typedef unsigned char UChar;
    typedef unsigned int UInt32;
    typedef int Int32;

    typedef struct {
        UChar* zbits;
        UInt32* arr2;
        Int32 nblock;
    } EState;
    ```

2. **Set Pointer**:
    ```c
    s.zbits = (UChar*) (&((UChar*)s.arr2)[s.nblock]);
    ```
    This line performs the following operations:
    - Casts `s.arr2` to a `UChar` type pointer.
    - Offsets this pointer by `s.nblock` and gets the address.
    - Assigns this address to `s.zbits`, making `s.zbits` point to the `nblock`th byte of `s.arr2`.

### 9.3.2. Rust Code Implementation:

The Rust code implements the same logic as the C code but in a safer manner by handling memory and endianness explicitly:

1. **Define Structure and Function**:
    ```rust
    pub struct EState { 
        pub zbits: Vec<u8>,
        pub arr2: Vec<u32>,
        pub nblock: i32,
    }

    fn get_zbits(estate: &mut EState) {
        let nblock = estate.nblock as usize;
        let offset = nblock / 4;
        let remaining_bytes = estate.arr2.len() * 4 - nblock;
        estate.zbits.clear();
        estate.zbits.reserve(remaining_bytes);

        for &num in &estate.arr2[offset..] {
            let bytes = if cfg!(target_endian = "little") {
                num.to_le_bytes()
            } else {
                num.to_be_bytes()
            };

            let start_index = if estate.zbits.is_empty() { nblock % 4 } else { 0 };
            estate.zbits.extend_from_slice(&bytes[start_index..]);
        }
    }
    ```

2. **Handle Endianness and Update Array**:
    ```rust
    fn update_block_from_zbits(estate: &mut EState) {
        /* ... */
    }
    ```

### 9.3.3. Summary of Differences:

- **C Code**: Directly manipulates memory pointers and offsets, a low-level approach prone to errors but often used in hardware control or high-performance needs.
- **Rust Code**: Uses safe methods to handle memory and endianness, ensuring correct operation across different endianness systems and avoiding memory safety issues.

The comparison highlights that Rust code is more robust in handling memory safety and cross-platform compatibility.

### 9.3.4. Summary of Differences:

- **C Code**: Directly manipulates memory pointers and offsets, a low-level approach prone to errors but often used in hardware control or high-performance needs.
- **Rust Code**: Uses safe methods to handle memory and endianness, ensuring correct operation across different endianness systems and avoiding memory safety issues.

The comparison highlights that Rust code is more robust in handling memory safety and cross-platform compatibility.








## 9.4. Illustrate Necessity to rewrite Complex Macro and how to rewrite C `switch-case` to Rust `while match`
In this folder, you can see that the `.c` switch case has a fall-through state. The corresponding `c2rust-decompress.rs` uses complex match blocks to handle this fall-through, while our `rustmap-decompress.rs` uses a relatively simple and clear `while-loop` to handle it.

You can clearly see the code explosion in `decompress.i` and `c2rust-decompress.rs`, so finding the correct way to rewrite it is extremely important.

See the code under `/root/rustmap/code_patterns/complex_switch_fall_through_complex_macros` to illustrate  -->


# 7. Macro Handling
[This section](./README_example.md) provides examples of different types of macro translations, showcasing how various C macros are transformed into more structured and maintainable formats in both C and Rust. These examples illustrate the process of converting numerical macros, complex macros, and unhandled macros, ensuring better readability, type safety, and platform compatibility. By following these references, readers can explore detailed case studies and understand the rationale behind each transformation.
