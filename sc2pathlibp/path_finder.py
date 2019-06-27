from . sc2pathlib import PathFind
#from . import _sc2pathlib
#import sc2pathlib
import numpy as np
from typing import Union, List, Tuple
from math import floor

class PathFinder():
    def __init__(self, maze: Union[List[List[int]], np.array]):
        """ 
        pathing values need to be integers to improve performance. 
        Initialization should be done with array consisting values of 0 and 1.
        """
        self._path_find = PathFind(maze)
        self.heuristic_accuracy = 0
    
    def normalize_influence(self, value: int):
        """ 
        Normalizes influence to integral value.    
        Influence does not need to be calculated each frame, but this quickly resets
        influence values to specified value without changing available paths.
        """
        self._path_find.normalize_influence(value)
    
    @property
    def width(self):
        self._path_find.width
    
    @property
    def height(self):
        self._path_find.height

    @property
    def map(self):
        self._path_find.map

    def find_path(self, start: (float, float), end: (float, float)) -> Tuple[List[Tuple[int, int]], float]:
        start_int = (floor(start[0]), floor(start[1]))
        end_int = (floor(end[0]), floor(end[1]))
        return self._path_find.find_path(start_int, end_int, self.heuristic_accuracy)
    
    def find_path_influence(self, start: (float, float), end: (float, float)) -> (List[Tuple[int, int]], float):
        start_int = (floor(start[0]), floor(start[1]))
        end_int = (floor(end[0]), floor(end[1]))
        return self._path_find.find_path_influence(start_int, end_int, self.heuristic_accuracy)

    def safest_spot(self, destination_center: (float, float), walk_distance: float) -> (Tuple[int, int], float):
        destination_int = (floor(destination_center[0]), floor(destination_center[1]))
        return self._path_find.lowest_influence_walk(destination_int, walk_distance)
    
    def lowest_influence_in_grid(self, destination_center: (float, float), radius: int) -> (Tuple[int, int], float):
        destination_int = (floor(destination_center[0]), floor(destination_center[1]))
        return self._path_find.lowest_influence(destination_int, radius)
    
    def add_influence(self, points: List[Tuple[float, float]], value: float, distance: float):
        list = []
        for point in points:
            list.append((floor(point[0]), floor(point[1])))
        
        self._path_find.add_influence(list, value, distance)

    def add_influence_walk(self, points: List[Tuple[float, float]], value: float, distance: float):
        list = []
        for point in points:
            list.append((floor(point[0]), floor(point[1])))
        
        self._path_find.add_walk_influence(list, value, distance)

    def plot(self, path: List[Tuple[int, int]]):
        """
        requires opencv-python
        """
        import cv2
        image = np.array(self._path_find.map, dtype = np.uint8)
        for point in path:
            image[point] = 255
        image = np.rot90(image, 1)
        resized = cv2.resize(image, dsize=None, fx=4, fy=4)
        cv2.imshow(f"influence map", resized);
        cv2.waitKey(1);