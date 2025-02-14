pub fn fallback_simple_sort(fmap: &mut [u32], eclass: &[u32], lo: i32, hi: i32) {
    let mut i: i32;
    let mut j: i32;
    let mut tmp: u32;
    let mut ec_tmp: u32;

    // If lo and hi are the same, return early as there's nothing to sort
    if lo == hi {
        return;
    }

    // Sort larger subarrays where the size difference is greater than 3
    if hi - lo > 3 {
        i = hi - 4;
        while i >= lo {
            tmp = fmap[i as usize];
            ec_tmp = eclass[tmp as usize];

            j = i + 4;
            while j <= hi && ec_tmp > eclass[fmap[j as usize] as usize] {
                fmap[(j - 4) as usize] = fmap[j as usize];
                j += 4;
            }

            fmap[(j - 4) as usize] = tmp;
            i -= 1;
        }
    }

    // Standard sorting loop for the remaining elements
    i = hi - 1;
    while i >= lo {
        tmp = fmap[i as usize];
        ec_tmp = eclass[tmp as usize];

        j = i + 1;
        while j <= hi && ec_tmp > eclass[fmap[j as usize] as usize] {
            fmap[(j - 1) as usize] = fmap[j as usize];
            j += 1;
        }

        fmap[(j - 1) as usize] = tmp;
        i -= 1;
    }
}
