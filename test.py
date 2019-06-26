import sc2pathlib
import time
from typing import List

def read_maze(file_name: str) -> List[List[int]]:
    with open(file_name, 'r') as text:
        m = text.read()
    lines = m.split('\n')
    final_maze = []
    for y in range(0, len(lines[0])):
        maze_line = []
        final_maze.append(maze_line)
        for x in range(0, len(lines)):
            maze_line.append(int(lines[x][y]))

    return final_maze

maze = read_maze("tests/maze4x4.txt")
pf = sc2pathlib.PathFind(maze)
print(pf.map)
print(pf.width)
print(pf.height)

print(pf.find_path((0,0), (0,2)))
pf.normalize_influence(100)
print(pf.lowest_influence((2,2), 5))
print(pf.find_path((0,0), (0,2)))

maze = read_maze("tests/AutomatonLE.txt")
pf = sc2pathlib.PathFind(maze)
pf.normalize_influence(10)

result = pf.find_path((32, 51),(150, 129), 0)
print(f"path distance: {result[1]} for path: {result[0]}")

result = pf.find_path((32, 51),(150, 129), 1)
print(f"path distance: {result[1]} for path: {result[0]}")

result = pf.find_path((32, 51),(150, 129), 2)
print(f"path distance: {result[1]} for path: {result[0]}")

result = pf.find_path_influence((32, 51),(150, 129), 0)
print(f"path distance: {result[1]} for path: {result[0]}")

result = pf.find_path_influence((32, 51),(150, 129), 1)
print(f"path distance: {result[1]} for path: {result[0]}")

result = pf.find_path_influence((32, 51),(150, 129), 2)
print(f"path distance: {result[1]} for path: {result[0]}")

expansions = [(29, 65), (35, 34), 
(63, 26), (56, 65),
(98, 26), (80, 66),
(33, 105), (129, 28),
(54, 151), (150, 74),
(103, 113), (85, 153),
(127, 114), (120, 153),
(148, 145), (154, 114)]

total_distance = 0
ns_pf = time.perf_counter_ns()
pf.normalize_influence(100)

for pos1 in expansions:
    for pos2 in expansions:
        result = pf.find_path_influence(pos1, pos2, 0)
        total_distance += result[1]

ns_pf = time.perf_counter_ns() - ns_pf

print(f"pathfinding took {ns_pf / 1000 / 1000} ms. Total distance {total_distance}")
print(f"noraml influence: {pf.normal_influence}")