#!/bin/bash

# 源文件夹路径
source_dir="./target/wasm32-wasip1/release/"

# 目标文件夹路径
destination_dir="./wasm_file"

# 检查源文件夹是否存在
if [ ! -d "$source_dir" ]; then
  echo "源文件夹不存在: $source_dir"
  exit 1
fi

# 检查目标文件夹是否存在，如果不存在则创建
if [ ! -d "$destination_dir" ]; then
  mkdir -p "$destination_dir"
fi

# 复制文件
cp -r "$source_dir"/*.wasm "$destination_dir"

# 检查复制是否成功
if [ $? -eq 0 ]; then
  echo "文件复制成功！"
else
  echo "文件复制失败！"
fi