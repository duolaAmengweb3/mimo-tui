---
name: python-style
description: When writing Python code, always include type hints and a docstring.
triggers:
  - python
  - .py
  - django
  - fastapi
---

Whenever you write or edit Python files in this workspace:

1. Add type hints to **all** function signatures (parameters and return type).
2. Include a one-line docstring for every public function.
3. Prefer `pathlib.Path` over `os.path`.
4. Use `dataclasses` or `pydantic.BaseModel` over plain dicts for structured data.
5. Test files go under `tests/` with `pytest`-style assertions.

Example function template:

```python
def foo(bar: int, baz: str) -> bool:
    """One-line summary of what foo does."""
    return baz.startswith(str(bar))
```
