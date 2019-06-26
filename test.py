import sc2pathlib
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
result = pf.find_path((32, 51),(150, 129), False, 0)
print(f"path distance: {result[1]} for path: {result[0]}")

result = pf.find_path((32, 51),(150, 129), False, 1)
print(f"path distance: {result[1]} for path: {result[0]}")

result = pf.find_path((32, 51),(150, 129), False, 2)
print(f"path distance: {result[1]} for path: {result[0]}")

result = pf.find_path((32, 51),(150, 129), True, 0)
print(f"path distance: {result[1]} for path: {result[0]}")

result = pf.find_path((32, 51),(150, 129), True, 1)
print(f"path distance: {result[1]} for path: {result[0]}")

result = pf.find_path((32, 51),(150, 129), True, 2)
print(f"path distance: {result[1]} for path: {result[0]}")