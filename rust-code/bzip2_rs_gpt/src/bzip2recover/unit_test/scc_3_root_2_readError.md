我使用了Box<dyn std::io::Write>代替FILE*。这是Rust中一个常见的方式来处理可以写的对象。
对于文件I/O错误处理，我目前添加了一个panic!作为占位符，你可能需要替换它以使用类似writeError的Rust函数。
BYTES_OUT被声明为static mut，意味着它是一个可变的全局变量。在Rust中，直接使用可变的全局变量是不安全的，这就是为什么我们需要用unsafe代码块来修改它。这只是一个直接转化，你可能需要考虑使用更Rust idiomatic的方式来处理这个，例如使用单例模式或其他方法。
