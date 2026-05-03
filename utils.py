import os
import sys

import sys
import os
import PyQt5.QtWidgets as QtWidgets
import PyQt5.QtCore as QtCore
import PyQt5.QtGui as QtGui
import random
import re
import time
import pathlib as pl
import pygetwindow
import keyboard
from collections import deque
import threading
import json
from datetime import datetime

# ==================== 获取程序目录（支持打包） ====================
def get_base_dir():
    """获取程序所在目录（兼容 PyInstaller 打包）"""
    if getattr(sys, 'frozen', False):
        # 打包后的可执行文件
        return os.path.dirname(sys.executable)
    else:
        # 源文件运行
        return os.path.dirname(os.path.abspath(__file__))
