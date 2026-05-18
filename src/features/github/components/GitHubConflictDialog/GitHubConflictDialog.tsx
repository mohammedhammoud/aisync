import { diffLines } from "diff";
import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import type {
  SyncConflict,
  SyncConflictResolution,
} from "@/base/tauri/bindings";
import { cx } from "@/base/utils/cx";
import { Button } from "@/ui/components/Button";
import { Dialog } from "@/ui/components/Dialog";
import { Text } from "@/ui/components/Text";
import { useTheme } from "@/ui/theme/useTheme";

type ConflictDiffProps = {
  localContent: string | null;
  remoteContent: string | null;
};

function ConflictDiff({ localContent, remoteContent }: ConflictDiffProps) {
  const { t } = useTranslation();
  const { getVariant } = useTheme();

  const v = {
    diff: getVariant("neutral").solid,
    added: getVariant("green").solid,
    removed: getVariant("red").solid,
  };

  const local = localContent ?? t("github.deleted");
  const remote = remoteContent ?? t("github.deleted");
  const parts = useMemo(() => diffLines(local, remote), [local, remote]);

  return (
    <div className="grid gap-2">
      <div className="flex gap-4 text-xs">
        <Text tone="muted">- {t("github.localVersion")}</Text>
        <Text tone="muted">+ {t("github.githubVersion")}</Text>
      </div>
      <pre
        aria-label={t("github.conflictDiff")}
        className={cx(
          "max-h-[28rem] overflow-auto whitespace-pre-wrap rounded p-3 text-xs",
          v.diff.base.background,
          v.diff.base.readableColor,
        )}
      >
        {parts.map((part, index) => {
          const prefix = part.added ? "+ " : part.removed ? "- " : "  ";
          return (
            <span
              className={cx(
                "block px-2",
                part.added && [
                  v.added.base.background,
                  v.added.base.readableColor,
                ],
                part.removed && [
                  v.removed.base.background,
                  v.removed.base.readableColor,
                ],
              )}
              key={`${prefix}-${index}`}
            >
              {prefix}
              {part.value}
            </span>
          );
        })}
      </pre>
    </div>
  );
}

type GitHubConflictDialogProps = {
  conflict: SyncConflict;
  onClose: () => void;
  onResolve: (resolution: SyncConflictResolution) => void;
};

export function GitHubConflictDialog({
  conflict,
  onClose,
  onResolve,
}: GitHubConflictDialogProps) {
  const { t } = useTranslation();

  return (
    <Dialog
      actions={
        <>
          <Button onClick={() => onResolve("local")}>
            {t("github.useLocal")}
          </Button>
          <Button mode="outline" onClick={() => onResolve("remote")}>
            {t("github.useRemote")}
          </Button>
        </>
      }
      heading={t("github.conflictFor", { path: conflict.path })}
      size="lg"
      onClose={(open) => {
        if (!open) onClose();
      }}
      open={true}
    >
      <ConflictDiff
        localContent={conflict.localContent}
        remoteContent={conflict.remoteContent}
      />
    </Dialog>
  );
}
