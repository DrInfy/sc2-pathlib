# sc2-pathlib
Pathfinding and terrain analysis library for Starcraft 2 bot api in Rust

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### Prerequisites
#### Python

You will need Python 3.7 or newer.

#### Rust

You will need to install the latest Rust:

https://rustup.rs/

### Installing

Clone the sc2-pathlib repository and run `cargo build --release` in the sc2-pathlib directory. This should create a `libsc2pathlib.so`(Linux), a `sc2pathlib.dll`(Windows), or a `libsc2pathlib.dylib` file in `sc2-pathlib\target\release`. If on Windows, you need to rename the `sc2pathlib.dll` file to `sc2pathlib.pyd`.
On Linux or Mac OS, rename it to `sc2pathlib.so`.

Alternatively, you can run helper scripts:
 - `build.bat` on Windows
 - `linux_build.sh` on Linux
 - `mac_build.sh` on Mac OS

Copy `sc2pathlib.so`/`sc2pathlib.pyd` to the directory where your Python program resides to be able to import it as a Python library.

#### Example
```
>>> import sc2pathlib
>>> pf = sc2pathlib.PathFind([[1,1,1,1],[0,0,0,1],[1,1,0,1],[1,1,1,1]])
>>> pf.find_path((0, 0), (2, 0))

([(0, 0), (0, 1), (0, 2), (0, 3), (1, 3), (2, 3), (3, 3), (3, 2), (3, 1), (2, 0)], 9.414199829101562)
>>>
```

## PathFind
#### Parameters
`grid`: A two-dimensional array using 1 for pathable and 0 for obstacles.
Example:
`[[1,1,1,1],[0,0,0,1],[1,1,0,1],[1,1,1,1]]`

## Functions

### find_path
Uses A* pathfinding algorithm and returns a tuple containing the path as an array of tuples and the distance.
#### Parameters
`start`: Tuple with the x and y value of the start position.
`end`: Tuple with the x and y value of the end position.
`possible_heuristic`: Optional parameter with value between 0-2. Lower value uses less accurate heuristic for distance calculation for improved performance.

### find_path_influence
Same function as above but uses influence to calculate path and return influenced distance.
#### Parameters
The same as `find_path`.
