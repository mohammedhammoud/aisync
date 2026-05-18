import { createLazyRoute } from "@tanstack/react-router";
import { useLanguage } from "@/base/i18n/hooks/useLanguage";
import { useGlobalsStore } from "@/base/store/globalsStore";
import { useTheme } from "@/ui/theme/useTheme";
import { SettingsPanel } from "@/features/settings/components/SettingsPanel";
import { GitHubSyncSettings } from "@/features/github/components/GitHubSyncSettings";
import { Pane } from "@/ui/components/Pane";

function SettingsView() {
  const globals = useGlobalsStore((state) => state.globals);
  const { changeLanguage, language } = useLanguage();
  const { colorScheme, setColorScheme } = useTheme();

  return (
    <div className="flex h-full flex-col gap-4 overflow-hidden">
      <Pane className="min-h-0">
        <SettingsPanel
          colorScheme={colorScheme}
          language={language}
          localRoot={globals?.setupPath ?? ""}
          onChangeColorScheme={setColorScheme}
          onChangeLanguage={changeLanguage}
        />
      </Pane>
      <Pane className="min-h-0">
        <GitHubSyncSettings />
      </Pane>
    </div>
  );
}

export const Route = createLazyRoute("/settings")({
  component: SettingsView,
});
