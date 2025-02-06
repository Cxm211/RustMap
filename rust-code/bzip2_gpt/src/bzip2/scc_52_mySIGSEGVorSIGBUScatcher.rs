use crate::bzip2::scc_79_root_46_setExit::*;
use crate::global_vars::bzip2_c2::*;
use crate::global_vars::bzip2_c5::*;
fn my_sigsegv_or_sigbus_catcher(n: i32) {
    let op_mode = unsafe {OP_MODE};
    let msg = if op_mode == 1 {
        ": Caught a SIGSEGV or SIGBUS whilst compressing.\n\
        \n\
        Possible causes are (most likely first):\n\
        (1) This computer has unreliable memory or cache hardware\n\
        \t(a surprisingly common problem; try a different machine.)\n\
        (2) A bug in the compiler used to create this executable\n\
        \t(unlikely, if you didn't compile bzip2 yourself.)\n\
        (3) A real bug in bzip2 -- I hope this should never be the case.\n\
        The user's manual, Section 4.3, has more info on (1) and (2).\n\
        \n\
        If you suspect this is a bug in bzip2, or are unsure about (1)\n\
        or (2), feel free to report it to: bzip2-devel@sourceware.org.\n\
        Section 4.3 of the user's manual describes the info a useful\n\
        bug report should have. If the manual is available on your\n\
        system, please try and read it before mailing me. If you don't\n\
        have the manual or can't be bothered to read it, mail me anyway.\n"
    } else {
        ": Caught a SIGSEGV or SIGBUS whilst decompressing.\n\
        \n\
        Possible causes are (most likely first):\n\
        (1) The compressed data is corrupted, and bzip2's usual checks\n\
        \tfailed to detect this. Try bzip2 -tvv my_file.bz2.\n\
        (2) This computer has unreliable memory or cache hardware\n\
        \t(a surprisingly common problem; try a different machine.)\n\
        (3) A bug in the compiler used to create this executable\n\
        \t(unlikely, if you didn't compile bzip2 yourself.)\n\
        (4) A real bug in bzip2 -- I hope this should never be the case.\n\
        The user's manual, Section 4.3, has more info on (2) and (3).\n\
        \n\
        If you suspect this is a bug in bzip2, or are unsure about (2)\n\
        or (3), feel free to report it to: bzip2-devel@sourceware.org.\n\
        Section 4.3 of the user's manual describes the info a useful\n\
        bug report should have. If the manual is available on your\n\
        system, please try and read it before mailing me. If you don't\n\
        have the manual or can't be bothered to read it, mail me anyway.\n"
    };

    unsafe {
        eprintln!("\n{}", String::from_utf8_lossy(&IN_NAME));
        eprintln!("{}", msg);
        eprintln!("\tInput file = {}", String::from_utf8_lossy(&OUT_NAME));
        eprintln!("\tOutput file = {}", String::from_utf8_lossy(&TMP_NAME));
    }
    if op_mode == 1 { set_exit(3); } else { set_exit(2); }
    
}
