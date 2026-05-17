import { describe, expect, test, vi } from "vitest";
import { translateTauriError } from "./translateTauriError";

vi.mock("@/base/i18n/client", () => ({
  i18n: {
    t: (key: string, options?: { defaultValue?: string }) => {
      const translations: Record<string, string> = {
        "errors.skill_not_found": "Skill not found.",
        "errors.unknown": "Something went wrong.",
      };

      return translations[key] ?? options?.defaultValue ?? key;
    },
  },
}));

describe("translateTauriError", () => {
  test("stringifies non app errors", () => {
    expect(translateTauriError("boom")).toBe("boom");
    expect(translateTauriError(404)).toBe("404");
  });

  test("translates known app error code", () => {
    expect(
      translateTauriError({ code: "skill_not_found", message: "missing" }),
    ).toBe("Skill not found.");
  });

  test("falls back to app error message for unknown app error code", () => {
    expect(translateTauriError({ code: "not_real", message: "missing" })).toBe(
      "missing",
    );
  });

  test("shows system unknown error message", () => {
    expect(translateTauriError({ code: "unknown", message: "network" })).toBe(
      "network",
    );
  });

  test("treats invalid app error shape as unknown value", () => {
    expect(translateTauriError({ code: "skill_not_found" })).toBe(
      "[object Object]",
    );
  });
});
