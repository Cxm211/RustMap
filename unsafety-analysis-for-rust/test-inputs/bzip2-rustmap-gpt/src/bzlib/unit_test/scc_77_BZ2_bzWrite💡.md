请提供以下信息：

BZFILE 和 bzFile 结构的定义。
pub struct bzFile {
 pub   handle: *mut File,
  pub  buf: [u8; 5000],
  pub  bufN: i32,
 pub   writing: bool,
  pub  strm: bz_stream,
  pub  strmD: bz_stream_DState,
 pub   lastErr: i32,
 pub   initialisedOk: bool,
}
type BZFILE = *mut bzFile; 

BZ2_bzCompress 函数的签名和行为描述。
pub fn bz2_compress(strm: &mut bz_stream, action: i32) -> i32
Bool 和 UChar 类型的定义。
pub type UChar = u8;
pub type Bool = bool;
