# SIMPLE EQUATION SOLVER

This crate writes an equation to JSON, runs a Python SymPy solver, and reads the answer back.

## Setup

```bash
docker compose up -d --build
```

and then launch the app (default in `http://localhost:5000/`)

## Files

- `src/pyo3temp.rs` contains the Rust bridge.
- `python/solver.py` contains the SymPy solver.