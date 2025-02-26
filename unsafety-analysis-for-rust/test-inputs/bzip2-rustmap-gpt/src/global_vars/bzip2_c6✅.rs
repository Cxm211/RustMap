// 在Rust中，静态变量是在整个程序执行期间都存在的，和C中的静态数组类似
// Int32 在C中可能是一个typedef，在Rust中我们通常使用i32来表示32位整数
pub static INCS: [i32; 14] = [1, 4, 13, 40, 121, 364, 1093, 3280,
                          9841, 29524, 88573, 265720,
                          797161, 2391484];
