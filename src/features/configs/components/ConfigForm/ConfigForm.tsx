import { RotateCcw, Save, Trash2 } from "lucide-react";
import { useTranslation } from "react-i18next";
import type { TargetConfig } from "@/base/tauri/bindings";
import { Button } from "@/ui/components/Button";
import { ConfirmButton } from "@/ui/components/ConfirmButton";
import { CheckboxField } from "@/ui/components/CheckboxField";
import { FormField } from "@/ui/components/FormField";
import { TextInput } from "@/ui/components/TextInput";
import { Toolbar } from "@/ui/components/Toolbar";
import { Text } from "@/ui/components/Text";

type ConfigFormProps = {
  config: TargetConfig;
  isCreating?: boolean;
  isDeleting?: boolean;
  isDirty: boolean;
  isUpdating?: boolean;
  onChange: (config: TargetConfig) => void;
  onDelete?: (id: string) => void;
  onDiscard: () => void;
  onSave: () => void;
};

export function ConfigForm({
  config,
  isCreating = false,
  isDeleting = false,
  isDirty,
  isUpdating = false,
  onChange,
  onDelete,
  onDiscard,
  onSave,
}: ConfigFormProps) {
  const { t } = useTranslation();

  return (
    <div className="flex h-full flex-col gap-4">
      <div className="grid grid-cols-2 gap-3">
        <FormField label={t("configs.id")}>
          <TextInput
            spellCheck={false}
            autoComplete="off"
            autoCapitalize="off"
            autoCorrect="off"
            value={config.id}
            onChange={(event) =>
              onChange({ ...config, id: event.target.value })
            }
          />
        </FormField>
        <FormField label={t("configs.name")}>
          <TextInput
            spellCheck={false}
            autoComplete="off"
            autoCapitalize="off"
            autoCorrect="off"
            value={config.name}
            onChange={(event) =>
              onChange({ ...config, name: event.target.value })
            }
          />
        </FormField>
      </div>
      <FormField label={t("configs.skillsPath")}>
        <TextInput
          spellCheck={false}
          autoComplete="off"
          autoCapitalize="off"
          autoCorrect="off"
          value={config.skillsPath}
          onChange={(event) =>
            onChange({ ...config, skillsPath: event.target.value })
          }
        />
      </FormField>
      <FormField label={t("configs.instructionsPath")}>
        <TextInput
          spellCheck={false}
          autoComplete="off"
          autoCapitalize="off"
          autoCorrect="off"
          value={config.instructionsPath}
          onChange={(event) =>
            onChange({ ...config, instructionsPath: event.target.value })
          }
        />
      </FormField>
      <CheckboxField
        checked={config.enabled}
        onChange={(checked) => onChange({ ...config, enabled: checked })}
      >
        {t("configs.enabled")}
      </CheckboxField>
      <Text tone="muted">{t("configs.syncBehavior")}</Text>
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
            {t("configs.save")}
          </Button>
          {onDelete && (
            <ConfirmButton
              cancelLabel={t("common.cancel")}
              confirmLabel={t("common.delete")}
              dialogVariant="red"
              disabled={!config.id || isDeleting}
              heading={t("dialogs.deleteConfigHeading")}
              icon={<Trash2 size={15} />}
              message={t("configs.deleteConfig", {
                name: config.name || config.id,
              })}
              onConfirm={() => onDelete(config.id)}
              variant="red"
            >
              {t("common.delete")}
            </ConfirmButton>
          )}
        </div>
      </Toolbar>
    </div>
  );
}
