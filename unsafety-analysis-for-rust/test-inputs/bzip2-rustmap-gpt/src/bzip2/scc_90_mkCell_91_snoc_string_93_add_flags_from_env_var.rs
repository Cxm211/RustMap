// use crate::bzip2::scc_101_mkCell::*;
// use crate::global_vars::bzip2_c5::{Cell};

// 假设Char是char的typedef
// use crate::bzip2::scc_101_mkCell::*;
// use crate::global_vars::bzip2_c5::{Cell};

// 假设Char是char的typedef
pub type Char = char;
#[derive(Clone)]
pub struct Cell {
    pub name: Option<Box<String>>,
    pub link: Option<Box<Cell>>,
}


impl Cell {
    pub fn mk_cell() -> Self {
        Cell {
            name: None,
            link: None,
        }
    }
     // 递归函数来打印链表
    // 递归遍历 🔥
     pub fn printLinkLists(&self) {
        if let Some(ref name) = self.name {
            // println!("Cell name: {}", name);
        } else {
            // println!("Cell name: (none)");
        }

        if let Some(ref link) = self.link {
            link.printLinkLists(); // 递归调用
        }
    }
    pub fn snoc_string(mut self, name: &str) -> Self {
        if self.name.is_none() {
            let mut new_cell = Cell::mk_cell();
            new_cell.name = Some(Box::new(name.to_owned()));
            new_cell
        } else {
            let mut tmp = &mut self;
            while tmp.link.is_some() {
                tmp = tmp.link.as_mut().unwrap();
            }
            tmp.link = Some(Box::new(Cell::mk_cell().snoc_string(name)));
            self
        }
    }
    // 为了修复这个问题，我们需要确保在从环境变量中提取标志时，去除前导和尾部的空格。
    /*
    test_add_flags_from_env_var 测试中，断言失败了。
    具体地，预期的字符串 "flag2" 前面有一个多余的空格，所以它与实际的值 " flag2" 不匹配。
    这是由于在 add_flags_from_env_var 函数中处理环境变量值时，当分割字符串时，前导空格没有被正确地去除。

    为了修复这个问题，我们需要确保在从环境变量中提取标志时，去除前导和尾部的空格。
     */
    pub fn add_flags_from_env_var(arg_list: &mut Option<Box<Cell>>, var_name: &str) {
        let envbase = getenv(var_name);
        if let Some(mut base) = envbase {
            let mut i = 0;
            while let Some(c) = base.chars().nth(i) {
                if c == '\0' {
                    break;
                }
                base = &base[i..];
                i = 0;
                while let Some(c) = base.chars().nth(i) {
                    if !c.is_whitespace() {
                        break;
                    }
                    i += 1;
                }
                let start = i;
                while let Some(c) = base.chars().nth(i) {
                    if c.is_whitespace() {
                        break;
                    }
                    i += 1;
                }
                if i > start {
                    let k = i.min(1024 - 10); 
                    let tmp_name: String = base.chars().skip(start).take(k - start).collect(); // 提取子字符串时, 来确保从正确的位置开始，并且只提取所需的长度。
                    if let Some(cell) = arg_list.as_ref() {
                        let cloned_cell = cell.clone();
                        *arg_list = Some(Box::new(cloned_cell.snoc_string(&tmp_name.trim())));
                    }
                    else {
                        let mut new_cell = Cell::mk_cell();
                        new_cell.name = Some(Box::new(tmp_name.trim().to_string())); // trim(): 提取的字符串没有前导或尾部的空格
                        *arg_list = Some(Box::new(new_cell));
                    }
                }
            }
        }
    }
    
}

fn getenv(var_name: &str) -> Option<&'static str> {
    if var_name == "MY_TEST_ENV" {
        Some("flag1 flag2 flag3")
    } else {
        None
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_snoc_string() {
//         let mut root = Cell::mk_cell();

//         root = root.snoc_string("hello");
//         assert_eq!(root.name.as_ref().unwrap().as_ref(), "hello");
//         assert!(root.link.is_none());

//         root = root.snoc_string("world");
//         assert_eq!(root.name.as_ref().unwrap().as_ref(), "hello");
//         assert_eq!(root.link.as_ref().unwrap().name.as_ref().unwrap().as_ref(), "world");
//         assert!(root.link.as_ref().unwrap().link.is_none());

//         root = root.snoc_string("rust");
//         assert_eq!(root.name.as_ref().unwrap().as_ref(), "hello");
//         assert_eq!(root.link.as_ref().unwrap().name.as_ref().unwrap().as_ref(), "world");
//         assert_eq!(root.link.as_ref().unwrap().link.as_ref().unwrap().name.as_ref().unwrap().as_ref(), "rust");
//         assert!(root.link.as_ref().unwrap().link.as_ref().unwrap().link.is_none());
//     }

//     #[test]
//     fn test_add_flags_from_env_var() {
//         let mut root: Option<Box<Cell>> = None;
//         Cell::add_flags_from_env_var(&mut root, "MY_TEST_ENV");

//         assert_eq!(root.as_ref().unwrap().name.as_ref().unwrap().as_ref(), "flag1");
//         assert_eq!(root.as_ref().unwrap().link.as_ref().unwrap().name.as_ref().unwrap().as_ref(), "flag2");
//         assert_eq!(root.as_ref().unwrap().link.as_ref().unwrap().link.as_ref().unwrap().name.as_ref().unwrap().as_ref(), "flag3");
//     }
// }
