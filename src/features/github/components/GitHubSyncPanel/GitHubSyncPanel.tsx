import { useTranslation } from "react-i18next";
import type { GithubSyncStatus, SyncConflict } from "@/base/tauri/bindings";
import { GitHubConflictList } from "@/features/github/components/GitHubConflictList";
import { GitHubLoginCodePanel } from "@/features/github/components/GitHubLoginCodePanel";
import { GitHubStatusCard } from "@/features/github/components/GitHubStatusCard";
import { GitHubSyncActions } from "@/features/github/components/GitHubSyncActions";
import type { GithubSyncActivity } from "@/features/github/store/githubSyncStore";
import { Text } from "@/ui/components/Text";
import { getGithubSyncStatusCardInfo } from "./getGithubSyncStatusCardInfo";

export type GitHubSyncPanelProps = {
  loginCodeExpiresInSeconds: number;
  loginCodeOpenDelaySeconds: number;
  loginCodeUserCode: string | null;
  conflicts: SyncConflict[];
  syncActivity: GithubSyncActivity;
  syncStatus: GithubSyncStatus | null;
  onConnect: () => void;
  onCreateRepo: () => void;
  onDisconnect: () => void;
  onLoginCodeCopy: () => void;
  onLoginCodeOpen: () => void;
  onOpenConflict: (conflict: SyncConflict) => void;
  onSync: () => void;
};

export function GitHubSyncPanel({
  loginCodeExpiresInSeconds,
  loginCodeOpenDelaySeconds,
  loginCodeUserCode,
  conflicts,
  syncActivity,
  syncStatus,
  onConnect,
  onCreateRepo,
  onDisconnect,
  onLoginCodeCopy,
  onLoginCodeOpen,
  onOpenConflict,
  onSync,
}: GitHubSyncPanelProps) {
  const { t } = useTranslation();
  const statusCardInfo = getGithubSyncStatusCardInfo({
    activity: syncActivity,
    status: syncStatus,
    t,
  });

  return (
    <div className="grid gap-4">
      <div className="grid gap-1">
        <Text as="h2" className="text-sm font-semibold">
          {t("github.title")}
        </Text>
        <Text as="p" className="text-xs" tone="muted">
          {t("github.description")}
        </Text>
      </div>

      <GitHubStatusCard
        detail={statusCardInfo.detail}
        lastSyncedAt={statusCardInfo.lastSyncedAt}
        text={statusCardInfo.text}
        variant={statusCardInfo.variant}
      />

      {loginCodeUserCode ? (
        <GitHubLoginCodePanel
          expiresInSeconds={loginCodeExpiresInSeconds}
          openDelaySeconds={loginCodeOpenDelaySeconds}
          userCode={loginCodeUserCode}
          onCopyCode={onLoginCodeCopy}
          onOpenLogin={onLoginCodeOpen}
        />
      ) : null}

      <GitHubConflictList
        conflicts={conflicts}
        onOpenConflict={onOpenConflict}
      />

      <GitHubSyncActions
        activity={syncActivity}
        syncStatus={syncStatus}
        onConnect={onConnect}
        onCreateRepo={onCreateRepo}
        onDisconnect={onDisconnect}
        onSync={onSync}
      />
    </div>
  );
}
