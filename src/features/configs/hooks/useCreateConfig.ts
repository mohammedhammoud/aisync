import { useState } from "react";
import { useTranslation } from "react-i18next";
import { createToast } from "@/base/store/toastStore";
import type { TargetConfig } from "@/base/tauri/bindings";
import { translateTauriError } from "@/base/tauri/utils/translateTauriError";
import { commands } from "@/base/tauri/bindings";

export function useCreateConfig() {
  const { t } = useTranslation();
  const [isCreating, setIsCreating] = useState(false);

  async function createConfig(config: TargetConfig) {
    setIsCreating(true);

    try {
      await commands.createConfig(config);
      createToast({ message: t("common.saved") });
      return true;
    } catch (error) {
      createToast({ message: translateTauriError(error), variant: "red" });
      return false;
    } finally {
      setIsCreating(false);
    }
  }

  return { createConfig, isCreating };
}
