import sys
from PyQt5 import QtWidgets
from app import KillSayApp

def main():
    app = QtWidgets.QApplication(sys.argv)
    window = KillSayApp()
    window.run()
    sys.exit(app.exec_())

if __name__ == "__main__":
    main()
