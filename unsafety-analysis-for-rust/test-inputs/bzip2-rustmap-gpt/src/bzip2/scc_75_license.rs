use std::io::{self, Write};
use crate::bzlib::scc_42_root_57_BZ2_bzlibVersion::*;
pub fn license() {
    writeln!(
        io::stderr(),
        "bzip2, a block-sorting file compressor.  Version {}.\n\
        \n\
        Copyright (C) 1996-2019 by Julian Seward.\n\
        \n\
        This program is free software; you can redistribute it and/or modify\n\
        it under the terms set out in the LICENSE file, which is included\n\
        in the bzip2 source distribution.\n\
        \n\
        This program is distributed in the hope that it will be useful,\n\
        but WITHOUT ANY WARRANTY; without even the implied warranty of\n\
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\n\
        LICENSE file for more details.\n\
        \n",
        bz2_bzlib_version()
    )
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_license_output() {
        // 如前面的C代码所述，此测试只是为了观察license函数的行为。
        // 在真实的测试环境中，可能需要捕获stderr的输出来进行验证。
        license();
    }
}
