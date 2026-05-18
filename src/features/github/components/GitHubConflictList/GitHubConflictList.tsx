import { useTranslation } from "react-i18next";
import type { SyncConflict } from "@/base/tauri/bindings";
import { Badge } from "@/ui/components/Badge";
import { Button } from "@/ui/components/Button";
import { Indicator } from "@/ui/components/Indicator";
import { Text } from "@/ui/components/Text";
import { Pane } from "@/ui/components/Pane";

type GitHubConflictListProps = {
  conflicts: SyncConflict[];
  onOpenConflict: (conflict: SyncConflict) => void;
};

export function GitHubConflictList({
  conflicts,
  onOpenConflict,
}: GitHubConflictListProps) {
  const { t } = useTranslation();

  if (!conflicts.length) return null;

  return (
    <Pane mode="outline" variant="neutral">
      <div className="flex items-center gap-2">
        <Text as="h2" className="text-lg font-semibold leading-none">
          {t("github.conflictsHeading")}
        </Text>
        <Badge size="md" variant="red">
          {conflicts.length}
        </Badge>
      </div>

      <Text as="p" className="mt-2 text-sm leading-5" tone="secondary">
        {t("github.conflictsDescription")}
      </Text>

      <div className="mt-4 flex flex-col gap-2">
        {conflicts.map((conflict) => (
          <div
            className="flex items-center justify-between gap-2 rounded-md border border-neutral-200 bg-neutral-50/60 px-4 py-2 dark:border-neutral-800 dark:bg-neutral-950/60"
            key={conflict.path}
          >
            <div className="flex min-w-0 flex-1 items-center gap-2">
              <Indicator label={t("github.conflict")} variant="yellow" />
              <Text as="div" className="truncate text-sm font-semibold">
                {conflict.path}
              </Text>
            </div>
            <Button
              size="sm"
              variant="violet"
              mode="outline"
              onClick={() => onOpenConflict(conflict)}
            >
              <span>{t("github.review")}</span>
            </Button>
          </div>
        ))}
      </div>
    </Pane>
  );
}
