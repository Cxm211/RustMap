#!/bin/bash

# 遍历当前目录下的所有子目录
for dir in */ ; do
    echo "Processing directory: $dir"
    cd "$dir"
    c_file=$(ls *.c 2> /dev/null | head -n 1)
    if [ ! -z "$c_file" ]; then
        echo "Compiling and running $c_file..."
        gcc -fprofile-arcs -ftest-coverage "$c_file" -o program
        ./program
        gcov "$c_file"
    else
        echo "No C files found in $dir"
    fi
    cd ..
done

echo "Collecting coverage data..."
# 收集所有覆盖数据
lcov --capture --directory . --output-file coverage.info
echo "Generating HTML report..."
# 生成HTML报告
genhtml coverage.info --output-directory coverage_report
echo "Report generated in coverage_report directory."
