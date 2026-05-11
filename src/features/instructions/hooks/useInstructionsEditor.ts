import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { useAppLockState } from "@/base/root/appLock";
import { createToast } from "@/base/store/toastStore";
import { translateTauriError } from "@/base/tauri/utils/translateTauriError";
import { commands } from "@/base/tauri/bindings";

export function useInstructionsEditor() {
  const { t } = useTranslation();
  const [instructions, setInstructions] = useState("");
  const [savedInstructions, setSavedInstructions] = useState("");
  const isDirty = instructions !== savedInstructions;

  useAppLockState({
    isLocked: isDirty,
    message: isDirty ? t("dirty.unsaved") : null,
  });

  useEffect(() => {
    async function loadInstructions() {
      try {
        const content = await commands.readInstructions();
        setInstructions(content);
        setSavedInstructions(content);
      } catch (error) {
        createToast({ message: translateTauriError(error), variant: "red" });
      }
    }

    loadInstructions();
  }, []);

  function discardChanges() {
    setInstructions(savedInstructions);
    createToast({ message: t("common.discarded") });
  }

  async function saveInstructions() {
    try {
      await commands.writeInstructions(instructions);
      setSavedInstructions(instructions);
      createToast({ message: t("common.saved") });
    } catch (error) {
      createToast({ message: translateTauriError(error), variant: "red" });
    }
  }

  return {
    discardChanges,
    instructions,
    isDirty,
    saveInstructions,
    setInstructions,
  };
}
