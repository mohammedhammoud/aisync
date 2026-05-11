import { createRoute } from "@tanstack/react-router";
import type { rootRoute } from "@/base/root/routes";

const _SettingsView = () =>
  import("./views/SettingsView").then((module) => module.Route);

export function createSettingsRoute(parentRoute: typeof rootRoute) {
  return createRoute({
    getParentRoute: () => parentRoute,
    path: "/settings",
    staticData: {
      i18nTitleKey: "settings.title",
    },
  }).lazy(_SettingsView);
}
