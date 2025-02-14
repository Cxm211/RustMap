# Rewriting Global Variables
Global variables are widely used in C programs,
but if directly translated into Rust, they are not thread-safe and can cause static initialization order fiasco if imported multiple times into different files. Thus, simple syntax-rule-based translation done by other tools are insufficient in ensuring the safety of the translated code.

For global variables that are of primitive types in C, we observe that we can still use simple rule-based automation translation like other tools without the cost of invoking LLMs and associated uncertainty and error fixing, but need to enforce the global variables are initialized only once in a thread-safe manner when they are first accessed by using the `lazy_static` macro and `Mutex` in Rust. Specifically, we rewrite C primitive-type global variables using `lazy_static` and providing getter and setter functions to replace direct accesses to them, as recommended for Rust programmers. As a comparison, the code translated by C2Rust would lead to reading or modifying a mutable static variable, that is an unsafe practice in Rust.

For non-primitive global variables, because it is more challenging to use rule-based automation to rewrite accesses into various data fields of non-primitive global variables, we leave it to LLMs to translate them following the same translation process for other code constructs and rely on the same error checking and fixing processes to ensure their functionality.

## Sample Translations of Global Variables
The example below illustrates [sample global variables declared in C](#sample-global-variable-definitions-in-c) and their translations [by C2Rust](#sample-global-variables-translated-by-c2rust) and [by RustMap](#sample-global-variables-translated-via-lazy_static).
For example, we rewrite `fileMetaInfo` via a `Mutex` inside the body of the `lazy_static` macro. In the Rust code using [RustMap](#sample-global-variables-translated-via-lazy_static), `lazy_static` is used to create lazily instantiated global variables.
This is necessary because Rust does not support mutable global variables directly. By using `lazy_static` and `Mutex`, we ensure that these global variables are safely initialized only once, and access to them is thread-safe, preventing data races.
### Sample Global Variable Definitions in C
```c
// bzip2.i
struct MY_STAT fileMetaInfo;
...

static 
void saveInputFileMetaInfo ( Char *srcName )
{
#  if BZ_UNIX
   IntNative retVal;
   retVal = MY_STAT( srcName, &fileMetaInfo );
   ERROR_IF_NOT_ZERO ( retVal );
#  endif
}
```

---

### Sample Global Variable Definitions translated by C2Rust
```c
// bzip2.rs from C2Rust
static mut fileMetaInfo: stat = stat {
    st_dev: 0,
    st_ino: 0,
    st_nlink: 0,
    st_mode: 0,
    st_uid: 0,
    st_gid: 0,
    __pad0: 0,
    st_rdev: 0,
    st_size: 0,
    st_blksize: 0,
    st_blocks: 0,
    st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
    st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
    st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
    __glibc_reserved: [0; 3],
};
unsafe extern "C" fn saveInputFileMetaInfo(mut srcName: *mut Char) {
    let mut retVal: IntNative = 0;
    retVal = stat(srcName, &mut fileMetaInfo);
    ERROR_IF_NOT_ZERO(retVal);
}
```
---

### Sample Global Variables translated via `lazy_static!`
```rust
// idiomatic lazily instantiated global variable
// bzip2/scc_84_applySavedTimeInfoToOutputFile_85_applySavedFileAttrToOutputFile_86_saveInputFileMetaInfo.rs

lazy_static! {
    static ref FILE_META_INFO: Mutex<Option<fs::Metadata>> = Mutex::new(None);
}

pub fn save_input_file_meta_info(src_name: &str) -> std::io::Result<()> {
    let metadata = fs::metadata(src_name)?;
    let mut file_meta_info = FILE_META_INFO.lock().unwrap();
    *file_meta_info = Some(metadata);
    Ok(())
}

```

