pub fn bz2_bz_decompress_end(strm: &mut bz_stream) -> Result<i32, i32> {
    let s = strm.state.as_mut().ok_or(-2)?;

    if s.strm.as_ref().map_or(true, |ptr| ptr as *const _ != strm) {
        return Err(-2);
    }

    if let Some(tt) = s.tt.take() {
        (strm.bzfree)(strm.opaque, tt);
    }
    if let Some(ll16) = s.ll16.take() {
        (strm.bzfree)(strm.opaque, ll16);
    }
    if let Some(ll4) = s.ll4.take() {
        (strm.bzfree)(strm.opaque, ll4);
    }

    (strm.bzfree)(strm.opaque, strm.state.take().unwrap());
    Ok(0)
}
