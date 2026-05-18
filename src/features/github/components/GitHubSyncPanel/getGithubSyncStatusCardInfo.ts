import type { TFunction } from "i18next";
import type { GithubSyncStatus } from "@/base/tauri/bindings";
import type { GitHubStatusCardVariant } from "@/features/github/components/GitHubStatusCard/GitHubStatusCard";
import type { GithubSyncActivity } from "@/features/github/store/githubSyncStore";

type GithubSyncStatusCardInfo = {
  detail: string | null;
  lastSyncedAt: string | null;
  text: string;
  variant: GitHubStatusCardVariant;
};

type GithubSyncStatusCardInfoOptions = {
  activity: GithubSyncActivity;
  status: GithubSyncStatus | null;
  t: TFunction;
};

export function getGithubSyncStatusCardInfo({
  activity,
  status,
  t,
}: GithubSyncStatusCardInfoOptions): GithubSyncStatusCardInfo {
  const repo =
    status?.repoOwner && status.repoName
      ? `${status.repoOwner}/${status.repoName}`
      : null;

  if (activity === "connecting") {
    return {
      detail: t("github.connectingDetail"),
      lastSyncedAt: status?.lastSyncedAt ?? null,
      text: t("github.connecting"),
      variant: "loading",
    };
  }

  if (activity === "syncing") {
    return {
      detail: t("github.syncingDetail"),
      lastSyncedAt: status?.lastSyncedAt ?? null,
      text: repo ? t("github.pushingChanges") : t("github.settingUp"),
      variant: "loading",
    };
  }

  if (status?.connected) {
    if (status.hasLocalChanges) {
      return {
        detail: t("github.pendingSyncDetail"),
        lastSyncedAt: status.lastSyncedAt ?? null,
        text: t("github.pendingSync"),
        variant: "success",
      };
    }

    return {
      detail: null,
      lastSyncedAt: status.lastSyncedAt ?? null,
      text: t("github.connected", { repo }),
      variant: "success",
    };
  }

  if (repo) {
    return {
      detail: t("github.reconnectRequiredDetail"),
      lastSyncedAt: status?.lastSyncedAt ?? null,
      text: t("github.reconnectRequired", { repo }),
      variant: "failed",
    };
  }

  if (status?.hasToken) {
    return {
      detail: t("github.authorizedDetail"),
      lastSyncedAt: status.lastSyncedAt ?? null,
      text: t("github.authorized"),
      variant: "failed",
    };
  }

  return {
    detail: null,
    lastSyncedAt: status?.lastSyncedAt ?? null,
    text: t("github.notConnected"),
    variant: "failed",
  };
}
