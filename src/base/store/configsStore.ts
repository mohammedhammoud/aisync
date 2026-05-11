import { listen } from "@tauri-apps/api/event";
import { create } from "zustand";
import type { Defaults, TargetConfig } from "@/base/tauri/bindings";
import { translateTauriError } from "@/base/tauri/utils/translateTauriError";
import { CONFIGS_CHANGED_EVENT } from "@/base/tauri/consts";
import { commands } from "@/base/tauri/bindings";
import { createToast } from "@/base/store/toastStore";

type ConfigsState = {
  configs: TargetConfig[] | null;
  defaults: Defaults | null;
};

export const useConfigsStore = create<ConfigsState>(() => ({
  configs: null,
  defaults: null,
}));

async function loadConfigs() {
  try {
    const configs = await commands.getConfigs();
    useConfigsStore.setState({ configs });
  } catch (error) {
    createToast({ message: translateTauriError(error), variant: "red" });
  }
}

async function loadDefaults() {
  try {
    const defaults = await commands.getDefaults();
    useConfigsStore.setState({ defaults });
  } catch (error) {
    createToast({ message: translateTauriError(error), variant: "red" });
  }
}

export async function initConfigsStore(): Promise<() => void> {
  await Promise.all([loadConfigs(), loadDefaults()]);
  return listen(CONFIGS_CHANGED_EVENT, loadConfigs);
}
