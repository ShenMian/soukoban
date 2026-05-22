# Benchmark

## Environment

### Hardware & Operating System

- **Date of Execution**: 2026-05-01.
- **CPU**: AMD EPYC 9634.
- **Memory**: 1.5 TB.
- **Operating system**: Rocky Linux 9.5.

### Solver Configuration

- **Time limit per level**: 10 minutes.
- **Strategy**: Greedy Best-First Search (GBFS).
- **Heuristic**: Simple Lower Bound.
- **Deadlock detection**: Static deadlock, freeze deadlock.

## Dataset

The level collections comes from the "Large Test Suite" level set curated by the [Sokoban Solver Statistics](https://sourceforge.net/projects/sokoban-solver-statistics/) project, containing a total of 3272 levels.

![Size distribution](imgs/size_distribution.webp)

The test results for other top solvers can be viewed [here](https://sokoban-solver-statistics.sourceforge.io/statistics/LargeTestSuite/).

## Results

![Status distribution](imgs/status_distribution.webp)

![Box success rate](imgs/box_success_rate.webp)

![Floor success rate](imgs/floor_success_rate.webp)

![Cumulative solved](imgs/cumulative_solved.webp)

![Solve time](imgs/solve_time.webp)
