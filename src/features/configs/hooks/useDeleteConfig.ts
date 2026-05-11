import { useNavigate } from "@tanstack/react-router";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { createToast } from "@/base/store/toastStore";
import { translateTauriError } from "@/base/tauri/utils/translateTauriError";
import { commands } from "@/base/tauri/bindings";

export function useDeleteConfig() {
  const navigate = useNavigate();
  const { t } = useTranslation();
  const [isDeleting, setIsDeleting] = useState(false);

  async function deleteConfig(id: string) {
    setIsDeleting(true);

    try {
      await commands.deleteConfig(id);
      createToast({ message: t("common.deleted") });
      navigate({ replace: true, to: "/configs" });
    } catch (error) {
      createToast({ message: translateTauriError(error), variant: "red" });
    } finally {
      setIsDeleting(false);
    }
  }

  return { deleteConfig, isDeleting };
}
