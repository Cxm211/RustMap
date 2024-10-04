pub fn BZ2_bzlibVersion() -> &'static str {
    "1.0.8, 13-Jul-2019"
}

pub extern "C" fn BZ2_bz__AssertH__fail(errcode: i32) {
    println!(
        "\n\nbzip2/libbzip2: internal error number {}.\n\
        This is a bug in bzip2/libbzip2, {}.\n\
        Please report it to: bzip2-devel@sourceware.org...",
        errcode,
        BZ2_bzlibVersion()
    );

    if errcode == 1007 {
        println!(
            "*** A special note about internal error number 1007 ***\n"
        );
    }

    std::process::exit(3);
}
