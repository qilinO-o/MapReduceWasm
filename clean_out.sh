#!/bin/bash

# 定义源文件夹和目标文件夹
SOURCE_DIR="./out"
BACKUP_DIR="./out-back"

# 检查源文件夹是否存在
if [ ! -d "$SOURCE_DIR" ]; then
  echo "Error: Source directory '$SOURCE_DIR' does not exist."
  exit 1
fi

# 确保目标文件夹存在，如果不存在则创建
mkdir -p "$BACKUP_DIR"

# 获取当前时间并格式化为字符串（例如：20231012_153045）
CURRENT_TIME=$(date +"%Y%m%d_%H%M%S")

# 定义备份文件夹的名称
BACKUP_NAME="out_$CURRENT_TIME"

# 复制源文件夹到目标文件夹，并重命名
cp -r "$SOURCE_DIR" "$BACKUP_DIR/$BACKUP_NAME"

# 检查复制是否成功
if [ $? -eq 0 ]; then
  echo "Backup successful: '$SOURCE_DIR' copied to '$BACKUP_DIR/$BACKUP_NAME'"
else
  echo "Error: Failed to copy '$SOURCE_DIR' to '$BACKUP_DIR/$BACKUP_NAME'"
  exit 1
fi

# 清除源文件夹下的所有文件
rm -rf "$SOURCE_DIR"/*

# 检查清除是否成功
if [ $? -eq 0 ]; then
  echo "Cleared all files in '$SOURCE_DIR'"
else
  echo "Error: Failed to clear files in '$SOURCE_DIR'"
  exit 1
fi

echo "Script completed successfully."