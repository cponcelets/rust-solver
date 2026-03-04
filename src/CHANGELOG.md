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

## [0.1.0] - 2026-03-04

- Module AST:
  - Pretty print using format
- Module Constraint:
  - Add consistency methods: 
    - Valid tuple checking
    - Get first invalid position
  - make_extensional_from for helping constructing extensional constraints
  - snapshot (deep cloning)
- Module Domain:
  - add method to access absent and next
- Module Variable:
  - add Hash (using label) for vvalue 
- Module CSP:
  - add fn assign(&mut self, vvalue: VValue<T>);
- Module solver:
  - add module consistency with:
    - definition of types arc and cvalue
    - consistency.rs (front)
    - fc.rs for testing forward consistency (AC1)
    - revise for different revising algorithm
      - AC, AC and AC2001
    - scheme for different oriented consistency enforcement
      - Arc and Variable oriented
- Module test:
  - Add tests for valid tuples and variable oriented algorithm (AC1)
