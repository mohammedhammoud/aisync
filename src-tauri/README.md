# AISync Tauri backend

Rust owns local data, filesystem sync, and GitHub sync. The frontend calls only Tauri commands exported in `src/base/tauri/bindings.ts`.

AISync currently supports macOS only. Linux/Windows platform files are intentionally kept as stubs so support can be added later, but non-macOS builds fail at compile time.

## Modules

- `core/config.rs`: app config, target configs, skill metadata, defaults.
- `core/skills.rs`: skill editor records, frontmatter preservation, skill file writes.
- `core/instructions.rs`: shared instruction file read/write.
- `core/sync.rs`: local target sync. Current mode is symlink-only.
- `core/github/*`: GitHub auth, settings, API calls, merge/conflict handling, and sync workers.
- `core/path_safety.rs`: ID and path boundary checks.
- `os/*` and `platform.rs`: platform-specific defaults and symlink behavior.

## Data locations

- Main setup root: resolved by `platform().app_root()` / `core::path_safety::app_root()`.
- Main config: `config.json` under the setup root.
- Shared instructions: `instructions.md` under the setup root.
- Shared skills: `skills/<skill-id>/` under the setup root.
- GitHub local settings: platform local settings dir, file `github.json`.

## Sync safety model

Local target sync creates symlinks from target folders back to the setup root. Existing target files are backed up before replacement. Stale skill links are removed only when they are symlinks owned by AISync.

GitHub sync stores only shared skills/instructions. Local target paths stay local and are not pushed. Conflicts are saved under the setup root conflict directory for inspection.

## Bindings

After changing Tauri commands, serializable types, or error codes, regenerate frontend bindings:

```sh
pnpm generate:bindings
```

Do not hand-edit `src/base/tauri/bindings.ts`.
