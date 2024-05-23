# Contributing to pyproject-fmt-rust

Thank you for your interest in contributing to pyproject-fmt-rust! There are
many ways to contribute, and we appreciate all of them. As a reminder, all
contributors are expected to follow our [Code of Conduct](CODE_OF_CONDUCT.md).

## Development Setup

### Building the Project

To work on the project:

1. Install Rust (preferably through [rustup](https://rustup.rs)).
2. Clone the repository.
3. Build the project and run the unit tests:
   ```bash
   cargo test
   ```


## License
By contributing to pyproject-rust-format, you agree that your contributions
will be licensed under the [MIT License](LICENSE).

Thank you for your contributions! If you have any questions or need further
assistance, feel free to reach out via GitHub issues.

## Tips

### Always recompiling PyO3

If you find PyO3 constantly recompiling (such as if you are running
rust-analyser in your IDE and cargo test in a terminal), the problem is that
PyO3 has a `build.rs` that looks for Python, and it will recompile if it is run
with a different PATH. To fix it, put the following in `.cargo/config.toml`:

```toml
[env]
PYO3_PYTHON = "./.venv/bin/python"
```
And make sure you have a `.venv` folder with Python in it. This will ensure all
runs use the same Python and don't reconfigure.
