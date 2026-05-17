import { openUrl } from "@tauri-apps/plugin-opener";
import { useCallback } from "react";
import { useTranslation } from "react-i18next";
import { createToast } from "@/base/store/toastStore";
import { commands } from "@/base/tauri/bindings";
import type {
  SyncConflict,
  SyncConflictResolution,
} from "@/base/tauri/bindings";
import { translateTauriError } from "@/base/tauri/utils/translateTauriError";
import {
  clearGithubSyncLoginState,
  loadGithubSyncStatus,
  removeGithubSyncResolvedConflict,
  useGithubSyncStore,
} from "@/features/github/store/githubSyncStore";

const MILLISECONDS_PER_SECOND = 1000;

function showGithubActionError(error: unknown): void {
  createToast({ message: translateTauriError(error), variant: "red" });
}

export function useGithubSyncActions() {
  const { t } = useTranslation();

  const openGithubLogin = useCallback(async () => {
    const { login } = useGithubSyncStore.getState();
    if (!login) return;

    try {
      await openUrl(login.verificationUri);
    } catch (error) {
      showGithubActionError(error);
    }
  }, []);

  const copyGithubLoginCode = useCallback(async () => {
    const { login } = useGithubSyncStore.getState();
    if (!login) return;

    try {
      await navigator.clipboard.writeText(login.userCode);
      createToast({ message: t("github.codeCopied") });
    } catch (error) {
      showGithubActionError(error);
    }
  }, [t]);

  const connectGithub = useCallback(async () => {
    try {
      useGithubSyncStore.setState({ isConnecting: true });
      const login = await commands.startGithubLogin();
      useGithubSyncStore.setState({
        login,
        loginExpiresAtMs:
          Date.now() + login.expiresIn * MILLISECONDS_PER_SECOND,
      });
    } catch (error) {
      showGithubActionError(error);
    }
  }, []);

  const setupGithub = useCallback(async () => {
    try {
      await commands.setupGithubSync();
      createToast({ message: t("github.syncEnabled") });
    } catch (error) {
      showGithubActionError(error);
    }
  }, [t]);

  const syncGithub = useCallback(async () => {
    try {
      const result = await commands.syncGithubNow();
      if (result.status === "conflict") {
        createToast({ message: t("github.conflict"), variant: "red" });
      } else if (result.status === "up_to_date") {
        createToast({ message: t("github.upToDate") });
      } else if (result.status === "pulled") {
        createToast({ message: t("github.pulled") });
      } else {
        createToast({ message: t("github.synced") });
      }
    } catch (error) {
      showGithubActionError(error);
    }
  }, [t]);

  const disconnectGithub = useCallback(async () => {
    try {
      await commands.logoutGithub();
      clearGithubSyncLoginState();
      useGithubSyncStore.setState({ lastResult: null });
      await loadGithubSyncStatus();
      createToast({ message: t("github.disconnected") });
    } catch (error) {
      showGithubActionError(error);
    }
  }, [t]);

  const resolveGithubConflict = useCallback(
    async (conflict: SyncConflict, resolution: SyncConflictResolution) => {
      try {
        await commands.resolveSyncConflict(conflict.path, resolution);
        removeGithubSyncResolvedConflict(conflict.path);
        createToast({ message: t("github.conflictResolved") });
      } catch (error) {
        showGithubActionError(error);
      }
    },
    [t],
  );

  return {
    connectGithub,
    copyGithubLoginCode,
    disconnectGithub,
    openGithubLogin,
    resolveGithubConflict,
    setupGithub,
    syncGithub,
  };
}
