import type { KnipConfig } from "knip";

const config: KnipConfig = {
  entry: ["src/**/*.stories.tsx", "e2e/**/*.ts", ".storybook/**/*.ts"],
  project: [
    "src/**/*.{ts,tsx}",
    "e2e/**/*.ts",
    ".storybook/**/*.{ts,tsx}",
    "*.{ts,js}",
  ],
  ignoreDependencies: ["@tauri-apps/plugin-opener", "tailwindcss"],
  ignore: ["src/base/tauri/bindings.ts"],
};

export default config;
