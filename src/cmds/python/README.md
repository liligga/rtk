# Python Ecosystem

> Part of [`src/cmds/`](../README.md) — see also [docs/contributing/TECHNICAL.md](../../../docs/contributing/TECHNICAL.md)

## Specifics

- `pytest_cmd.rs` uses a state machine text parser (no JSON available from pytest)
- `ruff_cmd.rs` uses JSON for check mode (`--output-format=json`) and text filtering for format mode
- `py_compile_cmd.rs` runs `python -m py_compile` and trims syntax/error output
- `pip_cmd.rs` auto-detects `uv` as a pip alternative and routes accordingly
- `python -m pytest`, `uv run pytest`, and `uv run python -m pytest` are rewritten by the hook registry to `rtk pytest`
- `python -m py_compile` and `uv run python -m py_compile` are rewritten by the hook registry to `rtk py-compile`
- `python3 -m mypy` is rewritten by the hook registry to `rtk mypy`

## Cross-command

- `ruff_cmd` is called by `cmds/js/lint_cmd` and `cmds/system/format_cmd` for Python projects
- `mypy_cmd` is called by `cmds/js/lint_cmd` when detecting Python type checking
