# Merge3

A game of merging tiles. 

# Goals

- Implement a game in Rust
  - Learn about game development
- Develop an AI to play the game

- Explore game design space
  - diagonal merging
  - unconstrained swapping

# Gameplay

A 5x5 board is populated by tiles with values that are powers of 2.
Player interacts with a board by swapping a pair of neighboring tiles.
When three or more tiles with the same value are in a row or column, they merge into a single tile with the value of the sum of the merged tiles.


Inspired by [Exponentile](https://www.bellika.dk/exponentile)