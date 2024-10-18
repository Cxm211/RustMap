use crate::global_vars::bzip2_c1::{DState};


// makeMaps_d ( DState* s )


// 定义 make_maps_d 函数
fn make_maps_d(s: &mut DState) {
    s.nInUse = 0;
    for i in 0..256 {
        if s.inUse[i as usize] {
            s.seqToUnseq[s.nInUse as usize] = i as u8;
            s.nInUse += 1;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::DState;
    use super::make_maps_d;

    #[test]
    fn test_make_maps_d() {
        let mut ds = DState::default();

        // Initialize the structure with test values
        for i in 0..256 {
            ds.inUse[i] = i % 2 == 0;  // Assume even indices are in use
            ds.seqToUnseq[i] = 0;     // Clear previous values
        }

        make_maps_d(&mut ds);

        // Assertions
        assert_eq!(ds.nInUse, 128);  // Half of 256 (the even numbers) are in use

        for i in 0..ds.nInUse {
            assert_eq!(ds.seqToUnseq[i as usize], 2 * i as u8);  // Verify that we stored even numbers
        }
    }
}
