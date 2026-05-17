import { listen } from "@tauri-apps/api/event";
import { create } from "zustand";
import { useShallow } from "zustand/react/shallow";
import { createToast } from "@/base/store/toastStore";
import { commands } from "@/base/tauri/bindings";
import type {
  GithubLoginStart,
  GithubSyncStatus,
  SyncResult,
} from "@/base/tauri/bindings";
import { translateTauriError } from "@/base/tauri/utils/translateTauriError";

export type GithubSyncActivity = "idle" | "loading" | "connecting" | "syncing";

type GithubSyncEvent = {
  status: GithubSyncStatus | null;
  lastResult: SyncResult | null;
  isConnecting: boolean;
  isSyncing: boolean;
  isUserInitiatedSync: boolean;
  lastError: string | null;
};

type GithubSyncState = {
  login: GithubLoginStart | null;
  loginExpiresAtMs: number | null;
  status: GithubSyncStatus | null;
  lastResult: SyncResult | null;
  isConnecting: boolean;
  isLoadingStatus: boolean;
  isSyncing: boolean;
  isUserInitiatedSync: boolean;
};

const GITHUB_SYNC_EVENT_NAME = "github-sync-state-changed";

export const useGithubSyncStore = create<GithubSyncState>(() => ({
  login: null,
  loginExpiresAtMs: null,
  status: null,
  lastResult: null,
  isConnecting: false,
  isLoadingStatus: false,
  isSyncing: false,
  isUserInitiatedSync: false,
}));

function getGithubSyncActivity(state: GithubSyncState): GithubSyncActivity {
  if (state.isConnecting) return "connecting";
  if (state.isSyncing) return "syncing";
  if (state.isLoadingStatus) return "loading";
  return "idle";
}

export function useGithubSyncSettingsState() {
  return useGithubSyncStore(
    useShallow((state) => ({
      isConnecting: state.isConnecting,
      isSyncing: state.isSyncing,
      isUserInitiatedSync: state.isUserInitiatedSync,
      lastResult: state.lastResult,
      login: state.login,
      status: state.status,
      syncActivity: getGithubSyncActivity(state),
    })),
  );
}

export function clearGithubSyncLoginState(): void {
  useGithubSyncStore.setState({
    isConnecting: false,
    login: null,
    loginExpiresAtMs: null,
  });
}

export function removeGithubSyncResolvedConflict(path: string): void {
  const { lastResult } = useGithubSyncStore.getState();
  useGithubSyncStore.setState({
    lastResult: lastResult
      ? {
          ...lastResult,
          conflicts: lastResult.conflicts.filter((item) => item.path !== path),
        }
      : lastResult,
  });
}

function showGithubSyncError(error: unknown): void {
  createToast({ message: translateTauriError(error), variant: "red" });
}

export async function loadGithubSyncStatus(): Promise<void> {
  useGithubSyncStore.setState({ isLoadingStatus: true });
  try {
    const status = await commands.getGithubSyncStatus();
    useGithubSyncStore.setState({ status });
  } catch (error) {
    showGithubSyncError(error);
  } finally {
    useGithubSyncStore.setState({ isLoadingStatus: false });
  }
}

function getGithubSyncEventPatch(
  event: GithubSyncEvent,
  state: GithubSyncState,
): Partial<GithubSyncState> {
  const didFinishLogin =
    state.isConnecting && !event.isConnecting && !event.isSyncing;

  return {
    ...(event.status ? { status: event.status } : {}),
    ...(event.lastResult ? { lastResult: event.lastResult } : {}),
    ...(didFinishLogin ? { login: null, loginExpiresAtMs: null } : {}),
    isConnecting: didFinishLogin
      ? false
      : state.isConnecting || event.isConnecting,
    isSyncing: event.isSyncing,
    isUserInitiatedSync: event.isUserInitiatedSync,
  };
}

function onGithubSyncStateChanged(event: GithubSyncEvent): void {
  useGithubSyncStore.setState(
    getGithubSyncEventPatch(event, useGithubSyncStore.getState()),
  );

  if (event.lastError) {
    createToast({ message: event.lastError, variant: "red" });
  }
}

export async function initGithubSyncStore(): Promise<() => void> {
  const unlisten = await listen<GithubSyncEvent>(
    GITHUB_SYNC_EVENT_NAME,
    (event) => onGithubSyncStateChanged(event.payload),
  );
  void loadGithubSyncStatus();
  return unlisten;
}
