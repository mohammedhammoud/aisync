import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { SKILLS_CHANGED_EVENT } from "@/base/tauri/consts";
import type { SkillEditorRecord } from "@/base/tauri/bindings";
import { translateTauriError } from "@/base/tauri/utils/translateTauriError";
import { commands } from "@/base/tauri/bindings";
import { createToast } from "@/base/store/toastStore";

export function useGetSkill(skillId: string) {
  const [skill, setSkill] = useState<SkillEditorRecord | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    let isMounted = true;
    setIsLoading(true);

    async function load() {
      try {
        const result = await commands.getSkill(skillId);
        if (isMounted) setSkill(result);
      } catch (error) {
        if (isMounted)
          createToast({ message: translateTauriError(error), variant: "red" });
      } finally {
        if (isMounted) setIsLoading(false);
      }
    }

    load();

    const unlisten = listen(SKILLS_CHANGED_EVENT, load);

    return () => {
      isMounted = false;
      unlisten.then((fn) => fn());
    };
  }, [skillId]);

  return { isLoading, skill };
}
