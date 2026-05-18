import { createLazyRoute, useNavigate } from "@tanstack/react-router";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { useAppLockContext, useAppLockState } from "@/base/root/appLock";
import { createToast } from "@/base/store/toastStore";
import type { SkillMetadata } from "@/base/tauri/bindings";
import { SkillForm } from "@/features/skills/components/SkillForm";
import { useCreateSkill } from "@/features/skills/hooks/useCreateSkill";

function createNewSkill(name = "New Skill"): SkillMetadata {
  return {
    id: "new-skill",
    name,
    description: "",
    enabled: true,
    tags: [],
  };
}

function SkillsCreateView() {
  const navigate = useNavigate();
  const { t } = useTranslation();
  const { releaseLock } = useAppLockContext();
  const skillName = t("skills.newName");
  const [metadata, setMetadata] = useState<SkillMetadata>(() =>
    createNewSkill(skillName),
  );
  const [content, setContent] = useState(() =>
    t("skills.newTemplate", { name: skillName }),
  );
  const frontmatterLines: string[] = [];
  const { createSkill, isCreating } = useCreateSkill();

  useAppLockState({ isLocked: true, message: t("dirty.unsaved") });

  async function saveSkill() {
    const didCreate = await createSkill(metadata, content, frontmatterLines);

    if (!didCreate) {
      return;
    }

    releaseLock();
    navigate({
      ignoreBlocker: true,
      params: { skillId: metadata.id },
      replace: true,
      to: "/skills/$skillId",
    });
  }

  function discardSkill() {
    releaseLock();
    createToast({ message: t("common.discarded") });
    navigate({ ignoreBlocker: true, replace: true, to: "/skills" });
  }

  return (
    <SkillForm
      content={content}
      isCreating={isCreating}
      isDirty
      metadata={metadata}
      onChangeContent={setContent}
      onChangeMetadata={setMetadata}
      onDiscard={discardSkill}
      onSave={saveSkill}
    />
  );
}

export const Route = createLazyRoute("/skills/new")({
  component: SkillsCreateView,
});
