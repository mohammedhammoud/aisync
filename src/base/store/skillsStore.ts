import { listen } from "@tauri-apps/api/event";
import { create } from "zustand";
import { translateTauriError } from "@/base/tauri/utils/translateTauriError";
import { SKILLS_CHANGED_EVENT } from "@/base/tauri/consts";
import type { SkillMetadata } from "@/base/tauri/bindings";
import { commands } from "@/base/tauri/bindings";
import { createToast } from "@/base/store/toastStore";

type SkillsState = {
  skills: SkillMetadata[] | null;
};

export const useSkillsStore = create<SkillsState>(() => ({
  skills: null,
}));

async function loadSkills() {
  try {
    const skills = await commands.getSkills();
    useSkillsStore.setState({ skills });
  } catch (error) {
    createToast({ message: translateTauriError(error), variant: "red" });
  }
}

export async function initSkillsStore(): Promise<() => void> {
  await loadSkills();
  return listen(SKILLS_CHANGED_EVENT, loadSkills);
}
