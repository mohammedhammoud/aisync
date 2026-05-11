import { useNavigate } from "@tanstack/react-router";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { createToast } from "@/base/store/toastStore";
import { translateTauriError } from "@/base/tauri/utils/translateTauriError";
import { commands } from "@/base/tauri/bindings";

export function useDeleteSkill() {
  const navigate = useNavigate();
  const { t } = useTranslation();
  const [isDeleting, setIsDeleting] = useState(false);

  async function deleteSkill(skillId: string) {
    setIsDeleting(true);

    try {
      await commands.deleteSkill(skillId);
      createToast({ message: t("common.deleted") });

      navigate({ replace: true, to: "/skills" });
    } catch (error) {
      createToast({ message: translateTauriError(error), variant: "red" });
    } finally {
      setIsDeleting(false);
    }
  }

  return { deleteSkill, isDeleting };
}
