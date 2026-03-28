# wdl-lite-py

> A minimal WDL (Workflow Description Language) parser and linter written in Rust, exposed to Python via [PyO3](https://pyo3.rs/).

[![Build](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/Ayushd172005/wdl-lite-py)
[![PyO3](https://img.shields.io/badge/PyO3-0.22-blue)](https://pyo3.rs/)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/Python-3.8+-blue)](https://www.python.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

---

## Overview

`A-Mini-WDL-Parser-in-Rust-with-Python-Bindings` is a proof-of-concept project that bridges Rust's performance with Python's accessibility — the same goal as [Sprocket's `sprocket-py`](https://github.com/stjude-rust-labs/sprocket) at full scale.

It demonstrates:

- Parsing a WDL document from a Python string into structured types
- Running lint rules (snake_case naming, version validation) from Python
- Clean error propagation — Rust parse errors become Python exceptions
- Pythonic API design (properties, `__repr__`, idiomatic naming)

---

## Quick Start

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install maturin
pip install maturin

# Clone and build
git clone https://github.com/Ayushd172005/wdl-lite-py.git
cd wdl-lite-py
maturin develop
```

Then in Python:

```python
import wdl_lite

doc = wdl_lite.parse("""
version 1.2

task align_reads {
  command { bwa mem ref.fa reads.fastq }
}

workflow MyPipeline {
}
""")

print(doc.version)          # "1.2"
print(doc.task_names)       # ["align_reads"]
print(doc.workflow_names)   # ["MyPipeline"]

for d in wdl_lite.lint(doc):
    print(f"[{d.severity}] {d.message}")
# [warning] Workflow 'MyPipeline' should use snake_case (e.g. 'my_pipeline')
```

---

## API Reference

### `wdl_lite.parse(source: str) -> Document`

Parses a WDL source string and returns a `Document` object.

Raises `ValueError` (with a `ParseError:` prefix) if the source is invalid.

```python
doc = wdl_lite.parse("version 1.2\ntask hello { command { echo hi } }")
```

---

### `wdl_lite.lint(doc: Document) -> list[Diagnostic]`

Runs linting rules on a parsed `Document` and returns a list of `Diagnostic` objects.

```python
diagnostics = wdl_lite.lint(doc)
for d in diagnostics:
    print(d.severity, d.message)
```

---

### `Document`

| Property | Type | Description |
|---|---|---|
| `.version` | `str` | WDL version declared in the document (e.g. `"1.2"`) |
| `.task_names` | `list[str]` | Names of all `task` blocks |
| `.workflow_names` | `list[str]` | Names of all `workflow` blocks |

---

### `Diagnostic`

| Property | Type | Description |
|---|---|---|
| `.severity` | `str` | One of `"info"`, `"warning"`, `"error"` |
| `.message` | `str` | Human-readable description of the issue |

---

## Lint Rules

| Rule | Severity | Description |
|---|---|---|
| Unknown version | `warning` | Version is not one of `1.0`, `1.1`, `1.2` |
| Task not snake_case | `warning` | Task names should use `snake_case` |
| Workflow not snake_case | `warning` | Workflow names should use `snake_case` |
| Empty document | `info` | Document has no tasks or workflows defined |

---

## Error Handling

Parse errors are raised as `ValueError` with a descriptive message:

```python
try:
    wdl_lite.parse("this is not valid WDL")
except ValueError as e:
    print(e)
# ParseError: unexpected token 'this is not valid WDL' — did you forget 'version 1.x'?
```

---

## Running Tests

```bash
pip install pytest
pytest tests/
```

The test suite covers:

- Basic parse of version, tasks, and workflows
- Parse error on invalid input
- Lint rule: snake_case enforcement
- Lint rule: unknown version detection
- Lint info: empty document warning

---

## Project Structure

```
wdl-lite-py/
├── src/
│   └── lib.rs          # Rust implementation + PyO3 bindings
├── tests/
│   └── test_bindings.py  # Python test suite
├── Cargo.toml          # Rust dependencies
├── pyproject.toml      # Python build config (maturin)
└── README.md
```

---

## How It Works

The Rust side defines two internal structs (`ParsedDoc`, `LintDiag`) and a lightweight hand-written parser. PyO3 wraps these in `#[pyclass]` types (`Document`, `Diagnostic`) with `#[pymethods]` exposing Python-accessible properties and `__repr__`.

```
Python call: wdl_lite.parse(source)
        │
        ▼
  #[pyfunction] parse()  ←── PyO3 boundary
        │
        ▼
  parse_wdl(source)      ←── Pure Rust logic
        │
        ▼
  ParsedDoc { version, tasks, workflows }
        │
        ▼
  PyDocument { inner: ParsedDoc }   ←── Returned to Python
```

Rust errors (`Result<_, String>`) are mapped to `PyValueError` at the FFI boundary, so Python callers get standard exceptions.

---

## Relation to Sprocket

This project is a miniature version of what [`sprocket-py`](https://github.com/stjude-rust-labs/sprocket) will do at full scale. The real project will wrap:

- `wdl-grammar` — full lexer and parser
- `wdl-ast` — complete abstract syntax tree
- `wdl-lint` — all production lint rules
- `wdl-analysis` — semantic validation

This prototype validates the core FFI patterns (type wrapping, error propagation, Pythonic API design) that will be needed for the full implementation.

---

## License

MIT — see [LICENSE](./LICENSE).
