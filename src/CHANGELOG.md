## [0.1.0] - 2026-02-01
### Added : base of CSP modules
- CSP core architecture (domains, variables, constraints)
- Local consistency and support checking
- Hypergraph density computation
- Unit tests

## [0.1.0] - 2026-02-05
- Renaming ExDom to SetDom
- SetDom: size O(1) and min/max with trailing
- SetDomIter: iterate on active values
- CartesianWalker uses SetDomIter
- Added unit tests for size, min, max, iter
- Tests/csp_basic: Added consistency test

## [0.1.0] - 2026-02-12

- Module AST: extend constraint with:
  - Predicates, expressions and formula
  - ArithT, type compatible with arithmetic operators
  - Trait eval merging simple (Expr) and arithmetics (AExpr) expressions
  - Macros to ease expression constructions

- Cleaning Constraint interface
  - apply take an assignment as input
  - label method for pretty print
  - Intensional constraint gets a formula
    - constructors with automatic scope collector
- Update exVar for HashSet compatibility
- Module Solver
  - Module GAC, implementation of the first GAC algorithm
  - Example on domino