import { Outlet, useBlocker, useMatches } from "@tanstack/react-router";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { FileText, Settings, SlidersHorizontal, Sparkles } from "lucide-react";
import { useTranslation } from "react-i18next";
import { cx } from "@/base/utils/cx";
import { AppHeader } from "@/base/root/components/AppHeader";
import { useAppLockContext } from "@/base/root/appLock";
import { ShellSurface } from "@/base/root/theme/ShellSurface";
import { useGithubSyncNavigationBadge } from "@/features/github/hooks/useGithubSyncNavigationBadge";
import { useGlobalsStore } from "@/base/store/globalsStore";
import { Navigation } from "@/ui/components/Navigation";
import { AppLogo } from "@/ui/components/AppLogo";
import { Text } from "@/ui/components/Text";
import { useTheme } from "@/ui/theme/useTheme";

export function ShellLayout() {
  const { t } = useTranslation();
  const { isLocked, lockMessage } = useAppLockContext();
  const { githubSyncBadge } = useGithubSyncNavigationBadge();
  const availableUpdate = useGlobalsStore(
    (state) => state.globals?.availableUpdate,
  );
  const { globalClasses, getVariant } = useTheme();

  const v = {
    skipToContent: getVariant("yellow"),
  };

  useBlocker({
    shouldBlockFn: ({ current, next }) =>
      current.pathname !== next.pathname && isLocked,
    withResolver: true,
  });

  const titleKey = useMatches({
    select: (matches) => {
      for (let index = matches.length - 1; index >= 0; index -= 1) {
        const staticData = matches[index].staticData;
        if (
          staticData &&
          typeof staticData === "object" &&
          "i18nTitleKey" in staticData
        ) {
          return String(staticData.i18nTitleKey);
        }
      }
      return "skills.title";
    },
  });
  const title = t(titleKey);
  const settingsBadge = availableUpdate
    ? { label: t("updates.availableShort"), variant: "violet" as const }
    : githubSyncBadge;

  function startWindowDrag() {
    if (!("__TAURI_INTERNALS__" in window)) {
      return;
    }
    getCurrentWindow()
      .startDragging()
      .catch(() => {});
  }

  return (
    <ShellSurface className="relative grid h-screen min-w-3xl grid-cols-[14rem_1fr] overflow-hidden">
      <Text
        as="a"
        className={cx(
          "sr-only z-50 rounded px-3 py-2 focus:not-sr-only focus:absolute focus:left-3 focus:top-3",
          v.skipToContent.solid.base.background,
          v.skipToContent.solid.base.readableColor,
        )}
        href="#main-content"
      >
        {t("nav.skipToContent")}
      </Text>
      <div
        className="absolute inset-x-0 top-0 z-10 h-10"
        data-tauri-drag-region
        onPointerDown={startWindowDrag}
      />
      <aside
        className={cx(
          "flex flex-col gap-4 border-r px-6 pb-6 pt-16",
          globalClasses.sidebarBorder,
          globalClasses.sidebarBackground,
        )}
      >
        <Navigation aria-label={t("nav.primary")}>
          <Navigation.Item
            disabled={isLocked}
            icon={Sparkles}
            label={t("nav.skills")}
            to="/skills"
          />
          <Navigation.Item
            disabled={isLocked}
            icon={SlidersHorizontal}
            label={t("nav.configs")}
            to="/configs"
          />
          <Navigation.Item
            disabled={isLocked}
            icon={FileText}
            label={t("nav.instructions")}
            to="/instructions"
          />
          <Navigation.Item
            badge={settingsBadge}
            disabled={isLocked}
            icon={Settings}
            label={t("nav.settings")}
            to="/settings"
          />
        </Navigation>
        <div className="mt-auto flex justify-center pt-4">
          <AppLogo />
        </div>
      </aside>

      <main
        className="grid min-w-0 grid-rows-[auto_minmax(0,1fr)] overflow-hidden p-6"
        id="main-content"
      >
        <AppHeader lockMessage={lockMessage} title={title} />
        <div className="min-h-0 overflow-hidden">
          <Outlet />
        </div>
      </main>
    </ShellSurface>
  );
}
