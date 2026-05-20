import { RotateCcw, Save, Trash2 } from "lucide-react";
import { useTranslation } from "react-i18next";
import type { LinkStatus, SkillMetadata } from "@/base/tauri/bindings";
import { LinkStatusNotice } from "@/core/link-status/components/LinkStatusNotice";
import { Button } from "@/ui/components/Button";
import { ConfirmButton } from "@/ui/components/ConfirmButton";
import { CheckboxField } from "@/ui/components/CheckboxField";
import { FormField } from "@/ui/components/FormField";
import { TextEditor } from "@/ui/components/TextEditor";
import { TextInput } from "@/ui/components/TextInput";
import { Toolbar } from "@/ui/components/Toolbar";

type SkillFormProps = {
  content: string;
  isCreating?: boolean;
  isDeleting?: boolean;
  isDirty: boolean;
  isUpdating?: boolean;
  metadata: SkillMetadata;
  linkStatuses?: LinkStatus[];
  onChangeContent: (value: string) => void;
  onChangeMetadata: (metadata: SkillMetadata) => void;
  onDelete?: () => void;
  onDiscard: () => void;
  onFixLinkStatus?: (status: LinkStatus) => Promise<void>;
  onSave: () => void;
};

export function SkillForm({
  content,
  isCreating = false,
  isDeleting = false,
  isDirty,
  isUpdating = false,
  metadata,
  linkStatuses = [],
  onChangeContent,
  onChangeMetadata,
  onDelete,
  onDiscard,
  onFixLinkStatus,
  onSave,
}: SkillFormProps) {
  const { t } = useTranslation();
  return (
    <div className="flex h-full flex-col gap-4">
      <div className="grid grid-cols-2 gap-4">
        <FormField label={t("skills.id")}>
          <TextInput
            spellCheck={false}
            autoComplete="off"
            autoCapitalize="off"
            autoCorrect="off"
            value={metadata.id}
            onChange={(event) =>
              onChangeMetadata({ ...metadata, id: event.target.value })
            }
          />
        </FormField>
        <FormField label={t("skills.name")}>
          <TextInput
            spellCheck={false}
            autoComplete="off"
            autoCapitalize="off"
            autoCorrect="off"
            value={metadata.name}
            onChange={(event) =>
              onChangeMetadata({ ...metadata, name: event.target.value })
            }
          />
        </FormField>
      </div>
      <FormField label={t("skills.description")}>
        <TextInput
          spellCheck={false}
          autoComplete="off"
          autoCapitalize="off"
          autoCorrect="off"
          value={metadata.description}
          onChange={(event) =>
            onChangeMetadata({ ...metadata, description: event.target.value })
          }
        />
      </FormField>
      <CheckboxField
        checked={metadata.enabled}
        onChange={(checked) =>
          onChangeMetadata({ ...metadata, enabled: checked })
        }
      >
        {t("skills.enabled")}
      </CheckboxField>
      <LinkStatusNotice
        onFixLinkStatus={onFixLinkStatus}
        statuses={linkStatuses}
      />
      <TextEditor
        label={t("skills.content")}
        value={content}
        onChange={onChangeContent}
      />
      <Toolbar>
        {isDirty && (
          <Button
            icon={<RotateCcw size={15} />}
            onClick={onDiscard}
            variant="yellow"
          >
            {t("dirty.discard")}
          </Button>
        )}
        <div className="ml-auto flex items-center gap-2">
          <Button
            disabled={!isDirty || isCreating || isUpdating}
            icon={<Save size={15} />}
            onClick={onSave}
            variant="violet"
          >
            {t("skills.save")}
          </Button>
          {onDelete && (
            <ConfirmButton
              cancelLabel={t("common.cancel")}
              confirmLabel={t("common.delete")}
              dialogVariant="red"
              disabled={isDeleting}
              heading={t("dialogs.deleteSkillHeading")}
              icon={<Trash2 size={15} />}
              message={t("dialogs.deleteSkillMessage", {
                name: metadata.name || metadata.id,
              })}
              onConfirm={onDelete}
              variant="red"
            >
              {t("skills.delete")}
            </ConfirmButton>
          )}
        </div>
      </Toolbar>
    </div>
  );
}
