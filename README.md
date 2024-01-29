Rust based chess engine currently under development.

## Roadmap
**v0.0.0**
- board representation
  - legal move generator
  - draw detection
  - checkmate detection
- functional negamax and quiescence search
- full uci compliance
- strength testing
  - cutechess tournament script
  - STS based rating

 **v0.1.0**
 - piece square tables
 - transposition tables

**v0.2.0**
 - move ordering
 - delta pruning

**0.3.0**
- multi-threading
- transposition table improvements

**0.4.0**
- better evaluation
  - king safety
  - piece acitivity
  - pawn structure

...

**v1.0.0**
- search parameter tuning
- multi-threaded search
- benchmarks
  - nps
  - perft
