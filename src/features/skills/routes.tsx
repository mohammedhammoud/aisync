import { createRoute, redirect } from "@tanstack/react-router";
import type { rootRoute } from "@/base/root/routes";
import { commands } from "@/base/tauri/bindings";

const _SkillsRootView = () =>
  import("./views/SkillsRootView").then((module) => module.Route);
const _SkillsDetailView = () =>
  import("./views/SkillsDetailView").then((module) => module.Route);
const _SkillsCreateView = () =>
  import("./views/SkillsCreateView").then((module) => module.Route);

export function createSkillRoutes(parentRoute: typeof rootRoute) {
  const skillsRoute = createRoute({
    beforeLoad: async ({ location }) => {
      if (location.pathname !== "/skills") {
        return;
      }

      const skills = await commands.getSkills();
      const firstSkill = skills[0];
      if (firstSkill) {
        throw redirect({
          replace: true,
          to: "/skills/$skillId",
          params: { skillId: firstSkill.id },
        });
      }
    },
    getParentRoute: () => parentRoute,
    path: "/skills",
    staticData: {
      i18nTitleKey: "skills.title",
    },
  }).lazy(_SkillsRootView);

  const skillDetailRoute = createRoute({
    getParentRoute: () => skillsRoute,
    path: "$skillId",
  }).lazy(_SkillsDetailView);

  const skillCreateRoute = createRoute({
    getParentRoute: () => skillsRoute,
    path: "new",
  }).lazy(_SkillsCreateView);

  return skillsRoute.addChildren([skillDetailRoute, skillCreateRoute]);
}
