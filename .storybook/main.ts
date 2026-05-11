import type { StorybookConfig } from "@storybook/react-vite";
import { mergeConfig } from "vite";
import { fileURLToPath, URL } from "node:url";

const config: StorybookConfig = {
  stories: ["../src/**/*.stories.@(ts|tsx)"],
  framework: {
    name: "@storybook/react-vite",
    options: {},
  },
  async viteFinal(config) {
    const tailwindcss = (await import("@tailwindcss/vite")).default();

    return mergeConfig(config, {
      plugins: [tailwindcss],
      resolve: {
        alias: {
          "@": fileURLToPath(new URL("../src", import.meta.url)),
        },
      },
    });
  },
};

export default config;
