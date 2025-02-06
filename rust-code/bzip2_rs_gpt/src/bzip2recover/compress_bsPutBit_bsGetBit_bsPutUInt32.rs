use std::io::{Cursor, Write, Read};
use std::convert::TryInto;
use crate::global_vars::bzip2_c1::*;
use crate::global_vars::bzip2_c2::*;
use crate::global_vars::bzip2_c4::*;
use crate::bzip2recover::writeError::*;
use std::alloc::{alloc, realloc, Layout};

