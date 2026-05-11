import { useTranslation } from "react-i18next";
import { FormField } from "@/ui/components/FormField";
import { SelectInput } from "@/ui/components/SelectInput";
import type { ColorScheme } from "@/base/root/theme/systemColorScheme";
import { TextInput } from "@/ui/components/TextInput";

type SettingsPanelProps = {
  colorScheme: ColorScheme;
  language: string;
  localRoot: string;
  onChangeColorScheme: (colorScheme: ColorScheme) => void;
  onChangeLanguage: (language: string) => void;
};

export function SettingsPanel({
  colorScheme,
  language,
  localRoot,
  onChangeColorScheme,
  onChangeLanguage,
}: SettingsPanelProps) {
  const { t } = useTranslation();

  return (
    <div className="grid gap-4">
      <FormField label={t("settings.localRootLabel")}>
        <TextInput readOnly value={localRoot} />
      </FormField>
      <FormField label={t("settings.language")}>
        <SelectInput
          value={language}
          onChange={(event) => onChangeLanguage(event.target.value)}
        >
          <option value="en">{t("settings.english")}</option>
          <option value="sv">{t("settings.swedish")}</option>
        </SelectInput>
      </FormField>
      <FormField label={t("settings.theme")}>
        <SelectInput
          value={colorScheme}
          onChange={(event) =>
            onChangeColorScheme(event.target.value as ColorScheme)
          }
        >
          <option value="system">{t("settings.themeSystem")}</option>
          <option value="light">{t("settings.themeLight")}</option>
          <option value="dark">{t("settings.themeDark")}</option>
        </SelectInput>
      </FormField>
    </div>
  );
}
