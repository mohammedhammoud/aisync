import {
  createMemoryHistory,
  createRootRoute,
  createRoute,
  createRouter,
  RouterProvider,
} from "@tanstack/react-router";
import type { Meta, StoryObj } from "@storybook/react-vite";
import type { ComponentProps } from "react";
import type { TargetConfig } from "@/base/tauri/bindings";
import { ConfigList } from "./ConfigList";

const mockConfig: TargetConfig = {
  id: "mock-target",
  name: "Mock Target",
  skillsPath: "/mock/skills",
  instructionsPath: "/mock/instructions.md",
  enabled: true,
};

function StoryRouter(
  args: ComponentProps<typeof ConfigList> & { initialPath?: string },
) {
  const { initialPath = "/configs/mock-target", ...listArgs } = args;
  const rootRoute = createRootRoute({
    component: () => <ConfigList {...listArgs} />,
  });
  const configsRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/configs",
  });
  const configRoute = createRoute({
    getParentRoute: () => configsRoute,
    path: "$configId",
  });
  const newConfigRoute = createRoute({
    getParentRoute: () => configsRoute,
    path: "new",
  });
  const router = createRouter({
    history: createMemoryHistory({ initialEntries: [initialPath] }),
    routeTree: rootRoute.addChildren([
      configsRoute.addChildren([configRoute, newConfigRoute]),
    ]),
  });

  return <RouterProvider router={router} />;
}

const meta = {
  title: "Features/Configs/ConfigList",
  component: ConfigList,
  args: {
    configs: [mockConfig],
    onCreate: () => undefined,
  },
  render: (args) => <StoryRouter {...args} />,
} satisfies Meta<typeof ConfigList>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Empty: Story = {
  args: {
    configs: [],
  },
};
