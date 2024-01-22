## v0.1.1
- checkmate detection fix
- fifty move rule detection fix
- optimized compilation for runtime performance

Score of v0.1.1 vs v0.1.0: 553 - 436 - 11  [0.558] 1000 \
&nbsp; v0.1.1 playing White: 278 - 215 - 7  [0.563] 500 \
&nbsp; v0.1.1 playing Black: 275 - 221 - 4  [0.554] 500 \
&nbsp; White vs Black: 499 - 490 - 11  [0.504] 1000

Elo difference: 40.8 +/- 21.6, LOS: 100.0 %, DrawRatio: 1.1 % \
SPRT: llr 0 (0.0%), lbound -inf, ubound inf

Player: v0.1.1 \
&nbsp; "Draw by 3-fold repetition": 7 \
&nbsp; "Draw by timeout": 4 \
&nbsp; "Loss: Black loses on time": 221 \
&nbsp; "Loss: White loses on time": 215 \
&nbsp; "Win: Black loses on time": 135 \
&nbsp; "Win: Black mates": 112 \
&nbsp; "Win: White loses on time": 163 \
&nbsp; "Win: White mates": 143

STS rating: 1208 \
Hash: 128, Threads: 1, MoveTime: 1.0s

## v0.1.0
- transposition table
- piece square tables

Score of v0.1.0 vs v0.0.0: 1582 - 346 - 72  [0.809] 2000 \
&nbsp; v0.1.0 playing White: 774 - 181 - 45  [0.796] 1000 \
&nbsp; White vs Black: 939 - 989 - 72  [0.487] 2000

Elo difference: 250.8 +/- 18.8, LOS: 100.0 %, DrawRatio: 3.6 % \
SPRT: llr 0 (0.0%), lbound -inf, ubound inf

Player: v0.1.0 \
&nbsp; "Draw by 3-fold repetition": 12 \
&nbsp; "Draw by stalemate": 2 \
&nbsp; "Draw by timeout": 58 \
&nbsp; "Loss: Black loses on time": 165 \
&nbsp; "Loss: White loses on time": 181 \
&nbsp; "Win: Black loses on time": 774 \
&nbsp; "Win: White loses on time": 808

STS rating: 749 \
Hash: 128, Threads: 1, MoveTime: 1.0s

## v0.0.0
- board representation
  - legal move generator
  - draw detection
  - checkmate detection
- functional negamax and quiescence search
- full uci compliance
- strength testing
  - cutechess tournament script
  - STS based rating

STS rating 407 \
Hash: 128, Threads: 1, MoveTime: 1.0s
