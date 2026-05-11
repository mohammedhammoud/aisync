import { createLazyRoute } from "@tanstack/react-router";
import { useLanguage } from "@/base/i18n/hooks/useLanguage";
import { useGlobalsStore } from "@/base/store/globalsStore";
import { useTheme } from "@/ui/theme/useTheme";
import { SettingsPanel } from "@/features/settings/components/SettingsPanel";
import { Pane } from "@/ui/components/Pane";

function SettingsView() {
  const globals = useGlobalsStore((state) => state.globals);
  const { changeLanguage, language } = useLanguage();
  const { colorScheme, setColorScheme } = useTheme();

  return (
    <Pane>
      <SettingsPanel
        colorScheme={colorScheme}
        language={language}
        localRoot={globals?.setupPath ?? ""}
        onChangeColorScheme={setColorScheme}
        onChangeLanguage={changeLanguage}
      />
    </Pane>
  );
}

export const Route = createLazyRoute("/settings")({
  component: SettingsView,
});
