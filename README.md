# Game-of-Life-Rust

 * Anthony Gatti - 10/15/23
 * Description: Conway's game of life is a zero-player game where
   the initial input determines its evolution. To start a game one
   must create an initial board and then observe how it evolves over
   a set period of time.
 * Rules: 
   1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
   2. Any live cell with two or three live neighbours lives on to the next generation.
   3. Any live cell with more than three live neighbours dies, as if by overpopulation.
   4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
 * Input: File name of the configuration, wrap or nowrap, hide/show, if show can choose slow, med, or fast.
 * Output: If hide prints the final board and the total time to run, 
   if show outputs each iteration of the board and the total time to run.

   Example run commands:
   cargo build
   cargo run gliderGun.txt wrap show fast
   cargo run gliderGun.txt nowrap hide 
