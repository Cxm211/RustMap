
type UInt32 = u32;
type Int32 = i32;

pub fn fallback_simple_sort(fmap: &mut Vec<UInt32>, eclass: &Vec<UInt32>, lo: Int32, hi: Int32) {
    if lo == hi {
        return;
    }

    if hi - lo > 3 {
        for i in (lo..=(hi - 4)).rev() {
            let tmp = fmap[i as usize];
            let ec_tmp = eclass[tmp as usize];
            let mut j = i + 4;
            while j <= hi && ec_tmp > eclass[fmap[j as usize] as usize] {
                fmap[(j - 4) as usize] = fmap[j as usize];
                j += 4;
            }
            fmap[(j - 4) as usize] = tmp;
        }
    }

    for i in (lo..=hi - 1).rev() {
        let tmp = fmap[i as usize];
        let ec_tmp = eclass[tmp as usize];
        let mut j = i + 1;
        while j <= hi && ec_tmp > eclass[fmap[j as usize] as usize] {
            fmap[(j - 1) as usize] = fmap[j as usize];
            j += 1;
        }
        fmap[(j - 1) as usize] = tmp;
    }
}




#[cfg(test)]
mod tests {
    use super::fallback_simple_sort;
    #[test]
    fn test_fallback_simple_sort() {
        // 第一个测试用例
        let mut fmap1 = vec![120243, 242, 184, 183, 177, 113, 54];
        let mut eclass1 = vec![0u32; 120244];
        eclass1[120243] = 30061;
        eclass1[242] = 0;
        eclass1[184] = 90183;
        eclass1[183] = 0;
        eclass1[177] = 60126;
        eclass1[113] = 30122;
        eclass1[54] = 90187;
    
        fallback_simple_sort(&mut fmap1, &eclass1, 0, 6);
    
        let expected_fmap1 = vec![242, 183, 120243, 113, 177, 184, 54];
        for i in 0..7 {
            assert_eq!(fmap1[i], expected_fmap1[i]);
        }
    
        // 第二个测试用例
        let mut fmap2 = vec![210, 128, 220, 145, 159, 106];
        let mut eclass2 = vec![0u32; 221];
        eclass2[210] = 90200;
        eclass2[128] = 90187;
        eclass2[220] = 90187;
        eclass2[145] = 90200;
        eclass2[159] = 90200;
        eclass2[106] = 90200;
    
        fallback_simple_sort(&mut fmap2, &eclass2, 0, 5);
    
        let expected_fmap2 = vec![128, 220, 210, 145, 159, 106];
        for i in 0..6 {
            assert_eq!(fmap2[i], expected_fmap2[i]);
        }
    }
}

