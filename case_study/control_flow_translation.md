# Control Flow Translation: `switch-cases`
In C, switch statements can fall through, i.e., the execution of a case that does not contain a break statement will continue directly into the next case. In contrast, Rust’s `match`statement does not have fall-through; each arm must explicitly end with a value or an action, ensuring that only one arm is executed. The first code example below shows the original C code with a switch-case structure. The second code example shows the Rust code rewritten by C2Rust. The third code example shows the Rust code translated by RustMap, as suggested by ChatGPT. By leveraging match statements in a loop that encapsulates state transitions explicitly, it reduces unsafe code usage, and improves readability and maintainability.

## Sample C switch-cases with fallthrough
```c
// decompress.i 
switch (s->state) {
    /* ... */ 
    case 11:
    /* code without break */
    case 12:
    /* code without break */
    /* ... */
```
## Rewritten by C2Rust
```rust
// decompress/scc_59_BZ2_decompress.rs
match (*s).state {
    /* .. ... */
    11 => current_block = 12259750428863723923,
    12 => current_block = 15146946972525368609
    /* ... */ 
}
match current_block {
    12259750428863723923 => {
         /* ... */ \lx{something needs to be here}
    }
    15146946972525368609 => {
         /* ... */  
    }
     /* ... */
}
```

## Rewritten by RustMap
```rust
// decompress/scc_59_BZ2_decompress.rs
'state_loop: while s.state <= 50 {
    match s.state {
        /* ... */
        11 => {
            /* ... */
            s.state = 12
        },       
        12 => {
            /* ... */
            s.state = 12
        }
        /* ... */
    }
}
```

# Control Flow Translation: `goto`
`goto` statements, commonly used in C for jumping to specific code locations, are not support in Rust. The first code below is an example of using `goto` in C code. The GET_BITS macro calls another macro, RETURN, that utilizes `goto`. GET_BITS itself is used within a switch-case with fallthrough.
When prompted with all the code at once, ChatGPT could not generate correct translated Rust, although it provided valid suggestions to consider wrapping the code at the `goto` target into a function and splitting the code in the macro into two parts at the conditional statement checking for the `goto`. We then refined the prompts for ChatGPT according to its own suggestions, to (1) wrap the code at and after the save_state_and_return target label as a function and replace the `goto` with a function call, and (2) split the code at the if statement and replace the use of the GET_BITS macro with calls to the two split functions. ChatGPT was then successful in performing the tasks, producing one function save_state_and_return for the `goto` and two split functions GET_BITS_first_half and GET_BITS_second_half, based on its understanding of the functionality of the code. The first split function attempts to perform the same functionality of the code before the if statement and returns a Result to indicate if it is successful (OK) or not (Err). The second split function wraps all the remaining code after the if statement, ensuring correct functionality. The second code example below shows the Rust code by RustMap. This splitting and replacement worked well, although specific for this case; it makes the translated code more readable and maintainable through explicit error handling and state management.

In contrast, C2Rust uses a more generic approach to compile away `goto` and `switch-cases` by implementing the Relooper control-flow analysis and restructuring, but it does not consider project-specific code functionality, and its translated code grows much in size and becomes harder to read. Future work may consider using Rust `goto` macros with the prompts of RustMap to make the translated code more readable while retaining code functionality.

## Sample goto in C
```c
// decompress.i 
// definition of two macros
#define GET_BITS(lll,vvv,nnn)                     \
  ...                                             \
  if (s->strm->avail_in == 0) RETURN(BZ_OK);      \
  ...

#define RETURN(rrr)                               \
   { retVal = rrr; goto save_state_and_return; };

// usage of two macros
switch (s->state) {
  // case 13:
  GET_BITS(BZ_X_MAGIC_4, s->blockSize100k, 8)
```

## Rewritten by RustMap
```rust
fn save_state_and_return (s: &mut DState, /* ... */ ) {
    /* Code at and after goto target */
}
...
match s.state {
 13 => {
// #[cfg(debug_assertions)]
  s.state = 13;
  loop {
    let mut tmp_blockSize100k =  s.blockSize100k as u32;
    match GET_BITS_first_half(s, &mut tmp_blockSize100k, 8){     
        Ok(_) => {  s.blockSize100k = tmp_blockSize100k as i32; break;  }
        Err(_) => { s.blockSize100k = tmp_blockSize100k as i32; }, 
    } 
    if unsafe { (*s.strmD).avail_in } == 0 {  
        retVal = CONSTS.BZ_OK;   save_state_and_return(s, /* ... */);
        break; // break out the inner loop
    }
    GET_BITS_second_half(s);
...
```