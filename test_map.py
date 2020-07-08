import sc2pathlibp
import time
from typing import List
import numpy as np
import time

pathing = np.load("tests/AutomatonLE_pathing.npy")
placement = np.load("tests/AutomatonLE_placement.npy")
height = np.load("tests/AutomatonLE_height.npy")

class Rect:
    def __init__(self, x: int, y: int, width: int, height: int):
        self.x = x
        self.y = y
        self.width = width
        self.height = height

playable_area = Rect(18, 16, 148, 148)  # AutomatonLE
ns_pf = time.perf_counter_ns()
map = sc2pathlibp.Sc2Map(pathing, placement, height, playable_area)

ns_pf = time.perf_counter_ns() - ns_pf
print(f"Creating map object took {ns_pf / 1000 / 1000} ms.")

print(map.overlord_spots)
map.plot("cliffs")
map.plot_chokes("chokes")

input("Press Enter to continue...")
