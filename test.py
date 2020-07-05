import sc2pathlibp
import time
from typing import List


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


maze = read_maze("tests/maze4x4.txt")
pf = sc2pathlibp.PathFinder(maze)
print(pf.map)
print(pf.width)
print(pf.height)

print(pf.find_path((0, 0), (0, 2)))
pf.normalize_influence(100)
print(pf.lowest_influence_in_grid((2, 2), 5))
print(pf.find_path((0, 0), (0, 2)))

maze = read_maze("tests/AutomatonLE.txt")
pf = sc2pathlibp.PathFinder(maze)
pf.normalize_influence(10)

pf.heuristic_accuracy = 0
result = pf.find_path((32, 51), (150, 129))
print(f"path distance 0: {result[1]} for path: {result[0]}")

pf.heuristic_accuracy = 1
result = pf.find_path((32, 51), (150, 129))
print(f"path distance 1: {result[1]} for path: {result[0]}")

pf.heuristic_accuracy = 2
result = pf.find_path((32, 51), (150, 129))
print(f"path distance 2: {result[1]} for path: {result[0]}")

pf.heuristic_accuracy = 0
result = pf.find_path_influence((32, 51), (150, 129))
print(f"path influenced distance 0: {result[1]} for path: {result[0]}")
pf.heuristic_accuracy = 1
result = pf.find_path_influence((32, 51), (150, 129))
print(f"path influenced distance 1: {result[1]} for path: {result[0]}")
pf.heuristic_accuracy = 2
result = pf.find_path_influence((32, 51), (150, 129))
print(f"path influenced distance 2: {result[1]} for path: {result[0]}")

expansions = [
    (29, 65), (35, 34),
    (63, 26), (56, 65),
    (98, 26), (80, 66),
    (33, 105), (129, 28),
    (54, 151), (150, 74),
    (103, 113), (85, 153),
    (127, 114), (120, 153),
    (148, 145), (154, 114)
]

total_distance = 0
count = 0
ns_pf = time.perf_counter_ns()
pf.normalize_influence(100)
pf.heuristic_accuracy = 2

for pos1 in expansions:
    for pos2 in expansions:
        result = pf.find_path(pos1, pos2, False)
        total_distance += result[1]
        count += 1

ns_pf = time.perf_counter_ns() - ns_pf


print(f"pathfinding took {ns_pf / 1000 / 1000} ms. Total distance {total_distance}")
print(f"pathfinding took {ns_pf / 1000 / 1000 / count} ms per path.")

ns_pf = time.perf_counter_ns()
pf.add_influence([(56, 65), (110, 28), (100, 98)], 150, 10, False)
ns_pf = time.perf_counter_ns() - ns_pf
print(f"adding influence took {ns_pf / 1000 / 1000} ms.")

pf.normalize_influence(100)

ns_pf = time.perf_counter_ns()
pf.add_influence_walk([(56, 65), (110, 28), (100, 98)], 150, 10, False)
ns_pf = time.perf_counter_ns() - ns_pf
print(f"adding influence by walking distance took {ns_pf / 1000 / 1000} ms.")

result = pf.find_path_influence((29, 65), (154, 114))
# print(pf.map)
# pf.reset()
# pf.normalize_influence(100)
pf.plot(result[0])
pf.create_block([(11.5, 11.5), (21.5, 21.5), (31.5, 31.5), (31.5, 31.5)], (2, 1))
pf.plot(result[0])
input("Press Enter to continue...")
