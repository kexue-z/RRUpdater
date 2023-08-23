#!/bin/bash

# 获取当前目录
current_dir=$(pwd)


for file in "$current_dir"/*; do
  if [ -f "$file" ]; then

    filename=$(basename "$file")
    extension="${filename##*.}"


    if [ "$extension" == "exe" ]; then
      target_file="${filename%.exe}.zip"
    else
      target_file="$filename.tar.gz"
    fi

    # 压缩文件
    if [ "$extension" == "exe" ]; then
      zip -q "$target_file" "$file"
    else
      tar -czf "$target_file" "$file"
    fi

  fi
done