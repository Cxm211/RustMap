# Rewriting Pointer Aliasing
In C, pointer aliasing especially involving arrays, is often used to optimize memory usage and access patterns. The flexibility to cast pointers from one type to another enables flexible manipulation of data structures with varied granularity. However, it may lead to potential issues with type safety and endianness sensitivity.

For example, in the code below excerpted from bzip2, the EState structure contains three array pointers of different data types, and a 32-bit signed integer that indicates the number of blocks. The line 10 `s->block = (UChar*)s->arr2`; casts `UInt32*` to `UChar*`; the resulting pointer `block` is thus an alias of `arr2` of a different type and can access individual bytes of the integers pointed to by `arr2`. Whether the memory operations done through `block` is correct is dependent on the system’s byte order/endianness.

### Sample struct with Pointer Aliasing in C
```c
// bzlib_private.h
typedef struct {
    UChar* zbits;
    UInt32* arr2;
    UChar* block;
    Int32  nblock;
} EState;
// pointer aliasing without endianness concern (in bzlib.c)
// initialize EState *s
s->block = (UChar*)s->arr2;
// pointer aliasing with endianness concern (in compress.c)
s->zbits = (UChar*) (&((UChar*)s->arr2)[s->nblock]);

// passing one of both fields into calling function in fallbackSort
fallbackSort (/* ... */ s->arr2, /* ... */);
otherFunction (s->zbits);
```

## Rewrite Pointer Aliasing in C2Rust
The code below shows the Rust code translated by C2Rust: C2Rust keeps `arr2`, `block`, and `zbits` as raw pointers, and uses unsafe code blocks, that can increase the risk of unsafe memory operations and errors.

Another issue with the C2Rust translation is that it produces multiple definitions of struct EState across different translated files compress.rs, blocksort.rs, and bzlib.rs, even though the struct definition only appears once in one header file bzlib_private.h in the original C program. C2Rust and all C2Rust-based tools have this issue because it preprocesses each .c file into .i first before translation and if multiple .c files contain the same .h file, the header contents would appear multiple times across the files and C2Rust would produce erroneously duplicate code.

```rust
// Duplicated Defined in compress.rs, blocksort.rs, bzlib.rs
pub struct EState {
    pub arr2: *mut UInt32,
    pub block: *mut UChar,
    pub zbits: *mut UChar,
    pub nblock: Int32
    /* ... */
}
// bzlib.rs (without endianness concern)
unsafe{ (*s).block = (*s).arr2 as *mut UChar; }
// compress.rs (with endianness concern)
unsafe {
    (*s).zbits =
        &mut *((*s).arr2 as *mut UChar).offset((*s).nblock as isize) as
            *mut UChar;
}

// blocksort.rs
fallbackSort(/* ... */, (*s).arr2, /* ... */);
```

## Rewrite Pointer Aliasing in RustMap
RustMap replaces C pointer aliasing in a safer and idiomatic way that uses vectors and conversion functions. First, when prompting LLM with the struct definition in C, we also ask LLM to infer the suitable vector type for each pointer when converting it to Rust. For
example, Figure 8a shows that the pointers are converted to use safe collections
like `Vec<u8>` and `Vec<u32>`. However, note that both `arr2` and `zbits` are commented out, because they point to the same memory as `block` in the original C, and we can use just one vector to represent all three. We also used LLM to identify potential aliasing among all pointers in the given code during prompting. Then, to mimic the memory operations done via aliased pointers in the original C program, we also prompt LLM to construct specific functions for each aliasing pointer to read from and write to the aliased memory. The code below show-case such functions for the `arr2` and `zbits` pointers, respectively. The internal workings of the functions vary, depending on whether they are concerned of endianness.

### Sample C code of Pointer Aliasing in RustMap
```c
// Define only once in headers/bzlib_private.rs 
// use bzlib/globals_bzlib.rs 
pub struct EState {
    pub block: Vec<u8>, // use block to represent zbits
    // pub arr2: Vec<u32> 
    // pub zbits: Vec<u8>,
    pub nblock: i32,
}
```

### Aliased Pointer Operations WITHOUT Endianness Concern
```rust
pub fn rebuild_arr2_from_block(block: &[u8]) -> Vec<u32> {
    /* reconstructs a u32 array from a block of bytes */
}

pub fn build_block_from_arr2(arr2: &[u32]) -> Vec<u8> {
    /* converts a u32 array into a block of bytes */
}
// blocksort/scc_71_fallbacksort.rs
let mut arr2 = build_block_from_arr2(s.block);
fallback_sort(/*...*/, &mut arr2, /*...*/);
// Subsequent assignment of modified temporary arr2 to s.block
s.block = rebuild_arr2_from_block(&arr2);
```
Case 1: Pointer Aliasing without Endianness Concern. In this case, the two conversion functions are constructed by ChatGPT with human feedback. They essentially make copies of the array referenced by `block` before/after each read/write operation for safer access. Lines 8–12 illustrates their usage for migrating C code involving operations on aliasing pointer: a temporary variable `arr2` is used to hold a new copy of the original array referenced by `s.block`; then all read/write operations via the original `arr2` in C are done via the temporary variable; finally, a new copy of the array referenced by the temporary variable is passed back to the original reference `s.block`. Tihs way of translation produces safer access to shared arrays, while it may still not be safe for multi-thread programs and slow down the efficiency of the code. We leave its improvements for future work.


### Aliased Pointer Operations WITH Endianness Concern
```rust
pub fn get_zbits_from_arr2(estate: &mut EState) {
    /* construct a temporary arr2:Vec<u32> from estate.block:Vec<u8> */
    /* updates the zbits field based on the arr2, and
       Uses if-else checks based on system endianness. E.g., */
    let mut bytes = if cfg!(target_endian = "little") {
        arr2[offset].to_le_bytes() // Convert to little-endian byte array
    } else {
        arr2[offset].to_be_bytes() // Convert to big-endian byte array
    };
}

pub fn update_arr2_from_zbits(s: &mut EState, zbits: &[u8]){
    /* updates the arr2 field based on the zbits values.
       Uses if-else checks based on system endianness. E.g., */
    arr2[offset] = if cfg!(target_endian = "little") {
        u32::from_le_bytes(bytes) // Convert bytes back to little-endian u32
    } else {
        u32::from_be_bytes(bytes) // Convert bytes back to big-endian u32
    };
    /* converts arr2 array into s.block */
}

// In BZ2_compressBlock
let mut s = // initialize new EState
let mut zbits: Vec<u8> = get_zbits_from_arr2(&mut s); // obtain zbits
otherFunction(&zbits, ...);
update_arr2_from_zbits(&mut s, &zbits); // update block
```
Case 2: Pointer Aliasing with Endianness Concerns. Endianness concerns arise in C due to direct memory manipulation that may be sensitive to byte order, as shown in Sample struct with Pointer Aliasing in C line 12. In this line, `arr2` is first cast to `UChar*`. Then, the address of the `nblock`-th element of this array is taken and cast to `UChar*` and assigned to `zbits`. These seemingly convoluted operations may indicate the code’s intention to be endianness-aware, and thus the translated Rust code should also ensure correct handling of endianness.
The code above illustrates the two conversion functions and their usages for such cases. The essential idea of the conversion functions is the same as the case without concerning endianness by using a temporary variable to hold copies of the array; the main differences are the if-else checks of system endianness to manage byte order correctly when making copies of the array.

Compared to the direct raw pointer operation translated by C2Rust, RustMap handles pointer aliasing and endianness in a much safer way. Whether to be concerned of endianness when translating operations on an aliased pointer, we also prompted ChatGPT to make the choice.

