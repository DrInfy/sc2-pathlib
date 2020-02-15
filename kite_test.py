import sc2pathlibp
import time
from typing import List
from math import floor


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


maze = read_maze("tests/empty10x10.txt")
pf = sc2pathlibp.PathFinder(maze)
pf.normalize_influence(1)
enemy_pos = (4, 0)
start_pos = (5, 5)
pf.add_influence_walk([enemy_pos], 100, 7)
end_result = pf.find_low_inside_walk(start_pos, enemy_pos, 5)
print(end_result)
end_point = end_result[0]
start_point_int = (floor(start_pos[0]), floor(start_pos[1]))
end_point_int = (floor(end_point[0]), floor(end_point[1]))
pf.plot([start_point_int, end_point_int], resize=40)

input("Press Enter to continue...")
