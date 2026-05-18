import AxeBuilder from "@axe-core/playwright";
import { expect, test } from "@playwright/test";
import { installTauriMock } from "@/base/e2e/mockTauri";

test.beforeEach(async ({ page }) => {
  await installTauriMock(page);
});

test("home page has no detectable accessibility violations", async ({
  page,
}) => {
  await page.goto("/");
  await expect(page).toHaveURL(/\/skills\/audit$/);
  await expect(page.getByRole("heading", { level: 1 })).toBeVisible();

  const results = await new AxeBuilder({ page }).analyze();

  expect(results.violations).toEqual([]);
});
