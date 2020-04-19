from .sc2pathlib import Map
import numpy as np


class Sc2Map:
    def __init__(self, 
        pathing_grid: np.ndarray,
        placement_grid: np.ndarray,
        height_map: np.ndarray,
        playable_area: 'sc2.position.Rect'):
        self.height_map = height_map
        self._map = Map(
            np.swapaxes(pathing_grid, 0, 1),
            np.swapaxes(placement_grid, 0, 1),
            np.swapaxes(height_map, 0, 1), 
            playable_area.x, 
            playable_area.y, 
            playable_area.x + playable_area.width, 
            playable_area.y + playable_area.height
            )

        # self._map = Map(
        #     Sc2Map._convert_pathing_grid(pathing_grid),
        #     Sc2Map._convert_pathing_grid(placement_grid),
        #     Sc2Map._convert_height_grid(height_map), 
        #     playable_area.x, 
        #     playable_area.y, 
        #     playable_area.x + playable_area.width, 
        #     playable_area.y + playable_area.height
        #     )
    
    @staticmethod
    def _convert_pathing_grid(grid: np.ndarray):
        # pathing = grid / 255
        pathing = np.swapaxes(grid, 0, 1)
        def mapping(x):
            return 1-x
        
        # pathing = np.vectorize(mapping)(pathing)

        return pathing.tolist()
    
    @staticmethod
    def _convert_height_grid(grid: np.ndarray):
        pathing = np.swapaxes(grid, 0, 1)

        # pathing = np.vectorize(pathing)

        return pathing.tolist()
    
    def plot(self, image_name: str = "map", resize: int = 4):
        """
        Uses cv2 to draw current pathing grid.
        
        requires opencv-python

        :param path: list of points to colorize
        :param image_name: name of the window to show the image in. Unique names update only when used multiple times.
        :param resize: multiplier for resizing the image
        :return: None
        """
        import cv2
        # image = np.array(self._map.ground_pathing, dtype=np.uint8)
        
        # image = np.array(self.height_map, dtype=np.uint8)
        image = np.array(self._map.draw_climbs(), dtype=np.uint8)

        image = np.rot90(image, 1)
        # image = np.multiply(image, 250)
        resized = cv2.resize(image, dsize=None, fx=resize, fy=resize, interpolation=cv2.INTER_NEAREST)
        cv2.imshow(image_name, resized)
        cv2.waitKey(1)