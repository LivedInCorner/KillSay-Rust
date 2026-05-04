#!/usr/bin/env python3
"""
版本号自动修改脚本
用法: python bump_version.py [major|minor|patch]
"""

import json
import re
import sys
from pathlib import Path

# 项目根目录
ROOT_DIR = Path(__file__).parent

# 需要修改的文件
FILES = {
    "package.json": ROOT_DIR / "package.json",
    "tauri.conf.json": ROOT_DIR / "src-tauri" / "tauri.conf.json",
    "Cargo.toml": ROOT_DIR / "src-tauri" / "Cargo.toml",
}


def read_version(file_path: Path) -> str:
    """读取文件中的版本号"""
    content = file_path.read_text(encoding="utf-8")
    
    if file_path.name == "Cargo.toml":
        match = re.search(r'version\s*=\s*"(\d+\.\d+\.\d+)"', content)
    else:
        match = re.search(r'"version"\s*:\s*"(\d+\.\d+\.\d+)"', content)
    
    if match:
        return match.group(1)
    raise ValueError(f"无法在 {file_path} 中找到版本号")


def write_version(file_path: Path, old_version: str, new_version: str):
    """写入新版本号"""
    content = file_path.read_text(encoding="utf-8")
    
    if file_path.name == "Cargo.toml":
        new_content = content.replace(
            f'version = "{old_version}"',
            f'version = "{new_version}"'
        )
    else:
        new_content = content.replace(
            f'"version": "{old_version}"',
            f'"version": "{new_version}"'
        )
    
    file_path.write_text(new_content, encoding="utf-8")


def bump_version(version: str, part: str) -> str:
    """递增版本号"""
    major, minor, patch = map(int, version.split("."))
    
    if part == "major":
        return f"{major + 1}.0.0"
    elif part == "minor":
        return f"{major}.{minor + 1}.0"
    elif part == "patch":
        return f"{major}.{minor}.{patch + 1}"
    else:
        raise ValueError(f"无效的版本部分: {part}")


def main():
    if len(sys.argv) < 2:
        print("用法: python bump_version.py [major|minor|patch]")
        print("  major - 主版本号 (1.0.0 -> 2.0.0)")
        print("  minor - 次版本号 (1.0.0 -> 1.1.0)")
        print("  patch - 补丁版本号 (1.0.0 -> 1.0.1)")
        sys.exit(1)
    
    part = sys.argv[1].lower()
    if part not in ("major", "minor", "patch"):
        print(f"错误: 无效的版本部分 '{part}'")
        sys.exit(1)
    
    # 读取当前版本
    current_version = read_version(FILES["package.json"])
    print(f"当前版本: {current_version}")
    
    # 计算新版本
    new_version = bump_version(current_version, part)
    print(f"新版本: {new_version}")
    
    # 确认
    confirm = input(f"确认将版本从 {current_version} 更新到 {new_version}? (y/N): ")
    if confirm.lower() != "y":
        print("已取消")
        sys.exit(0)
    
    # 更新所有文件
    for name, path in FILES.items():
        try:
            write_version(path, current_version, new_version)
            print(f"✓ 已更新 {name}")
        except Exception as e:
            print(f"✗ 更新 {name} 失败: {e}")
    
    print(f"\n版本已更新为 {new_version}")
    print("请记得提交并创建新的 git tag:")
    print(f"  git add -A")
    print(f'  git commit -m "chore: bump version to {new_version}"')
    print(f"  git tag v{new_version}")
    print(f"  git push origin main --tags")


if __name__ == "__main__":
    main()
