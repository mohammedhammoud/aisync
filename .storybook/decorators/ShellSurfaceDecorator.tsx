import type { Decorator } from "@storybook/react-vite";
import { ShellSurface } from "@/base/root/theme/ShellSurface";

export const ShellSurfaceDecorator: Decorator = (Story) => (
  <ShellSurface className="min-h-screen p-6">
    <Story />
  </ShellSurface>
);
