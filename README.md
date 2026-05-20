<p align="center">
  <img src="./app-icon.png" alt="AISync logo" width="96" height="96" />
</p>

<h1 align="center">AISync</h1>

<p align="center">
  Stop copying AI coding-agent instructions between tools by hand.
</p>

<p align="center">
  <strong>One local source of truth</strong> · <strong>Optional private GitHub backup</strong> · <strong>macOS</strong> · <strong>Tauri</strong> · <strong>React</strong>
</p>

<p align="center">
  <img src="./screenshot.png" alt="AISync screenshot" />
</p>

## Overview

AISync solves one problem: AI coding tools each want their own instruction and skill files, so keeping them consistent becomes manual copy-paste work.

AISync gives you one place to edit those prompts, skills, and instructions. It keeps the source of truth in a local AISync folder, then syncs enabled skills and global instructions into configured tool folders with symbolic links. The default targets are Codex, Copilot, and Pi, and custom targets can be added from the app.

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

If GitHub sync is enabled, AISync uses GitHub device login, stores the token in the system keychain, and syncs AISync-owned instructions and skills under `.aisync/` in a private `aisync-config` repository. macOS may ask for permission to store or access the token in Keychain.

For development and tests, the local root can be overridden with `AISYNC_HOME`.

## Requirements

- macOS
- Node.js
- pnpm
- Rust
- Tauri prerequisites

Linux and Windows adapters exist in the codebase, but they are not implemented yet.

## Download and install on macOS

1. Open the latest GitHub Release: <https://github.com/mohammedhammoud/aisync/releases/latest>
2. Download the `.dmg` file from the release assets.
3. Open the DMG.
4. Drag `AISync.app` into `Applications`.
5. Launch AISync from `Applications`.

Only download AISync from the official GitHub Releases page above.

AISync is currently unsigned and not notarized. On first launch, macOS Gatekeeper may show:

```txt
"AISync" cannot be opened because Apple cannot check it for malicious software.
```

Safe workaround:

1. Move `AISync.app` to `Applications`.
2. Right-click `AISync.app`.
3. Choose `Open`.
4. Confirm `Open` in the dialog.

If macOS still blocks the app:

1. Open `System Settings`.
2. Go to `Privacy & Security`.
3. Scroll to `Security`.
4. Click `Open Anyway` for AISync.
5. Confirm `Open`.

If that still does not work, remove the quarantine flag manually. Only do this for an app downloaded from the official release page:

```sh
xattr -dr com.apple.quarantine /Applications/AISync.app
```

The app checks for updates on startup and can install signed Tauri updater releases in-app. If updater metadata is unavailable, the notice falls back to the GitHub Release page.

## Quick start

1. Download and install AISync.
2. Open the app and keep the default local AISync folder, or choose another folder.
3. Add or edit global instructions.
4. Add reusable skills.
5. Enable the tools you want to sync, then save.
6. Optional: connect GitHub sync for private backup and another-machine sync.

More detail: [Quick start tutorial](./docs/QUICKSTART.md).

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

## Release

The `Release` workflow runs Release Please from Conventional Commits on `main`. It opens a release PR; merging that PR updates versions and changelog, creates the `aisync-v*.*.*` tag, then builds an unsigned universal macOS DMG plus Tauri updater artifacts.

The same workflow can also be run manually with an existing tag to rebuild/upload the DMG.

No Apple Developer Program is required for this workflow. macOS Gatekeeper may warn users on first open because the app is not signed or notarized.

The release build uploads the DMG, updater metadata, SHA256 checksums, and a detached GPG signature for the checksum file when `GPG_PRIVATE_KEY` is configured.

Updater signing uses the public key in `src-tauri/tauri.conf.json`. Keep the private key outside the repo and configure `TAURI_SIGNING_PRIVATE_KEY` in GitHub Actions secrets.

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

## Issues and contributions

- Questions, bugs, and feature requests: [open an issue](https://github.com/mohammedhammoud/aisync/issues/new/choose).
- Pull requests: read [CONTRIBUTING.md](./CONTRIBUTING.md).
- Expected behavior: [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md).
- Sharing notes: [docs/PROMOTION.md](./docs/PROMOTION.md).

## Contributing

Issues and pull requests are welcome.

Keep changes small, focused, and easy to review. Prefer simple behavior, local-first defaults, and clear file-system safety. Good first contributions include docs improvements, macOS install notes, accessibility fixes, and small UI polish.

## License

MIT. See [LICENSE](./LICENSE).
