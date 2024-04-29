# Merge3

A game of merging tiles. 

# Goals

- Implement a game in Rust  
  - [] game state (position, tile, board)
  - [] game logic
  - [] scoring
- Develop an AI to play the game

- Explore game design space
  - diagonal merging
  - unconstrained swapping  
  - see next tiles (like in tetris)  
  

# Gameplay

A 5x5 board is populated by tiles with values that are powers of 2.

Player interacts with a board by swapping a pair of neighboring tiles.

When three or more tiles with the same value are in a row or column, they merge into a single tile with the value of the sum of the merged tiles.

A player can only swap tiles that result in a merge.

When there are no more swaps possible, the game is over.

Each merge is scored by the value of the merged tiles.

When tiles are merged, the resulting tile is placed in the position of the first tile in the merge. The other cells become empty. If there are tiles above an empty cell the gravity pulls them down. If there are no tiles above empty cells then an empty cell is filled with a random new tile (it's value is 2,4,8, or 16).


# Implementation

A game loop that takes input from the player and updates the game state. 

## Game state

- board: 5x5 array of tiles
- board state: WaitForInput, InputReceived, MaybeMergeable, EmptyCells, GameOver

## Game logic

WaitForInput state: (board)
  - wait for input, when received:
    - set state = InputReceived(pair)

InputReceived state: (board, pair)
    -check if input is valid == board after swap is mergeable:
        - yes => swap and set state = MaybeMergeable (board)
        - no => do nothing, set state = wait for the next input

MaybeMergeable state: (board)
    - check if board is mergeable:
      - yes => merge tiles, update score, set state = EmptyCells
      - no => check if any possible swaps:
          - yes => set state = WaitForInput  
          - no => set state = GameOver    

EmptyCells state: (board)
  - gravity?
    - yes => apply gravity, check if there are any empty cells:
      - yes => fill empty cells with random tiles and set state = MaybeMergeable
    - no => fill empty cells with random tiles and set state = MaybeMergeable


Inspired by [Exponentile](https://www.bellika.dk/exponentile)