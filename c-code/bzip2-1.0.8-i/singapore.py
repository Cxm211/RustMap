import os

def count_lines_in_file(file_path):
    with open(file_path, 'r', encoding='utf-8', errors='ignore') as file:
        return sum(1 for line in file)

def count_lines_in_directory(directory):
    total_lines = 0
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith('.c') or file.endswith('.h'):
                file_path = os.path.join(root, file)
                lines = count_lines_in_file(file_path)
                print(f"{file_path}: {lines} lines")
                total_lines += lines
    return total_lines

if __name__ == "__main__":
    directory = '.'  # 这里可以修改为您想要统计的目录
    total_lines = count_lines_in_directory(directory)
    print(f"Total lines in .c and .h files: {total_lines}")
