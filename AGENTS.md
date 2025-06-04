# Instructions for Codex Agents

## Formatting and Linting
- Always run `cargo fmt --all` to format Rust code before committing.
- Optionally, run `cargo clippy` to check for lints, but it is not required for PRs.

## Tests
- Run `cargo test` to ensure that unit and integration tests pass.
- If tests fail due to missing dependencies or environment limitations, note it in the PR.

## Running Rustlings
- Use `cargo run -- -V` to verify the Rustlings version.
- You can also run exercises or check them with `cargo run -- run <exercise>` or `cargo run -- check-all`.

