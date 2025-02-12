## Sample Translations of Global Variables
The example below illustrates [sample global variables declared in C](#sample-global-variable-definitions-in-c) and their translations [by C2Rust](#sample-global-variables-translated-by-c2rust) and [by RustMap](#sample-global-variables-translated-via-lazy_static).
For example, we rewrite `int numFileNames` via a `Mutex` inside the body of the `lazy_static` macro. In the Rust code using [RustMap](#sample-global-variables-translated-via-lazy_static), `lazy_static` is used to create lazily instantiated global variables.
This is necessary because Rust does not support mutable global variables directly. By using `lazy_static` and `Mutex`, we ensure that these global variables are safely initialized only once, and access to them is thread-safe, preventing data races.
### Sample Global Variable Definitions in C
```c
// bzip2.i
int numFileNames, numFileProcessed, blockSized100k;
```

---

### Sample Global Variable Definitions translated by C2Rust
```c
// bzip2.rs from C2Rust
#[no_mangle]
pub static mut numFileNames: Int32 = 0;
#[no_mangle]
pub static mut numFilesProcessed: Int32 = 0;
#[no_mangle]
pub static mut blockSize100k: Int32 = 0;
```

---

### Sample Global Variables translated via `lazy_static!`
```rust
// idiomatic lazily instantiated global variable
// bzip2/global_bzip2.rs
lazy_static! { 
    pub static ref numFileNames: Mutex<i32> = Mutex::new(0);  
    pub static ref numFilesProcessed: Mutex<i32> = Mutex::new(0);   
    pub static ref blockSize100k: Mutex<i32> = Mutex::new(0);
}

pub fn get_numFileNames() -> i32 { /* ... */ } 
pub fn set_numFileNames(new_value: i32) { /* ... */ }

pub fn get_numFilesProcessed() -> i32 { /* ... */ } 
pub fn set_numFilesProcessed(new_value: i32) { /* ... */ }

pub fn get_blockSize100k() -> i32 { /* ... */ } 
pub fn set_blockSize100k(new_value: i32) { /* ... */ }
```

