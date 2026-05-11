import { expect, test } from "@playwright/test";
import { installTauriMock } from "@/base/e2e/mockTauri";

test.beforeEach(async ({ page }) => {
  await installTauriMock(page);
});

test("opens instructions and shows editor", async ({ page }) => {
  await page.goto("/instructions");

  await expect(page).toHaveURL("/instructions");
  await expect(page.getByLabel("Global instructions")).toBeVisible();
});
