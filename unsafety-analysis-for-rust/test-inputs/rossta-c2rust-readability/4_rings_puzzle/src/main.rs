#[no_mangle]
pub unsafe extern "C" fn ge() {
    e = lo;
    while e <= hi {
        if unique == 0 || e != a && e != c && e != d {
            g = d + e;
            if g >= lo && g <= hi
                && (unique == 0 || g != a && g != c && g != d && g != e)
            {
                bf();
            }
        }
        e += 1;
        e;
    }
}