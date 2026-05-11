import { useState } from "react";
import { useTranslation } from "react-i18next";
import { createToast } from "@/base/store/toastStore";
import type { SkillMetadata } from "@/base/tauri/bindings";
import { translateTauriError } from "@/base/tauri/utils/translateTauriError";
import { commands } from "@/base/tauri/bindings";

export function useCreateSkill() {
  const { t } = useTranslation();
  const [isCreating, setIsCreating] = useState(false);

  async function createSkill(
    metadata: SkillMetadata,
    content: string,
    frontmatterLines: string[],
  ) {
    setIsCreating(true);

    try {
      await commands.createSkill(
        metadata.id,
        content,
        metadata,
        frontmatterLines,
      );
      createToast({ message: t("common.saved") });
      return true;
    } catch (error) {
      createToast({ message: translateTauriError(error), variant: "red" });
      return false;
    } finally {
      setIsCreating(false);
    }
  }

  return { createSkill, isCreating };
}
