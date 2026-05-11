import { afterEach, describe, expect, test, vi } from "vitest";
import {
  applyResolvedColorScheme,
  getColorSchemePreference,
  getSystemColorScheme,
  resolveColorScheme,
  setColorSchemePreference,
  COLOR_SCHEME_KEY,
  SYSTEM_DARK_QUERY,
} from "./systemColorScheme";

describe("systemColorScheme", () => {
  afterEach(() => {
    window.localStorage.clear();
    document.documentElement.className = "";
    document.documentElement.removeAttribute("style");
    vi.restoreAllMocks();
  });

  test("resolves system color scheme", () => {
    expect(resolveColorScheme("system", "dark")).toBe("dark");
    expect(resolveColorScheme("system", "light")).toBe("light");
  });

  test("uses explicit color scheme before system", () => {
    expect(resolveColorScheme("light", "dark")).toBe("light");
    expect(resolveColorScheme("dark", "light")).toBe("dark");
  });

  test("reads system color scheme from matchMedia", () => {
    vi.spyOn(window, "matchMedia").mockReturnValue({
      matches: true,
    } as MediaQueryList);

    expect(getSystemColorScheme()).toBe("dark");
    expect(window.matchMedia).toHaveBeenCalledWith(SYSTEM_DARK_QUERY);
  });

  test("falls back to system for invalid stored preference", () => {
    window.localStorage.setItem(COLOR_SCHEME_KEY, "blue");

    expect(getColorSchemePreference()).toBe("system");
  });

  test("returns valid stored preference", () => {
    window.localStorage.setItem(COLOR_SCHEME_KEY, "dark");

    expect(getColorSchemePreference()).toBe("dark");
  });

  test("stores color scheme preference", () => {
    setColorSchemePreference("light");

    expect(window.localStorage.getItem(COLOR_SCHEME_KEY)).toBe("light");
  });

  test("applies dark resolved color scheme", () => {
    applyResolvedColorScheme("dark");

    expect(document.documentElement).toHaveClass("dark");
    expect(document.documentElement.style.colorScheme).toBe("dark");
  });

  test("applies light resolved color scheme", () => {
    document.documentElement.classList.add("dark");

    applyResolvedColorScheme("light");

    expect(document.documentElement).not.toHaveClass("dark");
    expect(document.documentElement.style.colorScheme).toBe("light");
  });
});
