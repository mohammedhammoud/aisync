import { initI18n } from "@/base/i18n/client";
import "../tailwind.css";

import type { Preview } from "@storybook/react-vite";
import { ShellSurfaceDecorator } from "./decorators/ShellSurfaceDecorator";
import { ThemeProviderDecorator } from "./decorators/ThemeProviderDecorator";
import { ToastDecorator } from "./decorators/ToastDecorator";

initI18n();

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
