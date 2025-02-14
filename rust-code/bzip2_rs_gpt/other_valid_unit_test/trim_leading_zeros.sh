#!/bin/bash

# 脚本名称：trim_leading_zeros.sh

# 检查是否有一个参数传入
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <filename>"
    exit 1
fi

# 从命令行参数获取文件名
file=$1

# 使用xxd生成十六进制的转储，并找到第一个包含非零字节的行
first_non_zero_line=$(xxd -g 1 "$file" | grep -m 1 -v ' 00 00 00 00 00 00 00 00')

# 如果没有找到非零行，说明文件不包含连续的零字节块
if [ -z "$first_non_zero_line" ]; then
    echo "No leading zeros to trim."
    exit 0
fi

# 从找到的行中获取行号
line_no=$(echo "$first_non_zero_line" | awk '{print $1}' | tr -d ':')

# 计算要跳过的字节数（行号乘以每行字节数）
# xxd 默认一行16个字节
skip_bytes=$((16#"$line_no"))

# 使用dd命令跳过找到的零字节
dd if="$file" of="trimmed_$file" bs=1 skip=$skip_bytes status=none

# 报告操作结果
echo "Trimmed file created as: trimmed_$file"

