import sc2pathlibp
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
map = sc2pathlibp.Sc2Map(pathing, placement, height, playable_area)

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

input("Press Enter to continue...")
