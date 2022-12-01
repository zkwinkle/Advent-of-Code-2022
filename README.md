# Advent of Code 2022

## Structure
The data will be structured in a main file (`src/main.rs`) and one module for each day.
Those modules will just be named as `day1` and so on and will contain:
- `task.txt`: textual description if the task
- `testinput.txt`: Small input which can be used for fast algorithmic checks
- `input.txt`: File which contains the actual input data
- `mod.rs`: File containing the code for each day. Usually there will be the two public functions `task1` and `task2`.
