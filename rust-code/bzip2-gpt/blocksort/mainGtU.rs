pub fn main_gt_u(
    mut i1: u32,
    mut i2: u32,
    block: &[u8],
    quadrant: &[u16],
    nblock: u32,
    budget: &mut i32,
) -> bool {
    let mut k: i32;
    let mut c1: u8;
    let mut c2: u8;
    let mut s1: u16;
    let mut s2: u16;

    // Initial comparisons for the first few characters
    c1 = block[i1 as usize];
    c2 = block[i2 as usize];
    if c1 != c2 {
        return c1 > c2;
    }
    i1 += 1;
    i2 += 1;

    c1 = block[i1 as usize];
    c2 = block[i2 as usize];
    if c1 != c2 {
        return c1 > c2;
    }
    i1 += 1;
    i2 += 1;

    c1 = block[i1 as usize];
    c2 = block[i2 as usize];
    if c1 != c2 {
        return c1 > c2;
    }
    i1 += 1;
    i2 += 1;

    c1 = block[i1 as usize];
    c2 = block[i2 as usize];
    if c1 != c2 {
        return c1 > c2;
    }
    i1 += 1;
    i2 += 1;

    c1 = block[i1 as usize];
    c2 = block[i2 as usize];
    if c1 != c2 {
        return c1 > c2;
    }
    i1 += 1;
    i2 += 1;

    c1 = block[i1 as usize];
    c2 = block[i2 as usize];
    if c1 != c2 {
        return c1 > c2;
    }
    i1 += 1;
    i2 += 1;

    c1 = block[i1 as usize];
    c2 = block[i2 as usize];
    if c1 != c2 {
        return c1 > c2;
    }
    i1 += 1;
    i2 += 1;

    c1 = block[i1 as usize];
    c2 = block[i2 as usize];
    if c1 != c2 {
        return c1 > c2;
    }
    i1 += 1;
    i2 += 1;

    c1 = block[i1 as usize];
    c2 = block[i2 as usize];
    if c1 != c2 {
        return c1 > c2;
    }
    i1 += 1;
    i2 += 1;

    k = nblock as i32 + 8;

    // Loop to handle comparisons, including quadrant data
    loop {
        c1 = block[i1 as usize];
        c2 = block[i2 as usize];
        if c1 != c2 {
            return c1 > c2;
        }
        s1 = quadrant[i1 as usize];
        s2 = quadrant[i2 as usize];
        if s1 != s2 {
            return s1 > s2;
        }
        i1 += 1;
        i2 += 1;

        c1 = block[i1 as usize];
        c2 = block[i2 as usize];
        if c1 != c2 {
            return c1 > c2;
        }
        s1 = quadrant[i1 as usize];
        s2 = quadrant[i2 as usize];
        if s1 != s2 {
            return s1 > s2;
        }
        i1 += 1;
        i2 += 1;

        c1 = block[i1 as usize];
        c2 = block[i2 as usize];
        if c1 != c2 {
            return c1 > c2;
        }
        s1 = quadrant[i1 as usize];
        s2 = quadrant[i2 as usize];
        if s1 != s2 {
            return s1 > s2;
        }
        i1 += 1;
        i2 += 1;

        c1 = block[i1 as usize];
        c2 = block[i2 as usize];
        if c1 != c2 {
            return c1 > c2;
        }
        s1 = quadrant[i1 as usize];
        s2 = quadrant[i2 as usize];
        if s1 != s2 {
            return s1 > s2;
        }
        i1 += 1;
        i2 += 1;

        c1 = block[i1 as usize];
        c2 = block[i2 as usize];
        if c1 != c2 {
            return c1 > c2;
        }
        s1 = quadrant[i1 as usize];
        s2 = quadrant[i2 as usize];
        if s1 != s2 {
            return s1 > s2;
        }
        i1 += 1;
        i2 += 1;

        // Wrap-around indexing when reaching the end of the block
        if i1 >= nblock {
            i1 -= nblock;
        }
        if i2 >= nblock {
            i2 -= nblock;
        }

        k -= 8;
        *budget -= 1;

        if k < 0 {
            break;
        }
    }

    false
}
