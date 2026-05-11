import "@/i18n";
import "../tailwind.css";

import type { Preview } from "@storybook/react-vite";
import { ShellSurfaceDecorator } from "./decorators/ShellSurfaceDecorator";
import { ThemeProviderDecorator } from "./decorators/ThemeProviderDecorator";
import { ToastDecorator } from "./decorators/ToastDecorator";

const preview: Preview = {
  decorators: [ShellSurfaceDecorator, ToastDecorator, ThemeProviderDecorator],
  globalTypes: {
    ...(ThemeProviderDecorator.globalType ?? {}),
  },
  parameters: {
    layout: "fullscreen",
    backgrounds: { disable: true },
  },
};

export default preview;
