- [version introduction](#version-introduction)
- [Part 1: Feasibility Study](#part-1-feasibility-study)
  - [bzip2](#bzip2)
  - [which](#which)
  - [rosetta](#rosetta)
- [Part 2: Scaffolding Boilerplate Generation](#part-2-scaffolding-boilerplate-generation)
  - [Step 0: Prepare](#step-0-prepare)
  - [Step 1: add save preprocessed file \*.i during `make`](#step-1-add-save-preprocessed-file-i-during-make)
    - [Step 1.1 remove directives from \*.i](#step-11-remove-directives-from-i)
    - [Step 1.2 rename `bzip2recover.i` to `bzip2recover.i.bk`](#step-12-rename-bzip2recoveri-to-bzip2recoveribk)
  - [Step 2: Generating C Tags: Command for Recursive Ctags with Custom Fields and Language Mapping](#step-2-generating-c-tags-command-for-recursive-ctags-with-custom-fields-and-language-mapping)
  - [Step 3: Use cflow](#step-3-use-cflow)
  - [Step 4: Generate RustMap Scaffolding](#step-4-generate-rustmap-scaffolding)
- [Part 3: Case Study: Why we need Strongly Connected Component Recursive dependency?](#part-3-case-study-why-we-need-strongly-connected-component-recursive-dependency)

# version introduction
- bzip2-1.0.8:  https://gitlab.com/bzip2/bzip2/-/tree/bzip2-1.0.8?ref_type=tags 
- which-2.21: http://ftp.gnu.org/gnu/which/which-2.21.tar.gz
- rosetta code: https://rosettacode.org/


# Part 1: Feasibility Study
## bzip2
Firstly project bzip2 contains two executable binary files:
- bzip2
- bzip2recover

bzip2: This is the main program used for compressing files using a block-sorting file compressor. It also includes related commands like bunzip2, which is used for decompression, and bzcat, which decompresses files to stdout​​​​.
bzip2recover: This utility is specifically designed for recovering data from damaged bzip2 files. It works by searching for blocks in .bz2 files and writing each block out into its own .bz2 file. This allows users to test the integrity of these blocks and decompress those which are undamaged​​.

We will only sh
ow executable binary bzip2 in the aritfact

```bash
cargo build --release
```

## which  
```bash

```

## rosetta   
```bash

```



# Part 2: Scaffolding Boilerplate Generation 

## Step 0: Prepare
copy c-code/bzip2 to scaffolding
```bash
cp -r c-code/bzip2 scaffolding_test/
```

## Step 1: add save preprocessed file *.i during `make`   

mv
```bash
cd scaffolding_test/bzip2
# replace Makefile with the one that can save temp files
cp -r Makefile-save-temps Makefile
```

### Step 1.1 remove directives from *.i
 
```bash
for file in *.i; do awk '!/^#[ \t]*[0-9]+[ \t]+"/' "$file" > "${file}.tmp" && mv "${file}.tmp" "$file"; done

```

### Step 1.2 rename `bzip2recover.i` to `bzip2recover.i.bk`

Since we only focus on bzip2 executable binary, we need to exclude the bzip2recover.i
( Caveat: if the binary has more than one executable, we should exclude the unnecessary one )

## Step 2: Generating C Tags: Command for Recursive Ctags with Custom Fields and Language Mapping
   
```bash
ctags -R --fields=+l --c-kinds=+v+f --languages=C --langmap=C:.i -o ctagop.txt 
```

   
## Step 3: Use cflow

When using the cflow tool for a C project, it's generally recommended to have only one main function in the project. cflow is designed to analyze function call relationships in C programs and generates a call graph. If there are multiple main functions, cflow might face difficulties, as the main function typically serves as the entry point of a program. For projects with multiple main functions, like those containing independent sub-projects, you might need to run cflow separately for each part or adjust the project structure for effective analysis. In summary, having a single main function is the best practice for using cflow, unless you have specific needs and strategies to handle multiple instances.  


```bash
python3 cflow_generation.py /root/rustmap/bzip2-real-test
```



## Step 4: Generate RustMap Scaffolding
```bash
python3 extract.py /root/rustmap/bzip2-real-test
```

# Part 3: Case Study: Why we need Strongly Connected Component Recursive dependency?

 