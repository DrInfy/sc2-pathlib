import shutil
import sys

version = sys.argv[1].replace(".", "")

shutil.move("./target/release/sc2pathlib.dll", "./artifacts/sc2pathlib.cp{}-win_amd64.pyd".format(version))
