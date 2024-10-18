#!/bin/bash
input_file="/home/kali/alias-limit/artifact/18-lifetimes-rosta/valid_subfolders_numbered.md"
output_file="/home/kali/alias-limit/artifact/18-lifetimes-rosta/valid_subfolders_numbered_with_codeblocks.md"

# 创建或清空输出文件
> "$output_file"

while IFS= read -r line
do
  echo "$line" >> "$output_file"
  
  # 检查是否以 "#" 开头，也就是是否是一个标题
  if [[ "$line" =~ ^# ]]; then
    echo -e "```rust\n\n```" >> "$output_file"
  fi
done < "$input_file"




# input_file="/home/kali/alias-limit/artifact/18-lifetimes-rosta/valid_subfolders.md"
# temp_file="/tmp/valid_subfolders_temp.md"

# # 清空临时文件
# > "$temp_file"

# while IFS= read -r line
# do
#   # 去掉尾部的'/'
#   line=${line%/}
#   # 在头部添加'# '构成MD标题
#   echo "# $line" >> "$temp_file"
#   # 在每行下面空5行
#   for i in {1..5}; do
#     echo "" >> "$temp_file"
#   done
# done < "$input_file"

# # 替换原文件
# mv "$temp_file" "$input_file"
