import { createRoute } from "@tanstack/react-router";
import type { rootRoute } from "@/base/root/routes";

const _InstructionsView = () =>
  import("./views/InstructionsView").then((module) => module.Route);

export function createInstructionsRoute(parentRoute: typeof rootRoute) {
  return createRoute({
    getParentRoute: () => parentRoute,
    path: "/instructions",
    staticData: {
      i18nTitleKey: "instructions.title",
    },
  }).lazy(_InstructionsView);
}
