# Contributing to AISync

Thanks for helping improve AISync.

AISync is a small local-first macOS app. The main goal is simple: stop people from copying AI coding-agent instructions and skills between tools by hand.

## Good first contributions

Helpful contributions include:

- Fix unclear docs or screenshots.
- Improve the quick start or install notes.
- Add accessibility fixes.
- Add small UI polish that keeps the workflow simple.
- Add tests around existing behavior.
- Improve macOS filesystem safety messages.

Non-code contributions are welcome. Docs, examples, screenshots, issue triage, accessibility notes, and design feedback all help.

Please avoid for now:

- Large rewrites.
- New package dependencies without a strong reason.
- Linux or Windows support that is only partial.
- Changes that make local sync less safe or less predictable.
- Public-by-default cloud sync.

## Before opening a pull request

1. Open an issue first for large behavior changes.
2. Keep the pull request small and focused.
3. Explain the user problem, not only the implementation.
4. Include screenshots or recordings for UI changes.
5. Note any filesystem, auth, privacy, or data-loss risk.

## Development setup

```sh
pnpm install
pnpm dev
```

Desktop app:

```sh
pnpm tauri dev
```

## Checks

Run the smallest relevant checks before opening a PR:

```sh
pnpm typecheck
pnpm lint
pnpm test:unit
```

For end-to-end changes:

```sh
pnpm test:e2e
```

For Tauri command or serializable type changes:

```sh
pnpm generate:bindings
```

Do not hand-edit generated bindings.

## Issue labels

Labels used in this repo:

- `good first issue` for small, well-scoped work.
- `help wanted` for work where outside help would be useful.

If you want to contribute but are unsure where to start, ask on an issue with one of those labels.

## Pull request review

Responses may be slow. Small pull requests with a clear user problem are easiest to review. If something sits without a response, a short follow-up comment is fine.

## Commit style

Use Conventional Commits when practical:

```txt
feat: add target sync status
fix: preserve skill frontmatter
chore: update docs
```

## Code of conduct

By participating, you agree to follow the [Code of Conduct](./CODE_OF_CONDUCT.md).
