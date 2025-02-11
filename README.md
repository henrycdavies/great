# great

🚨 **This project is a quick (& dirty) WIP PoC. Do not trust it with your work unless you are comfortable with risk of data loss.** 🚨


A zippy [stacked workflow](https://www.stacking.dev) CLI leveraging [git2-rs libgit2 bindings for Rust](https://github.com/rust-lang/git2-rs).

The project is under active development with one regular contributor, and is in prototyping/proof-of-concept stage.


## Motivation

I had already been using a stacked workflow when I came round to using [Graphite](https://graphite.dev) a couple of years ago. I had worked on a project focusing on educating, enabling, and measuring developer productivity & software engineering maturity, so focusing on my team's practices was very much at the front of my mind.

My praises of the stacked workflow pretty much echo what others have to say about it:

- The ability to keep changes small and incremental (DORA - Working in small batches) is __so__ important for any team looking to deliver at speed. Incremental change shortens the feedback loop, and incremental and **atomic** change makes it easier to triage and fix defects.
- Stacking enables feedback and work in-parallel. One can continue working on a stack while they receive feedback and restack (i.e. rebase and shift their stack) when their change is merged (or others' changes are merged).
- Provided commit messages are clear and changes are atomic, it is so much easier to understand and search through a trunk's history.

Graphite's CLI implementation is a great one too. I really enjoy using it. Its commands are idiomatic, and folding/moving stacks with it is so useful. However, it does have its shortcomings (from the perspective of a professional engineer):

- `gt publish` (submitting PRs, stacks of PRs) is only useful for GitHub users. It doesn't support Azure DevOps, Bitbucket, GitLab, or any other tooling prevalent in enterprises. Even for small teams, using it (understandably) incurs costs.
- Graphite doesn't take open-source submissions anymore, given that it went closed source. So for non-GitHub support we have to wait for Graphite to support this.
- Graphite brought in breaking changes to `gt sync` in version 1.28 a local repo to be configured with a GitHub remote. This broke Graphite integration for any repositories configured with other remotes.

Great aims to provide an open-source alternative that addresses these shortcomings. Its overarching aims/philosophies are:

- Provide an open-source core product compatible with any remote.
- Modularity and extensibility: encourage open-source extensions for compatibility with various remotes and other integrations.


## Feature status

Below is a non-exhaustive list of commands planned for implementation. Their naming may change, and most don't have any switches/flags implemented. Commands may also be added/removed if they seem to reimplement behaviour already provided by git.

| Comand | Description | Status | Notes |
| -- | -- | -- | -- |
| `cascade` | Cascades changes from the current branch into the parent in the stack, or recursively does so if a grandparent is provided. | | |
| `checkout` | Checks out a branch and adds as child to the trunk if not configured | Working | Edge case bugs probably exist. |
| `delete` | Deletes a branch from local repository. | | |
| `down` | Navigates down the current stack of branches. | | |
| `new` | Creates a new branch on top of the current stack as direct descendent. | Working | Edge case bugs probably exist. |
| `publish` | Updates existing branch and pushes to the remote. | | Publish will push by default. Adding `--pr` will submit a pull request with the user's configured Git provider. |
| `sync` | Fetches and pulls the latest trunk revision into the local repository, merging upstream changes or requiring users to merge conflicts if they exist. | Work in progress | Works but no recursion for child branches. |
| `trunk` | Checks out the trunk branch. | Working | Edge case bugs probably exist. |
| `up` | Navigates up the current stack of branches. | | |
| `update` | Modifies the current working branch. Currently this essentially rewinds the commit history to the parent branch, keeping the changes from the exisiting commit, adding current changes, and making a fresh commit to the branch. Usage of `new` is preferable  | Working | Edge case bugs probably exist. |

Additionally:

- Unit tests
- Integration tests
- Refactoring and a bit of love
- Documentation

## Contributing

Requires:

- `rustc>=1.84.0`

Project can be built with

```bash
cargo build
```
