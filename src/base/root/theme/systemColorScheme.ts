export type ColorScheme = "light" | "dark" | "system";
export type ResolvedColorScheme = Exclude<ColorScheme, "system">;

export const COLOR_SCHEME_KEY = "app:colorScheme";
export const SYSTEM_DARK_QUERY = "(prefers-color-scheme: dark)";

const colorSchemes = ["light", "dark", "system"] as const;

function isColorScheme(value: string | null): value is ColorScheme {
  return colorSchemes.includes(value as ColorScheme);
}

export function getSystemColorScheme(): ResolvedColorScheme {
  return window.matchMedia(SYSTEM_DARK_QUERY).matches ? "dark" : "light";
}

export function resolveColorScheme(
  colorScheme: ColorScheme,
  systemColorScheme = getSystemColorScheme(),
): ResolvedColorScheme {
  return colorScheme === "system" ? systemColorScheme : colorScheme;
}

export function applyResolvedColorScheme(
  resolvedColorScheme: ResolvedColorScheme,
) {
  document.documentElement.classList.toggle(
    "dark",
    resolvedColorScheme === "dark",
  );
  document.documentElement.style.colorScheme = resolvedColorScheme;
}

export function getColorSchemePreference(): ColorScheme {
  const storedColorScheme = window.localStorage.getItem(COLOR_SCHEME_KEY);

  return isColorScheme(storedColorScheme) ? storedColorScheme : "system";
}

export function setColorSchemePreference(colorScheme: ColorScheme) {
  window.localStorage.setItem(COLOR_SCHEME_KEY, colorScheme);
}
