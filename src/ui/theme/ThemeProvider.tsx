import {
  useCallback,
  useEffect,
  useMemo,
  useState,
  type PropsWithChildren,
} from "react";
import {
  applyResolvedColorScheme,
  getColorSchemePreference,
  getSystemColorScheme,
  resolveColorScheme,
  setColorSchemePreference,
  SYSTEM_DARK_QUERY,
  type ColorScheme,
  type ResolvedColorScheme,
} from "@/base/root/theme/systemColorScheme";
import { globalClasses } from "@/ui/theme/globalClasses";
import { ThemeContext } from "@/ui/theme/ThemeContext";
import { VARIANT, type Variant, type VariantType } from "@/ui/theme/variants";

type ThemeOverrides = {
  globalClasses?: typeof globalClasses;
  variants?: Record<Variant, VariantType>;
};

type ThemeProviderProps = PropsWithChildren<{
  overrides?: ThemeOverrides;
}>;

export function ThemeProvider({
  children,
  overrides = {},
}: ThemeProviderProps) {
  const [colorScheme, setColorSchemeState] = useState<ColorScheme>(
    getColorSchemePreference,
  );
  const [systemColorScheme, setSystemColorScheme] =
    useState<ResolvedColorScheme>(getSystemColorScheme);
  const resolvedColorScheme = resolveColorScheme(
    colorScheme,
    systemColorScheme,
  );
  const themeVariants = overrides.variants ?? VARIANT;
  const classes = overrides.globalClasses ?? globalClasses;

  const setColorScheme = useCallback((nextColorScheme: ColorScheme) => {
    setColorSchemePreference(nextColorScheme);
    setColorSchemeState(nextColorScheme);
  }, []);

  useEffect(() => {
    const mediaQuery = window.matchMedia(SYSTEM_DARK_QUERY);
    const handleChange = () =>
      setSystemColorScheme(mediaQuery.matches ? "dark" : "light");

    handleChange();
    mediaQuery.addEventListener("change", handleChange);
    return () => mediaQuery.removeEventListener("change", handleChange);
  }, []);

  useEffect(() => {
    applyResolvedColorScheme(resolvedColorScheme);
  }, [resolvedColorScheme]);

  const value = useMemo(
    () => ({
      globalClasses: classes,
      colorScheme,
      getVariant: (variant: Variant) => themeVariants[variant],
      setColorScheme,
    }),
    [classes, colorScheme, setColorScheme, themeVariants],
  );

  return (
    <ThemeContext.Provider value={value}>{children}</ThemeContext.Provider>
  );
}
