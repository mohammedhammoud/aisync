<p align="center">
  <img src="./app-icon.png" alt="AISync logo" width="96" height="96" />
</p>

<h1 align="center">AISync</h1>

<p align="center">
  A small desktop app for keeping AI coding-agent skills and instructions in sync.
</p>

<p align="center">
  <strong>Local-first</strong> · <strong>Optional GitHub sync</strong> · <strong>macOS</strong> · <strong>Tauri</strong> · <strong>React</strong>
</p>

<p align="center">
  <img src="./screenshot.png" alt="AISync screenshot" />
</p>

## Overview

AISync gives you one place to edit the prompts, skills, and instruction files you use across AI coding tools.

It keeps the source of truth in a local AISync folder, then syncs enabled skills and global instructions into configured tool folders with symbolic links. The default targets are Codex, Copilot, and Pi, and custom targets can be added from the app.

AISync is local-first by default. You can also connect GitHub to back up and sync your AISync instructions and skills through a private repository.

## Features

- Manage reusable AI-agent skills.
- Edit shared global instructions.
- Sync into multiple tool configurations.
- Optionally sync instructions and skills through GitHub.
- Resolve local/remote sync conflicts.
- Enable or disable individual skills and targets.
- Backup existing target files before replacing them.
- Light, dark, and system themes.
- English and Swedish UI.

## How it works

AISync stores its data under `~/.aisync` by default:

- `config.json` stores sync targets and skill metadata.
- `skills/` stores skill folders.
- `instructions.md` stores global instructions.

When you save changes, AISync updates enabled targets by creating symlinks from the target tool folders back to the AISync source files. Existing target files are backed up into `.aisync-backups` before they are replaced.

If GitHub sync is enabled, AISync uses GitHub device login, stores the token in the system keychain, and syncs AISync-owned instructions and skills under `.aisync/` in a private `aisync-config` repository.

For development and tests, the local root can be overridden with `AISYNC_HOME`.

## Requirements

- macOS
- Node.js
- pnpm
- Rust
- Tauri prerequisites

Linux and Windows adapters exist in the codebase, but they are not implemented yet.

## Development

Install dependencies:

```sh
pnpm install
```

Run the web app:

```sh
pnpm dev
```

Run the desktop app:

```sh
pnpm tauri dev
```

Build:

```sh
pnpm build
pnpm tauri build
```

## Quality checks

```sh
pnpm typecheck
pnpm lint
pnpm test:unit
pnpm test:e2e
```

Storybook:

```sh
pnpm storybook
```

## Tech stack

- Tauri 2
- React 19
- TypeScript
- Vite
- Tailwind CSS
- Zustand
- TanStack Router
- i18next
- Vitest
- Playwright
- Storybook

## Contributing

Issues and pull requests are welcome.

Keep changes small, focused, and easy to review. Prefer simple behavior, local-first defaults, and clear file-system safety.

## License

Add a license before publishing this repository as open source.
