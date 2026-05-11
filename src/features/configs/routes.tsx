import { createRoute, redirect } from "@tanstack/react-router";
import type { rootRoute } from "@/base/root/routes";
import { commands } from "@/base/tauri/bindings";

const _ConfigsRootView = () =>
  import("./views/ConfigsRootView").then((module) => module.Route);
const _ConfigsDetailView = () =>
  import("./views/ConfigsDetailView").then((module) => module.Route);
const _ConfigsCreateView = () =>
  import("./views/ConfigsCreateView").then((module) => module.Route);

export function createConfigRoutes(parentRoute: typeof rootRoute) {
  const configRoute = createRoute({
    beforeLoad: async ({ location }) => {
      if (location.pathname !== "/configs") {
        return;
      }

      const configs = await commands.getConfigs();
      const firstConfig = configs[0];
      if (firstConfig) {
        throw redirect({
          replace: true,
          to: "/configs/$configId",
          params: { configId: firstConfig.id },
        });
      }
    },
    getParentRoute: () => parentRoute,
    path: "/configs",
    staticData: {
      i18nTitleKey: "configs.title",
    },
  }).lazy(_ConfigsRootView);

  const configDetailRoute = createRoute({
    getParentRoute: () => configRoute,
    path: "$configId",
  }).lazy(_ConfigsDetailView);

  const configCreateRoute = createRoute({
    getParentRoute: () => configRoute,
    path: "new",
  }).lazy(_ConfigsCreateView);

  return configRoute.addChildren([configDetailRoute, configCreateRoute]);
}
