from .sc2pathlib import Map
import numpy as np


class sc2Map:
    def __init__(self, 
        pathing_grid: np.ndarray,
        placement_grid: np.ndarray,
        height_map: np.ndarray,
        playable_area: 'sc2.position.Rect'):
        self._map = Map(pathing_grid, placement_grid, height_map, 
            playable_area.x, playable_area.y, 
            playable_area.x + playable_area.width, 
            playable_area.y + playable_area.height
            )