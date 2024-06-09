# soukoban

[![docs.rs](https://img.shields.io/docsrs/soukoban)](https://docs.rs/soukoban)
[![Test status](https://img.shields.io/github/actions/workflow/status/ShenMian/soukoban/test.yml?label=test)](https://github.com/ShenMian/soukoban/actions/workflows/test.yml)
[![Code coverage](https://img.shields.io/codecov/c/github/ShenMian/soukoban)](https://app.codecov.io/gh/ShenMian/soukoban)

A library provides the implementation of some algorithms and data structures related to [Sokoban].

## Features

- **Level**
  - **Lazy Loading**: Loads the first n levels, or the nth level, in a long XSB string.
  - **Map Reconstruction**: Reverse build the level map from the solution.
  - **Normalization**: Remove elements from the map that are not relevant to the solution.
  - **RLE Support**: Enables loading of levels encoded in Run-Length Encoding (RLE) format.
- **Solution**
  - **Reversal Move Handling**: Automatically interprets reversal moves as undo actions.
  - **Metrics Calculation**: Computes metrics such as `box_lines`, `box_changes`, `pushing_sessions`, and `player_lines`.
- **Pathfinding**: Finds the optimal player path to push a box to a position.
- **Deadlock Detection**: Detects dead positions and freeze deadlocks.

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.

The level files in the `assets` directory are licensed solely under
their respective licenses, available in the `LICENSE` file in the directory.

[sokoban]: https://en.wikipedia.org/wiki/Sokoban
