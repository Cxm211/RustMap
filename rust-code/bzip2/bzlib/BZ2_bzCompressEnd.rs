pub fn bz2_bz_compress_end(strm: &mut bz_stream) -> i32 {
    // Check if the stream is null
    if strm.is_none() {
        return -2;
    }

    let s = match strm.state.as_mut() {
        Some(state) => state,
        None => return -2,
    };

    if !std::ptr::eq(s.strm.unwrap() as *const _, strm as *const _) {
        return -2;
    }

    // Free allocated resources if they are not None
    if let Some(arr1) = s.arr1.take() {
        (strm.bzfree)(strm.opaque, arr1.as_ptr() as *mut _);
    }
    if let Some(arr2) = s.arr2.take() {
        (strm.bzfree)(strm.opaque, arr2.as_ptr() as *mut _);
    }
    if let Some(ftab) = s.ftab.take() {
        (strm.bzfree)(strm.opaque, ftab.as_ptr() as *mut _);
    }

    // Free the state itself
    (strm.bzfree)(strm.opaque, strm.state.take().unwrap() as *mut _ as *mut _);

    // Set the state to None to signify it has been freed
    strm.state = None;

    0
}
