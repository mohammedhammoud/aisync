import { expect, test } from "@playwright/test";
import { installTauriMock } from "@/base/e2e/mockTauri";

test.beforeEach(async ({ page }) => {
  await installTauriMock(page);
});

test("opens configs and renders selected config", async ({ page }) => {
  await page.goto("/configs");

  await expect(page).toHaveURL(/\/configs\/local$/);
  await expect(page.getByLabel("ID")).toHaveValue("local");
  await expect(page.getByLabel("Name")).toHaveValue("Local project");
});
