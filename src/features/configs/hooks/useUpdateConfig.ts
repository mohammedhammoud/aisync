import { useState } from "react";
import { useTranslation } from "react-i18next";
import { createToast } from "@/base/store/toastStore";
import type { TargetConfig } from "@/base/tauri/bindings";
import { translateTauriError } from "@/base/tauri/utils/translateTauriError";
import { commands } from "@/base/tauri/bindings";

export function useUpdateConfig() {
  const { t } = useTranslation();
  const [isUpdating, setIsUpdating] = useState(false);

  async function updateConfig(id: string, config: TargetConfig) {
    setIsUpdating(true);

    try {
      await commands.updateConfig(id, config);
      createToast({ message: t("common.saved") });
      return true;
    } catch (error) {
      createToast({ message: translateTauriError(error), variant: "red" });
      return false;
    } finally {
      setIsUpdating(false);
    }
  }

  return { isUpdating, updateConfig };
}
