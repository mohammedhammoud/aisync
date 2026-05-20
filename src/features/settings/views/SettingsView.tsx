import { createLazyRoute } from "@tanstack/react-router";
import { openUrl } from "@tauri-apps/plugin-opener";
import { relaunch } from "@tauri-apps/plugin-process";
import { check } from "@tauri-apps/plugin-updater";
import { useLanguage } from "@/base/i18n/hooks/useLanguage";
import { useGlobalsStore } from "@/base/store/globalsStore";
import { useTheme } from "@/ui/theme/useTheme";
import { SettingsPanel } from "@/features/settings/components/SettingsPanel";
import { GitHubSyncSettings } from "@/features/github/components/GitHubSyncSettings";
import { UpdateNotice } from "@/features/settings/components/UpdateNotice";
import { Pane } from "@/ui/components/Pane";

function SettingsView() {
  const globals = useGlobalsStore((state) => state.globals);
  const availableUpdate = useGlobalsStore(
    (state) => state.globals?.availableUpdate,
  );
  const { changeLanguage, language } = useLanguage();
  const { colorScheme, setColorScheme } = useTheme();

  async function installUpdate(downloadUrl: string) {
    try {
      const update = await check();
      if (!update) {
        await openUrl(downloadUrl);
        return;
      }

      await update.downloadAndInstall();
      await relaunch();
    } catch {
      await openUrl(downloadUrl);
    }
  }

  return (
    <div className="flex h-full flex-col gap-4 overflow-hidden">
      {availableUpdate ? (
        <UpdateNotice
          onDownload={() => void installUpdate(availableUpdate.downloadUrl)}
          version={availableUpdate.version}
        />
      ) : null}
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
