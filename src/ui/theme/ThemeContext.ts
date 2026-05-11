import { createContext } from "react";
import type { ColorScheme } from "@/base/root/theme/systemColorScheme";
import type { globalClasses } from "@/ui/theme/globalClasses";
import type { Variant, VariantType } from "@/ui/theme/variants";

type ThemeContextValue = {
  globalClasses: typeof globalClasses;
  colorScheme: ColorScheme;
  getVariant: (variant: Variant) => VariantType;
  setColorScheme: (colorScheme: ColorScheme) => void;
};

export const ThemeContext = createContext<ThemeContextValue | null>(null);
