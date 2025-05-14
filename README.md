<p align="center">
  <img src="https://github.com/user-attachments/assets/02acf234-bde7-46e2-8cea-acf9efbc7bad" />
</p>
<hr/>
Rust based chess engine currently under development.

## Roadmap
**v0.0.0** - done
- board representation
  - legal move generator
  - draw detection
  - checkmate detection
- functional negamax and quiescence search
- full uci compliance
- strength testing
  - cutechess tournament script
  - STS based rating

 **v0.1.0** - done
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
