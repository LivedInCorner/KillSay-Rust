import os
import sys
from PyQt5 import QtWidgets, QtGui
from app import KillSayApp
from utils import get_base_dir

def main():
    app = QtWidgets.QApplication(sys.argv)
    icon_path = os.path.join(get_base_dir(), "icon.ico")
    app.setWindowIcon(QtGui.QIcon(icon_path))
    window = KillSayApp()
    window.setWindowIcon(QtGui.QIcon(icon_path))
    window.run()
    sys.exit(app.exec_())

if __name__ == "__main__":
    main()
