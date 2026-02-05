## [0.1.0] - 2026-02-01
### Added : base of CSP modules
- CSP core architecture (domains, variables, constraints)
- Local consistency and support checking
- Hypergraph density computation
- Unit tests

# [0.1.0] - 2026-02-05
- Renaming ExDom to SetDom
- SetDom: size O(1) and min/max with trailing
- SetDomIter: iterate on active values
- CartesianWalker uses SetDomIter
- Added unit tests for size, min, max, iter
- tests/csp_basic: Added consistency test