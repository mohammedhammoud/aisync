import { beforeAll, describe, expect, test } from "vitest";
import i18n from "@/i18n";
import { translateTauriError } from "./translateTauriError";

describe("translateTauriError", () => {
  beforeAll(async () => {
    await i18n.changeLanguage("en");
  });

  test("stringifies non app errors", () => {
    expect(translateTauriError("boom")).toBe("boom");
    expect(translateTauriError(404)).toBe("404");
  });

  test("translates known app error code", () => {
    expect(
      translateTauriError({ code: "skill_not_found", message: "missing" }),
    ).toBe("Skill not found.");
  });

  test("falls back to unknown translation for unknown app error code", () => {
    expect(translateTauriError({ code: "not_real", message: "missing" })).toBe(
      "Something went wrong.",
    );
  });

  test("treats invalid app error shape as unknown value", () => {
    expect(translateTauriError({ code: "skill_not_found" })).toBe(
      "[object Object]",
    );
  });
});
