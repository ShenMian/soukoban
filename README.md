# soukoban

[![docs.rs](https://img.shields.io/docsrs/soukoban)](https://docs.rs/soukoban)
[![Test status](https://img.shields.io/github/actions/workflow/status/ShenMian/soukoban/test.yml?label=test)](https://github.com/ShenMian/soukoban/actions/workflows/test.yml)
[![Code coverage](https://img.shields.io/codecov/c/github/ShenMian/soukoban)](https://app.codecov.io/gh/ShenMian/soukoban)

A library providing implementations of algorithms and data structures related to [Sokoban].

## Features

- **Solver**
  - **Search algorithms**: Supports [A\*], [IDA\*], [GBFS] and [BFS] search.
  - **Strategies**: Supports quick, push-optimal, and move-optimal strategies.
- **Level**
  - **Zero-allocation lazy parsing**: Parses levels lazily from an in-memory string without memory allocations except for level creation.
  - **Lazy stream parsing**: Parses levels lazily from a stream.
  - **Map reconstruction**: Reconstructs the map from the solution.
  - **Canonicalization**: Removes elements from the map that are not relevant to the solution.
  - **RLE support**: Enables loading of levels encoded in Run-Length Encoding (RLE) format.
  - **Symmetry transformations**: Supports rotating and flipping levels.
- **Actions**
  - **Reversal move handling**: Automatically interprets reversal moves as undo actions.
  - **Metrics calculation**: Computes metrics such as `box_lines`, `box_changes`, `pushing_sessions`, and `player_lines`.
  - **Symmetry transformations**: Supports rotating and flipping action sequences mapping.
- **Pathfinding**: Finds paths for box/player with support for different strategies.
- **Deadlock detection**: Detects static deadlocks and freeze deadlocks.

## Example

```rust
use std::str::FromStr as _;
use soukoban::{prelude::*, solver::*};

fn main() {
    // Create a sequence of actions from a LURD string
    let actions = Actions::from_str("R").unwrap();

    // Reconstruct the map from the actions
    let map = Map::with_actions(&actions).unwrap();

    // Print the reconstructed map
    //
    // #####
    // #@$.#
    // #####
    println!("{map}");

    // Search for a solution using A* algorithm
    let solution = Solver::new(map, Strategy::Quick)
        .search(Algorithm::AStar)
        .unwrap();

    // Verify the solution matches the original actions
    assert_eq!(solution, actions);
}
```

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.

The level files in the `assets` directory are licensed solely under
their respective licenses, available in the `LICENSE` file in the directory.

[sokoban]: https://en.wikipedia.org/wiki/Sokoban
[A\*]: https://en.wikipedia.org/wiki/A*_search_algorithm
[IDA\*]: https://en.wikipedia.org/wiki/Iterative_deepening_A*
[GBFS]: https://en.wikipedia.org/wiki/Best-first_search
[BFS]: https://en.wikipedia.org/wiki/Breadth-first_search
