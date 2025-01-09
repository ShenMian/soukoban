# soukoban

[![docs.rs](https://img.shields.io/docsrs/soukoban)](https://docs.rs/soukoban)
[![Test status](https://img.shields.io/github/actions/workflow/status/ShenMian/soukoban/test.yml?label=test)](https://github.com/ShenMian/soukoban/actions/workflows/test.yml)
[![Code coverage](https://img.shields.io/codecov/c/github/ShenMian/soukoban)](https://app.codecov.io/gh/ShenMian/soukoban)

A library provides the implementation of some algorithms and data structures related to [Sokoban].

## Features

- **Level**
  - **Zero-allocation lazy parsing**: Parses levels lazily from an in-memory string without memory allocations except for level creation.
  - **Lazy stream parsing**: Parses levels lazily from a stream.
  - **Map reconstruction**: Reconstructs the map from the solution.
  - **Normalization**: Removes elements from the map that are not relevant to the solution.
  - **RLE support**: Enables loading of levels encoded in Run-Length Encoding (RLE) format.
- **Solution**
  - **Reversal move handling**: Automatically interprets reversal moves as undo actions.
  - **Metrics calculation**: Computes metrics such as `box_lines`, `box_changes`, `pushing_sessions`, and `player_lines`.
- **Pathfinding**: Finds the optimal player path to push a box to a position.
- **Deadlock detection**: Detects static deadlocks and freeze deadlocks.

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.

The level files in the `assets` directory are licensed solely under
their respective licenses, available in the `LICENSE` file in the directory.

[sokoban]: https://en.wikipedia.org/wiki/Sokoban
