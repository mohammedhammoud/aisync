import { createLazyRoute, useNavigate } from "@tanstack/react-router";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { useAppLockState } from "@/base/root/appLock";
import { createToast } from "@/base/store/toastStore";
import type { SkillMetadata } from "@/base/tauri/bindings";
import { useLinkStatuses } from "@/core/link-status/hooks/useLinkStatuses";
import { linkStatusesForSkill } from "@/core/link-status/utils/linkStatus";
import { SkillForm } from "@/features/skills/components/SkillForm";
import { Spinner } from "@/ui/components/Spinner";
import { Alert } from "@/ui/components/Alert";
import { useDeleteSkill } from "@/features/skills/hooks/useDeleteSkill";
import { useGetSkill } from "@/features/skills/hooks/useGetSkill";
import { useUpdateSkill } from "@/features/skills/hooks/useUpdateSkill";

type Baseline = {
  content: string;
  metadata: SkillMetadata;
  frontmatterLines: string[];
};

function SkillsDetailView() {
  const navigate = useNavigate();
  const { skillId } = Route.useParams();
  const { t } = useTranslation();

  const { isLoading, skill } = useGetSkill(skillId);
  const { deleteSkill, isDeleting } = useDeleteSkill();
  const { isUpdating, updateSkill } = useUpdateSkill();

  const [metadata, setMetadata] = useState<SkillMetadata | null>(null);
  const [content, setContent] = useState("");
  const [frontmatterLines, setFrontmatterLines] = useState<string[]>([]);
  const [baseline, setBaseline] = useState<Baseline | null>(null);
  const { fixLinkStatus, statuses } = useLinkStatuses();
  const linkStatuses = linkStatusesForSkill(statuses, skillId);

  const isDirty =
    baseline !== null &&
    JSON.stringify({ content, metadata, frontmatterLines }) !==
      JSON.stringify({
        content: baseline.content,
        metadata: baseline.metadata,
        frontmatterLines: baseline.frontmatterLines,
      });

  useEffect(() => {
    if (skill) {
      setMetadata(skill.metadata);
      setContent(skill.body);
      setFrontmatterLines(skill.frontmatterLines);
      setBaseline({
        content: skill.body,
        metadata: skill.metadata,
        frontmatterLines: skill.frontmatterLines,
      });
    }
  }, [skill]);

  useAppLockState({
    isLocked: isDirty,
    message: isDirty ? t("dirty.unsaved") : null,
  });

  async function saveSkill() {
    if (!metadata) {
      return;
    }

    const didSave = await updateSkill(
      skillId,
      metadata,
      content,
      frontmatterLines,
    );

    if (didSave) {
      setBaseline({ content, metadata, frontmatterLines });
      if (metadata.id !== skillId) {
        navigate({
          ignoreBlocker: true,
          params: { skillId: metadata.id },
          replace: true,
          to: "/skills/$skillId",
        });
      }
    }
  }

  function discardSkill() {
    if (!baseline) {
      return;
    }

    setMetadata(baseline.metadata);
    setContent(baseline.content);
    createToast({ message: t("common.discarded") });
  }

  if (isLoading) {
    return (
      <div className="flex h-full items-center justify-center">
        <Spinner size="lg" />
      </div>
    );
  }

  if (!metadata) {
    return (
      <div className="p-4">
        <Alert variant="red">{t("skills.notFound")}</Alert>
      </div>
    );
  }

  return (
    <SkillForm
      content={content}
      isDeleting={isDeleting}
      isDirty={isDirty}
      isUpdating={isUpdating}
      metadata={metadata}
      linkStatuses={linkStatuses}
      onChangeContent={setContent}
      onChangeMetadata={setMetadata}
      onDelete={() => deleteSkill(skillId)}
      onDiscard={discardSkill}
      onFixLinkStatus={fixLinkStatus}
      onSave={saveSkill}
    />
  );
}

export const Route = createLazyRoute("/skills/$skillId")({
  component: SkillsDetailView,
});
