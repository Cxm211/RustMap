use crate::global_vars::bzip2_c2::*;
pub fn has_suffix(s: &[Char], suffix: &[Char]) -> bool {
    // 获取s的长度
    let ns = s.len(); 
    // 获取suffix的长度
    let nx = suffix.len(); 
    // 如果s的长度小于suffix的长度，则返回false
    if ns < nx {
        return false;
    }
    // 比较s的最后几个字符和suffix是否相同，相同则返回true，否则返回false
    &s[(ns-nx)..] == suffix
}

pub fn map_suffix(name: &mut Vec<Char>, old_suffix: &[Char], new_suffix: &[Char]) -> bool {
    // 如果name不以oldSuffix为后缀，则返回false
    if !has_suffix(name, old_suffix) {
        return false;
    }
    // 将name的后缀oldSuffix去除
    let new_length = name.len() - old_suffix.len();
    name.truncate(new_length);
    // 追加新后缀newSuffix
    name.extend_from_slice(new_suffix);
    true  // 返回true表示后缀已成功更改
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_suffix() {
        assert!(has_suffix(b"filename.txt", b".txt"));
        assert!(!has_suffix(b"filename.txt", b".png"));
    }

    #[test]
    fn test_map_suffix() {
        let mut name1 = b"image.jpg".to_vec();
        let mut name2 = b"document.txt".to_vec();
        
        assert!(map_suffix(&mut name1, b".jpg", b".png"));
        assert_eq!(name1, b"image.png");
        
        assert!(!map_suffix(&mut name2, b".jpg", b".png"));
        assert_eq!(name2, b"document.txt");
    }
}
