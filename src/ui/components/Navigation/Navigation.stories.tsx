import {
  createMemoryHistory,
  createRootRoute,
  createRoute,
  createRouter,
  RouterProvider,
} from "@tanstack/react-router";
import { expect, userEvent, within } from "storybook/test";
import { FileText, Settings, Sparkles } from "lucide-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { Navigation } from "./Navigation";

type StoryRouterProps = {
  disabled?: boolean;
  initialPath?: string;
};

function NavigationStory({ disabled = false }: { disabled?: boolean }) {
  return (
    <Navigation aria-label="Primary navigation" className="w-48">
      <Navigation.Item
        disabled={disabled}
        icon={Sparkles}
        label="Skills"
        to="/skills"
      />
      <Navigation.Item
        disabled={disabled}
        icon={Settings}
        label="Configurations"
        to="/configs"
      />
      <Navigation.Item
        disabled={disabled}
        icon={FileText}
        label="Instructions"
        to="/instructions"
      />
      <Navigation.Item
        disabled={disabled}
        icon={Settings}
        label="Settings"
        to="/settings"
      />
    </Navigation>
  );
}

function StoryRouter({
  disabled = false,
  initialPath = "/skills",
}: StoryRouterProps) {
  const rootRoute = createRootRoute({
    component: () => <NavigationStory disabled={disabled} />,
  });
  const skillsRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/skills",
  });
  const configRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/configs",
  });
  const instructionsRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/instructions",
  });
  const settingsRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/settings",
  });
  const router = createRouter({
    history: createMemoryHistory({ initialEntries: [initialPath] }),
    routeTree: rootRoute.addChildren([
      skillsRoute,
      configRoute,
      instructionsRoute,
      settingsRoute,
    ]),
  });

  return <RouterProvider router={router} />;
}

const meta = {
  title: "UI/Navigation",
  component: Navigation,
  args: {
    "aria-label": "Primary navigation",
    children: null,
  },
} satisfies Meta<typeof Navigation>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => <StoryRouter />,
};

export const ActiveInstructions: Story = {
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      canvas.getByRole("link", { current: "page", name: "Instructions" }),
    ).toBeVisible();
  },
  render: () => <StoryRouter initialPath="/instructions" />,
};

export const Disabled: Story = {
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(canvas.getByRole("link", { name: "Instructions" }));
    await expect(
      canvas.getByRole("link", { current: "page", name: "Skills" }),
    ).toBeVisible();
  },
  render: () => <StoryRouter disabled />,
};
