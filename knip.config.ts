import type { KnipConfig } from "knip";

const config: KnipConfig = {
  entry: ["src/**/*.stories.tsx", "src/**/*.e2e.spec.ts", ".storybook/**/*.ts"],
  project: ["src/**/*.{ts,tsx}", ".storybook/**/*.{ts,tsx}", "*.{ts,js}"],
  ignoreDependencies: ["tailwindcss"],
  ignore: ["src/base/tauri/bindings.ts"],
};

export default config;
