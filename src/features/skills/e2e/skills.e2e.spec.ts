import { expect, test } from "@playwright/test";
import { installTauriMock } from "@/base/e2e/mockTauri";

test.beforeEach(async ({ page }) => {
  await installTauriMock(page);
});

test("redirects to first skill and shows skill details", async ({ page }) => {
  await page.goto("/");

  await expect(page).toHaveURL(/\/skills\/audit$/);
  await expect(page.getByLabel("ID")).toHaveValue("audit");
  await expect(page.getByLabel("Name")).toHaveValue("Audit");
});

test("locks navigation on unsaved skill create form", async ({ page }) => {
  await page.goto("/skills/new");

  await expect(page).toHaveURL("/skills/new");
  await expect(page.locator('a[href="/configs"]')).toHaveAttribute(
    "aria-disabled",
    "true",
  );
  await expect(page.getByRole("status")).toContainText(/save|spara/i);
});
