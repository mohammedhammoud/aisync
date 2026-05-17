import {
  createMemoryHistory,
  createRootRoute,
  createRoute,
  createRouter,
  RouterProvider,
} from "@tanstack/react-router";
import type { Meta, StoryObj } from "@storybook/react-vite";
import type { ComponentProps } from "react";
import { SkillList } from "./SkillList";

const skills = [
  {
    id: "audit",
    name: "Audit",
    description: "Review changed code",
    enabled: true,
    tags: [],
  },
  {
    id: "debug",
    name: "Debug",
    description: "Find root cause",
    enabled: true,
    tags: [],
  },
  {
    id: "test",
    name: "Test",
    description: "Run tests",
    enabled: true,
    tags: [],
  },
  {
    id: "document",
    name: "Document",
    description: "Generate documentation",
    enabled: true,
    tags: [],
  },
  {
    id: "refactor",
    name: "Refactor",
    description: "Improve code structure",
    enabled: true,
    tags: [],
  },
  {
    id: "optimize",
    name: "Optimize",
    description: "Enhance performance",
    enabled: true,
    tags: [],
  },
];

function StoryRouter(
  args: ComponentProps<typeof SkillList> & { initialPath?: string },
) {
  const { initialPath = "/skills/audit", ...listArgs } = args;
  const rootRoute = createRootRoute({
    component: () => <SkillList {...listArgs} />,
  });
  const skillsRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/skills",
  });
  const skillRoute = createRoute({
    getParentRoute: () => skillsRoute,
    path: "$skillId",
  });
  const newSkillRoute = createRoute({
    getParentRoute: () => skillsRoute,
    path: "new",
  });
  const router = createRouter({
    history: createMemoryHistory({ initialEntries: [initialPath] }),
    routeTree: rootRoute.addChildren([
      skillsRoute.addChildren([skillRoute, newSkillRoute]),
    ]),
  });

  return <RouterProvider router={router} />;
}

const meta = {
  title: "Features/Skills/SkillList",
  component: SkillList,
  args: {
    onCreate: () => undefined,
    skills,
  },
  render: (args) => <StoryRouter {...args} />,
} satisfies Meta<typeof SkillList>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Empty: Story = {
  args: {
    skills: [],
  },
};
