import sys

if "python-sc2" not in sys.path:
    sys.path.insert(1, "python-sc2")


from sc2pathlib.mappings import MapType
import sc2pathlib
import time
from typing import List
import numpy as np
import time

def read_maze(file_name: str) -> List[List[int]]:
    with open(file_name, "r") as text:
        m = text.read()
    lines = m.split("\n")
    final_maze = []
    for y in range(0, len(lines[0])):
        maze_line = []
        final_maze.append(maze_line)
        for x in range(0, len(lines)):
            maze_line.append(int(lines[x][y]))
    return final_maze


class Rect:
    def __init__(self, x: int, y: int, width: int, height: int):
        self.x = x
        self.y = y
        self.width = width
        self.height = height

# playable_area = Rect(2, 2, 38, 38) 
# maze = read_maze("tests/choke.txt")
# map = sc2pathlibp.Sc2Map(maze, maze, maze, playable_area)
# print(f"Choke lines found: {len(map.chokes)}")

# map.plot("cliffs")
# map.plot_chokes("chokes")
# input("Press Enter to continue...")

map_name = "AutomatonLE"
pathing = np.load(f"tests/{map_name}_pathing.npy")
placement = np.load(f"tests/{map_name}_placement.npy")
height = np.load(f"tests/{map_name}_height.npy")

playable_area = Rect(18, 16, 148, 148)  # AutomatonLE
ns_pf = time.perf_counter_ns()
map = sc2pathlib.Sc2Map(pathing, placement, height, playable_area)

print("Path distance: " + str(map.find_path(MapType.Ground, (32, 51), (150, 118))[1]))

ns_pf = time.perf_counter_ns() - ns_pf
print(f"Creating map object took {ns_pf / 1000 / 1000} ms.")

print(map.overlord_spots)
map.plot("cliffs")
map.plot_chokes("chokes")
print(f"Choke lines found: {len(map.chokes)}")
arr = []
for choke in map.chokes:
    arr.append(choke.main_line)
    # Available properties
    # choke.lines
    # choke.side1
    # choke.side2
    # choke.pixels
    # choke.min_length
print(arr)


ns_pf = time.perf_counter_ns()
map._map.calculate_zones([
    (29, 65), (35, 34),
    (63, 26), (56, 65),
    (98, 26), (80, 66),
    (33, 105), (129, 28),
    (54, 151), (150, 74),
    (103, 113), (85, 153),
    (127, 114), (120, 153),
    (148, 145), (154, 114)
])

ns_pf = time.perf_counter_ns() - ns_pf
print(f"Solving map zones took {ns_pf / 1000 / 1000} ms.")

ns_pf = time.perf_counter_ns()

map.add_influence_without_zones([1, 2], 1000)

ns_pf = time.perf_counter_ns() - ns_pf
print(f"Adding map influence took {ns_pf / 1000 / 1000} ms.")

map.plot_zones("zones")

ns_pf = time.perf_counter_ns()
map.clear_vision()
for i in range(1, 40):
    # vision_unit = VisionUnit(i % 3 == 0, i % 2 == 0, (i * 10 + 30, i * 10 + 30), (i % 5 + 7))
    map.add_vision_params(i % 3 == 0, False, (i * 3 + 30, (i % 10) * 7 + 30), 15)
    # map.add_vision_params(i % 3 == 0, i % 2 == 0, (i * 5 + 30, (i % 5) * 20 + 30), (i % 5 + 7))
map.calculate_vision()

ns_pf = time.perf_counter_ns() - ns_pf
print(f"Calculating vision took {ns_pf / 1000 / 1000} ms.")

map.plot_vision()

input("Press Enter to continue...")