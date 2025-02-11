## Sample Translations of Numerical Macros

The following example shows sample numerical macros and their refactoring and translations.  
For example, `#define BZ_RUNB 1` is refactored to `const int BZ_RUNB=1;` in C, and later translated to `const BZ_RUNB:i32=1;` in Rust. The refactor C code and the translated Rust code would both keep using the variable `BZ_RUNB` instead of the value `1`, while C2Rust directly puts the value into the translated code, lowing code readability.

### **Original C Macros**
```c
#define BZ_RUNB 1
#define BZ_M_IDLE 1
#define BZ_S_OUTPUT 1
```

### **Refactored as `const` Global Variables in C**
```c
const int BZ_RUNB = 1;
const int BZ_M_IDLE = 1;
const int BZ_S_OUTPUT = 1;
```

### **Translated into Rust**
```rust
const BZ_RUNB: i32 = 1;
const BZ_M_IDLE: i32 = 1;
const BZ_S_OUTPUT: i32 = 1;
```

> **Note:** Converting `#define` macros to `const` variables improves **type safety**,  
> enhances **readability**, and prevents unintended macro expansions.

---

## Sample Translations of Complex Macros

The following example shows an example where one complex macro `fswap(zz1, zz2)` is converted to a function `fswap(UInt32* zz1, UInt32* zz2)`.

### **Original Complex Macro in C**

```c
#define fswap(zz1, zz2) \
{ Int32 zztmp = zz1; zz1 = zz2; zz2 = zztmp; }



static
void fallbackQSort3 ( UInt32* fmap, 
                      UInt32* eclass,
                      Int32   loSt, 
                      Int32   hiSt )
{
...
if (n == 0) { 
   fswap(fmap[unLo], fmap[ltLo]); 
   ltLo++; unLo++; 
   continue; 
};
...
}
```

### **Rewritten as a Function in C**

```c
static void fswap(UInt32* zz1, UInt32* zz2) {
    UInt32 zztmp = *zz1;
    *zz1 = *zz2;
    *zz2 = zztmp;
}

static
void fallbackQSort3 ( UInt32* fmap, 
                      UInt32* eclass,
                      Int32   loSt, 
                      Int32   hiSt )
{
...
if (n == 0) {
   fswap(&fmap[unLo], &fmap[ltLo]);
   ltLo++; unLo++;
   continue;
}
...
}
```

> **Note:** Converting the macro `fswap` to a function improves code maintainability and reduces macro-related issues.

---
## Samples of Unhandled Macros

For example, `#ifdef _WIN32` is used to check whether the code is being compiled on a Windows system, allowing conditional compilation of platform-specific implementations, and the value of `_WIN32` is automatically defined by the compiler when targeting a Windows platform.

```c
#ifdef _WIN32
#   include <windows.h>
#   ifdef small
#      undef small
#   endif
#   ifdef BZ_EXPORT
#   define BZ_API(func) WINAPI func
#   define BZ_EXTERN extern
#   else
#   define BZ_API(func) (WINAPI * func)
#   define BZ_EXTERN
#   endif
#else
#   define BZ_API(func) func
#   define BZ_EXTERN extern
#endif
```

This macro setup ensures that functions and external variables are handled correctly across different platforms, maintaining compatibility while leveraging platform-specific features.

