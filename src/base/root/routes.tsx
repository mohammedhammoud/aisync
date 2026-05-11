import { createRootRoute, createRoute, redirect } from "@tanstack/react-router";
import { createConfigRoutes } from "@/features/configs/routes";
import { createInstructionsRoute } from "@/features/instructions/routes";
import { createSettingsRoute } from "@/features/settings/routes";
import { createSkillRoutes } from "@/features/skills/routes";
import { ShellLayout } from "./layouts/ShellLayout";

export const rootRoute = createRootRoute({
  component: ShellLayout,
});

const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  beforeLoad: () => {
    throw redirect({ to: "/skills" });
  },
});

export const routeTree = rootRoute.addChildren([
  indexRoute,
  createSkillRoutes(rootRoute),
  createConfigRoutes(rootRoute),
  createInstructionsRoute(rootRoute),
  createSettingsRoute(rootRoute),
]);
