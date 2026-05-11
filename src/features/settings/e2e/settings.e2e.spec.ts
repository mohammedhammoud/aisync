import { expect, test } from "@playwright/test";
import { installTauriMock } from "@/base/e2e/mockTauri";

test.beforeEach(async ({ page }) => {
  await installTauriMock(page);
});

test("opens settings and shows language/theme controls", async ({ page }) => {
  await page.goto("/settings");

  await expect(page).toHaveURL("/settings");
  await expect(page.getByLabel("Language")).toBeVisible();
  await expect(page.getByLabel("Theme")).toBeVisible();
});
