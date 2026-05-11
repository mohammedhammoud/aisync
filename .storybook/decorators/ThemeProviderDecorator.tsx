import type { Decorator } from "@storybook/react-vite";
import type { InputType } from "storybook/internal/csf";
import { useEffect, type PropsWithChildren } from "react";
import {
  getColorSchemePreference,
  type ColorScheme,
} from "@/base/root/theme/systemColorScheme";
import { ThemeProvider } from "@/ui/theme/ThemeProvider";
import { useTheme } from "@/ui/theme/useTheme";

type ExtendedDecorator = Decorator & {
  globalType?: {
    colorScheme: InputType;
  };
};

type ThemeSyncProps = PropsWithChildren<{
  colorScheme: ColorScheme;
}>;

function isColorScheme(value: unknown): value is ColorScheme {
  return value === "light" || value === "dark" || value === "system";
}

function ThemeSync({ children, colorScheme }: ThemeSyncProps) {
  const { setColorScheme } = useTheme();

  useEffect(() => {
    setColorScheme(colorScheme);
  }, [colorScheme, setColorScheme]);

  return <>{children}</>;
}

export const ThemeProviderDecorator: ExtendedDecorator = (Story, context) => {
  const colorScheme = isColorScheme(context.globals.colorScheme)
    ? context.globals.colorScheme
    : "system";

  return (
    <ThemeProvider>
      <ThemeSync colorScheme={colorScheme}>
        <Story />
      </ThemeSync>
    </ThemeProvider>
  );
};

ThemeProviderDecorator.globalType = {
  colorScheme: {
    defaultValue: getColorSchemePreference(),
    description: "Color scheme",
    toolbar: {
      dynamicTitle: true,
      icon: "circlehollow",
      items: [
        { title: "System", value: "system" },
        { title: "Light", value: "light" },
        { title: "Dark", value: "dark" },
      ],
      title: "Theme",
    },
  },
};
