# AISync quick start

AISync keeps AI coding-agent instructions and skills in one local place, then syncs them into the tools you use.

## 1. Install AISync

Download the latest macOS DMG from:

<https://github.com/mohammedhammoud/aisync/releases/latest>

Open the DMG, drag `AISync.app` to `Applications`, then launch it.

AISync is currently unsigned. If macOS blocks the first launch, right-click `AISync.app`, choose `Open`, then confirm.

After installation, AISync can install signed in-app updates when release updater metadata is available.

## 2. Choose the AISync folder

By default, AISync stores its source files in:

```txt
~/.aisync
```

That folder contains:

- `instructions.md` for shared global instructions.
- `skills/` for reusable skills.
- `config.json` for target and skill metadata.

## 3. Add global instructions

Open AISync and edit the shared instructions. These are the baseline instructions you want available in each enabled coding tool.

Keep them general. Tool-specific details are better as separate skills or target-specific settings.

## 4. Add skills

Create a skill for repeatable workflows, for example:

- Audit changes.
- Write release notes.
- Generate tests.
- Review accessibility.

Enable only the skills you want synced.

## 5. Enable sync targets

AISync ships with default targets for Codex, Copilot, and Pi. You can add custom targets from the app.

When you save, AISync creates symlinks from each enabled target back to the AISync source files. Existing files are backed up into `.aisync-backups` before replacement.

## 6. Optional: enable GitHub sync

GitHub sync backs up AISync-owned instructions and skills to a private `aisync-config` repository.

AISync stores the GitHub token in the system keychain. It does not push local target paths.

## Troubleshooting

- If a target does not update, confirm that the target is enabled.
- If macOS blocks launch, use the Gatekeeper steps in the main README.
- If GitHub sync reports a conflict, inspect the conflict file under the AISync conflict directory before overwriting anything.
