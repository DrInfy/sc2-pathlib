# sc2-pathlib
Pathfinding and terrain analysis library for Starcraft 2 bot api in Rust

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### Prerequisites
#### Python

You will need Python 3.6 or newer.

#### Rust

You will need the nightly build of Rust:

https://www.rust-lang.org/tools/install


`rustup toolchain install
nightly`

Now Rust Nightly is installed but not activated.

To switch to nightly
globally, change the default with `rustup default nightly`:

```console
$ rustup default nightly
info: using existing install for 'nightly'
info: default toolchain set to 'nightly'

  nightly unchanged: rustc 1.9.0-nightly (02310fd31 2016-03-19)

```

Now any time you run `cargo` or `rustc` you will be running the
nightly compiler.

### Installing


Clone the sc2-pathlib repository and run `cargo build --release` in the sc2-pathlib directory. This should create a `sc2pathlib.so`(Linux) or a `sc2pathlib.dll`(Windows) file in `sc2-pathlib\target\release`. If on Windows, you need to rename the `sc2pathlib.dll` file to `sc2pathlib.pyd`. Copy `sc2pathlib.so`/`sc2pathlib.pyd` to the directory where your Python program resides to be able to import it as a Python library

#### Example
```
>>> import sc2pathlib
>>> sc2pathlib.find_path([[1,1,1,1],[0,0,0,1],[1,1,0,1],[1,1,1,1]], 0, 0, 2, 0)

([(0, 0), (0, 1), (0, 2), (0, 3), (1, 3), (2, 3), (3, 3), (3, 2), (3, 1), (2, 0)], 94142)
>>>
```
## Functions

### find_path
Uses A* pathfinding algorithm and returns a tuple containing the path as an array of tuples and the distance x 10000.
#### Parameters
`grid`: A two-dimensional array using 1 for pathable and 0 for obstacles. 
Example:
`[[1,1,1,1],[0,0,0,1],[1,1,0,1],[1,1,1,1]]`

`start_x`: Int with the x-value of the start position.
`start_y`: Int with the y-value of the start position
`x`: Int with the x-value of the goal position.
`y`: Int with the y-value of the goal position.
Example:
If the start position is (10,5) then `start_x` will be `10` and `start_y` will be 5.

### debug_path
Same function as above but returns a string containing the length of the array of tuples, the distance x 10000, the start position and the goal position.
#### Parameters
The same as `find_path`.
