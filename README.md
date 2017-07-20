Modelling n-queens problem as propositional formula and solve it with DPLL 
------------
[![Build Status](https://travis-ci.org/erohkohl/n-queens-sat.svg?branch=master)](https://travis-ci.org/erohkohl/n-queens-sat) [![codecov](https://codecov.io/gh/erohkohl/n-queens-sat/branch/master/graph/badge.svg)](https://codecov.io/gh/erohkohl/n-queens-sat)

This project provides a simple implementation of the *Davis, Putnam, Logemann* and *Loveland* algorithm 
(see [DPLL](https://en.wikipedia.org/wiki/DPLL_algorithm)) in the modern systems programming language 
[Rust](https://www.rust-lang.org/en-US/). The DPLL algorithm decides whether a propositional 
formula in *conjunctive normal form* (CNF) is satisfiable and returns a satisfiable assignment. Therefore it extends the 
depth first search with *unit propagation* (UP), if a literal must be true in the current partial assignment. You
can find my implementation of the DPLL algorithm in the Rust-file 
[src/logic/sat.rs](https://github.com/erohkohl/n-queens-sat/blob/master/src/logic/sat.rs). Furthermore the file 
[src/model/n_queens.rs](https://github.com/erohkohl/n-queens-sat/blob/master/src/model/n_queens.rs) 
contains the modelling of the n-queens problem as propositional formula. The following listing shows one possible
solution of the four-queens problem.

```bash
. Q . . 
. . . Q 
Q . . . 
. . Q . 
```

This implementation is not able to solve six-queens problem in acceptable time, because it's
data structures and UP is not fast enough.

#### Future work

- Improve performance of UP and data structures to solve eight-queens problem in appropriate time
- Add *Conflict-Driven Clause Learning* (see [CDCL](https://en.wikipedia.org/wiki/Conflict-Driven_Clause_Learning)) 
to DPLL algorithm
