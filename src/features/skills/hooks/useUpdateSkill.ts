import { useState } from "react";
import { useTranslation } from "react-i18next";
import { createToast } from "@/base/store/toastStore";
import type { SkillMetadata } from "@/base/tauri/bindings";
import { translateTauriError } from "@/base/tauri/utils/translateTauriError";
import { commands } from "@/base/tauri/bindings";

export function useUpdateSkill() {
  const { t } = useTranslation();
  const [isUpdating, setIsUpdating] = useState(false);

  async function updateSkill(
    id: string,
    metadata: SkillMetadata,
    content: string,
    frontmatterLines: string[],
  ) {
    setIsUpdating(true);

    try {
      await commands.updateSkill(id, content, metadata, frontmatterLines);
      createToast({ message: t("common.saved") });
      return true;
    } catch (error) {
      createToast({ message: translateTauriError(error), variant: "red" });
      return false;
    } finally {
      setIsUpdating(false);
    }
  }

  return { isUpdating, updateSkill };
}
