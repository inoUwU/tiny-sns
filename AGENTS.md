# Project Guidelines

## Project State

- This repository is design-first today. Treat the product behavior in [docs/specification.md](docs/specification.md), the schema in [docs/sns.dbml](docs/sns.dbml), the shared diagrams in [docs/mermaid_diagrams.md](docs/mermaid_diagrams.md), and the timeline notes in [Considerations/pull_push_timeline.md](Considerations/pull_push_timeline.md) as the current source of truth.
- The Rust implementation is still minimal. [backend/src/main.rs](backend/src/main.rs) is only a stub, so do not infer finished backend behavior from code that is not there yet.
- If a requested change affects product behavior or data shape, update the relevant documentation alongside code, or explicitly call out the doc/code mismatch.

## Build And Test

- The only runnable code today is the standalone Rust crate under [backend](backend).
- Run Rust commands from [backend](backend): `cargo build`, `cargo test`, `cargo run`.
- There is no workspace-level Cargo project at the repository root.
- Do not edit generated artifacts under `backend/target/`.

## Documentation Conventions

- Prefer linking to existing docs instead of restating them in new files.
- Keep shared Mermaid diagrams in [docs/mermaid_diagrams.md](docs/mermaid_diagrams.md). Do not introduce separate `.mmd` files for shared diagrams unless explicitly asked.
- Keep schema changes in [docs/sns.dbml](docs/sns.dbml). Use [docs/dbml_gap_analysis.md](docs/dbml_gap_analysis.md) when you need to explain why the schema changed.

## Domain Notes

- Timeline design work should start from [Considerations/pull_push_timeline.md](Considerations/pull_push_timeline.md). The documented default direction is a simple push model that stores post IDs, with a threshold-based hybrid fallback for large accounts.
- Preserve the documented domain rules when implementing features: pinned posts are profile-level (`user_profiles.pinned_post_id`), reports target either a user or a post, and auth/reset/email verification tokens should be stored hashed rather than in plaintext.

## Repository-Specific Guidance

- [README.md](README.md) is only a doc index; the detailed behavior lives in the documents it links to.
- [design.pen](design.pen) is a design artifact. Use `.pen`-aware design tooling for it rather than treating it as a plain text file.
