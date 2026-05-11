import { create } from "zustand";
import { translateTauriError } from "@/base/tauri/utils/translateTauriError";
import type { Globals } from "@/base/tauri/bindings";
import { commands } from "@/base/tauri/bindings";
import { createToast } from "@/base/store/toastStore";

type GlobalsState = {
  globals: Globals | null;
};

export const useGlobalsStore = create<GlobalsState>(() => ({
  globals: null,
}));

export async function initGlobalsStore(): Promise<void> {
  try {
    const globals = await commands.getGlobals();
    document.title = globals.appName;
    useGlobalsStore.setState({ globals });
  } catch (error) {
    createToast({ message: translateTauriError(error), variant: "red" });
  }
}
