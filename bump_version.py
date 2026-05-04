#!/usr/bin/env python3
"""
版本号修改脚本
用法: python bump_version.py <version> [--write]

示例:
  python bump_version.py 1.0.0 --write
  python bump_version.py 2.1.0 --write
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


def validate_version(version: str) -> bool:
    """验证版本号格式"""
    return bool(re.match(r'^\d+\.\d+\.\d+$', version))


def main():
    if len(sys.argv) < 2:
        print("用法: python bump_version.py <version> [--write]")
        print("示例: python bump_version.py 1.0.0 --write")
        sys.exit(1)
    
    new_version = sys.argv[1]
    write_mode = "--write" in sys.argv
    
    if not validate_version(new_version):
        print(f"错误: 无效的版本号格式 '{new_version}'")
        print("版本号格式应为: x.y.z (例如 1.0.0)")
        sys.exit(1)
    
    # 读取当前版本
    current_version = read_version(FILES["package.json"])
    print(f"当前版本: {current_version}")
    print(f"目标版本: {new_version}")
    
    if current_version == new_version:
        print("版本号相同，无需修改")
        sys.exit(0)
    
    if not write_mode:
        print("\n预览模式，未修改任何文件")
        print(f"如需修改，请执行: python bump_version.py {new_version} --write")
        sys.exit(0)
    
    # 更新所有文件
    success = True
    for name, path in FILES.items():
        try:
            write_version(path, current_version, new_version)
            print(f"✓ 已更新 {name}")
        except Exception as e:
            print(f"✗ 更新 {name} 失败: {e}")
            success = False
    
    if success:
        print(f"\n版本已更新为 {new_version}")
        print("请记得提交并创建新的 git tag:")
        print(f"  git add -A")
        print(f'  git commit -m "chore: bump version to {new_version}"')
        print(f"  git tag v{new_version}")
        print(f"  git push origin main --tags")
    else:
        print("\n部分文件更新失败，请检查")
        sys.exit(1)


if __name__ == "__main__":
    main()
