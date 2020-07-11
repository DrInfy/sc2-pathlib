import os
import sys
import argparse
import numpy as np


if "python-sc2" not in sys.path:
    sys.path.insert(1, "python-sc2")

from sc2.game_info import GameInfo
from sc2.player import Bot, Computer
from sc2 import BotAI, maps, run_game, Race


class PullerBot(BotAI):
    async def on_step(self, iteration):
        if iteration == 1:
            game_info: GameInfo = self.game_info
            path_grid = game_info.pathing_grid
            placement_grid = game_info.placement_grid
            map_name = game_info.map_name

            with open(f"tests\\{map_name}_height.npy", 'wb') as f:
                np.save(f, game_info.terrain_height.data_numpy, False, False)
            with open(f"tests\\{map_name}_pathing.npy", 'wb') as f:
                np.save(f, path_grid.data_numpy, False, False)
            with open(f"tests\\{map_name}_placement.npy", 'wb') as f:
                np.save(f, placement_grid.data_numpy, False, False)
            print("MAPS SAVED")
        elif iteration > 1:
            await self.client.leave()



def main():
    parser = argparse.ArgumentParser(description="Save numpy map data")
    parser.add_argument("-m", "--map", help="map name to use.")
    args = parser.parse_args()

    run_game(
        maps.get(args.map),
        [Bot(Race.Random, PullerBot()),
        Computer(Race.Random)],
        realtime=False
    )

if __name__ == '__main__':
    main()
