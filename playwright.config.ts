import { defineConfig, devices } from "@playwright/test";

export default defineConfig({
  testDir: ".",
  testMatch: ["**/*.e2e.spec.ts"],
  webServer: {
    command: "pnpm dev --host 127.0.0.1",
    reuseExistingServer: !process.env.CI,
    url: "http://127.0.0.1:1420",
  },
  use: {
    baseURL: "http://127.0.0.1:1420",
    trace: "on-first-retry",
  },
  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },
  ],
});
