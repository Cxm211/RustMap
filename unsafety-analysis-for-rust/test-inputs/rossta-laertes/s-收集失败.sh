#!/bin/bash

# 初始化一个空字符串，用于收集所有失败的例子
failed_examples=""

# 遍历目标目录下的所有文件夹
for dir in /home/kali/alias-limit/artifact/18-lifetimes-rosta/*; do
    # 检查是否是文件夹
    if [ -d "$dir" ]; then
        # 进入文件夹
        cd "$dir" || exit
        # 执行 cargo run，并将标准输出和错误输出重定向到/dev/null
        # 如果失败，则将文件夹名添加到failed_examples字符串中
        if ! cargo run > /dev/null 2>&1; then
            failed_examples+="$dir\n"
        fi
        # 返回原目录
        cd - || exit
    fi
done

# 打印所有失败的例子
echo -e "Failed examples:\n$failed_examples"
