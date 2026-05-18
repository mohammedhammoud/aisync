import { useTranslation } from "react-i18next";
import type { GithubSyncStatus } from "@/base/tauri/bindings";
import type { GithubSyncActivity } from "@/features/github/store/githubSyncStore";
import { Button } from "@/ui/components/Button";
import { Toolbar } from "@/ui/components/Toolbar";

type GitHubSyncActionsProps = {
  activity: GithubSyncActivity;
  syncStatus: GithubSyncStatus | null;
  onConnect: () => void;
  onCreateRepo: () => void;
  onDisconnect: () => void;
  onSync: () => void;
};

export function GitHubSyncActions({
  activity,
  syncStatus,
  onConnect,
  onCreateRepo,
  onDisconnect,
  onSync,
}: GitHubSyncActionsProps) {
  const { t } = useTranslation();
  const isConnected = Boolean(syncStatus?.connected);
  const hasRepo = Boolean(syncStatus?.repoOwner && syncStatus?.repoName);
  const hasToken = Boolean(syncStatus?.hasToken);

  if (activity !== "idle") return null;

  return (
    <Toolbar>
      {!hasToken ? (
        <Button onClick={onConnect}>
          {hasRepo ? t("github.reconnect") : t("github.connect")}
        </Button>
      ) : null}
      {hasToken && !isConnected ? (
        <Button onClick={onCreateRepo}>{t("github.createRepo")}</Button>
      ) : null}
      {isConnected ? (
        <Button onClick={onSync}>{t("github.syncNow")}</Button>
      ) : null}
      {hasToken ? (
        <Button onClick={onDisconnect} variant="red">
          {t("github.disconnect")}
        </Button>
      ) : null}
    </Toolbar>
  );
}
