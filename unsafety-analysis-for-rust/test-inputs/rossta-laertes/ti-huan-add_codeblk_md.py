input_file_path = "valid_subfolders_numbered.md"
output_file_path = "valid_subfolders_numbered_with_codeblocks.md"

with open(input_file_path, "r") as infile, open(output_file_path, "w") as outfile:
    for line in infile:
        if line.startswith("#"):
            # 写入标题，并在标题后添加一行带有五个空格的空行
            outfile.write(line)
            outfile.write("     \n")
            # 在代码块内添加两个空格行
            outfile.write("```rust\n     \n     \n```\n")
            # 在代码块和下一个标题之间添加带有五个空格的空行
            outfile.write("     \n     \n     \n     \n     \n")
        else:
            outfile.write(line)

print(f"Rust code blocks with empty lines have been added. Check the output file at {output_file_path}.")
