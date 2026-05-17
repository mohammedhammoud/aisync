import { useCallback, useState } from "react";
import { useTranslation } from "react-i18next";
import type {
  SyncConflict,
  SyncConflictResolution,
} from "@/base/tauri/bindings";
import { useAppLockState } from "@/base/root/appLock";
import { GitHubConflictDialog } from "@/features/github/components/GitHubConflictDialog";
import { GitHubSyncPanel } from "@/features/github/components/GitHubSyncPanel";
import { useGithubSyncLoginCodeTimer } from "@/features/github/hooks/useGithubSyncLoginCodeTimer";
import { useGithubSyncActions } from "@/features/github/hooks/useGithubSyncActions";
import { useGithubSyncSettingsState } from "@/features/github/store/githubSyncStore";

export function GitHubSyncSettings() {
  const { t } = useTranslation();
  const {
    connectGithub,
    copyGithubLoginCode,
    disconnectGithub,
    openGithubLogin,
    resolveGithubConflict,
    setupGithub,
    syncGithub,
  } = useGithubSyncActions();
  const {
    isConnecting,
    isUserInitiatedSync,
    lastResult,
    login,
    status,
    syncActivity,
  } = useGithubSyncSettingsState();
  const [openConflict, setOpenConflict] = useState<SyncConflict | null>(null);

  const { expiresInSeconds, openDelaySeconds, openGithubLoginNow } =
    useGithubSyncLoginCodeTimer(openGithubLogin);

  const shouldLockUi = isConnecting || isUserInitiatedSync;

  useAppLockState({
    isLocked: shouldLockUi,
    message: shouldLockUi ? t("github.lockMessage") : null,
  });

  const handleOpenConflict = useCallback(
    (conflict: SyncConflict) => setOpenConflict(conflict),
    [],
  );

  const handleCloseConflict = useCallback(() => {
    setOpenConflict(null);
  }, []);

  const handleResolveConflict = useCallback(
    (resolution: SyncConflictResolution) => {
      if (!openConflict) return;
      resolveGithubConflict(openConflict, resolution);
      setOpenConflict(null);
    },
    [openConflict, resolveGithubConflict],
  );

  return (
    <>
      <GitHubSyncPanel
        loginCodeExpiresInSeconds={expiresInSeconds}
        loginCodeOpenDelaySeconds={openDelaySeconds}
        loginCodeUserCode={login?.userCode ?? null}
        conflicts={lastResult?.conflicts ?? []}
        syncActivity={syncActivity}
        syncStatus={status}
        onConnect={connectGithub}
        onCreateRepo={setupGithub}
        onDisconnect={disconnectGithub}
        onLoginCodeCopy={copyGithubLoginCode}
        onLoginCodeOpen={openGithubLoginNow}
        onOpenConflict={handleOpenConflict}
        onSync={syncGithub}
      />
      {openConflict ? (
        <GitHubConflictDialog
          conflict={openConflict}
          onClose={handleCloseConflict}
          onResolve={handleResolveConflict}
        />
      ) : null}
    </>
  );
}
