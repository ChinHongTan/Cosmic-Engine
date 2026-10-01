# Cosmic Engine

Cosmic is an engine I built from the ground up with Rust. The engine took me about one and a half months of work, and was built during my semester break.

Cosmic is stabilising at around 1900 - 2000 rating on Lichess. Its primary limitation lies in evaluation function and raw search speed, when compared to Stockfish.
Constants are common defaults from engine-programming practice. Each feature was verified with an SPRT, but the values themselves aren't tuned.

The parameters can be tuned, together with the evaluation function, but I will leave them as is for now.

No extra evaluation functions are coded, except PeSTO's Evaluation Function. I am planning to replace it with NNUE in the future.

Another feature that will drastically improve the search is to implement bitboards, but it involves major code changes, so is not implemented for now.

## Motivation and Background
I used to play chess regularly during my free time, and had a rating of close to 900 on chess.com at the highest point.
This project was created as a way for me to get into what low-level programming feels like, how to write code and algorithms that are faster and more efficient.
There is still a lot of room for improvement, and more features are planned in the future!

### Play on Lichess
When the bot is online, the engine is playable live on Lichess:
https://lichess.org/@/Cosmic_Engine/
The engine only accepts blitz games, standard variant.

## Features
Cosmic utilises a lot of features found on Chess Programming Wiki. It is a really detailed website if anyone is interested in making their own engine,
and I highly recommend it.

Listed in the order they were built.

### 1. Board and rules (August 2026)
- 8×8 board representation, with moves for every piece, including sliders blocked by other pieces
- FEN parsing
- Pawn double pushes, en passant and promotion
- Castling, with castling rights tracked through captures and rook/king moves
- Legal move generation (moves that leave the king in check are filtered out)
- Checkmate, stalemate, insufficient material and the 50-move rule

### 2. Making it fast (early September)
- Zobrist hashing, updated incrementally on every move
- Make / unmake moves in place instead of copying the board
- Attack detection by casting rays outward from a square, instead of scanning all 64 squares
- Reusing move lists to avoid allocations
- Perft test suite to verify the move generator

### 3. Search (September)
- Material evaluation
- Negamax search
- Alpha-beta pruning
- Quiescence search
- Piece-square tables
- Iterative deepening
- Transposition table (TT), with mate scores adjusted by ply
- UCI protocol, with `info` output
- Threefold repetition detection
- Check extensions

### 4. Strength, each change measured with an SPRT (late September)
- Time management: soft/hard limits, iteration-time prediction, best-move stability
- Null-move pruning (NMP)
- Killer moves
- PeSTO's evaluation function (tapered middlegame/endgame tables)
- Late move reductions (LMR)
- History heuristic
- Principal variation search (PVS) and aspiration windows

See the [development log](#development-log) for the measured result of each change.

It also supports UCI Protocol. Supported commands are: `uci`, `isready`, `ucinewgame`, `position` and `go`.

## Setup
### Building
To compile the engine locally:
```shell
cargo build --release
```

The resulting binary will be located in `target/release/cosmic`.

## Testing
### Correctness
```shell
cargo test
```
This runs:
- a perft suite, which counts every legal position to a fixed depth from standard test positions and compares against the published numbers
- a check that the incrementally updated Zobrist hash matches a full recomputation after every make and unmake
- repetition and castling-rights edge cases

### Strength
Every change in phase 4 was tested by playing the new version against the previous accepted version with [fastchess](https://github.com/Disservin/fastchess), using a sequential probability ratio test (SPRT):

- Time control: 5 seconds + 0.05 seconds per move
- Openings: `8moves_v3.pgn` from the [Stockfish books repository](https://github.com/official-stockfish/books), in random order
- Bounds: `elo0=0 elo1=5`, `alpha=0.05 beta=0.05`, which asks "is this a gain of at least about 5 Elo?" For changes kept unless they hurt, the non-regression bounds `elo0=-5 elo1=0` were used instead.
- Hardware: Apple M2, 4 games at a time

```shell
./fastchess/fastchess -engine cmd=./engines/new name=new -engine cmd=./engines/old name=old \
  -each proto=uci tc=5+0.05 -rounds 10000 -concurrency 4 \
  -openings file=8moves_v3.pgn format=pgn order=random \
  -sprt elo0=0 elo1=5 alpha=0.05 beta=0.05
```

## Development log
Results of each phase 4 change against the version before it, in the order they were tested.

| Change | Elo | Games | Result |
|---|---|---|---|
| Time management: iteration-time prediction and best-move stability | +11.25 ± 7.13 | 6,764 | accepted |
| Time extension when the score drops between iterations | −0.26 ± 2.67 | 20,000 | rejected |
| Null-move pruning | +68.16 ± 19.13 | 986 | accepted |
| Killer moves | +90.55 ± 21.59 | 718 | accepted |
| PeSTO's evaluation function | +186.66 ± 32.40 | 440 | accepted |
| Late move reductions | not recorded | not recorded | accepted |
| History heuristic | +17.19 ± 8.87 | 3,742 | accepted |
| Passed-pawn bonus | −13.92 ± 9.38 | 3,746 | rejected |
| PVS alone | −2.13 ± 4.66 | 13,860 | not a gain |
| PVS and aspiration windows | +5.64 ± 6.44 | 7,634 | kept (non-regression test passed) |

Notes:
- The time management result was measured against the first version with soft and hard time limits, not against the original engine, and its parts were not tested separately.
- The killer moves result also includes a speedup to `evaluate`, which was tested together with it.
- The score-drop time extension had no measurable effect, likely because the engine only completes a few iterations per move at this time control. It is kept on the `experiment/score-drop` branch.
- The passed-pawn bonus made the engine weaker. PeSTO's pawn tables already reward advanced pawns, and those are usually passed, so the bonus counted the same thing twice. It is kept on the `passed-pawn-points` branch.

## Roadmap
Some features that are considered but not yet implemented due to time constraints:
- NNUE network
- Parameter tuning
- bitboards

## References
This project wouldn't be possible without the help of the following projects:
- https://chessprogramming.org
- PeSTO evaluation (Ronald Friederich): https://www.talkchess.com/forum3/viewtopic.php?f=2&t=68311&start=19
- https://github.com/Disservin/fastchess
- https://github.com/official-stockfish/stockfish