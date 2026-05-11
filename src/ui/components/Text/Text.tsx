import type { ElementType, ReactNode } from "react";
import { cx } from "@/base/utils/cx";
import { useTheme } from "@/ui/theme/useTheme";

type TextTone = "primary" | "secondary" | "muted" | "subtle";

type TextProps<T extends ElementType> = {
  as?: T;
  children: ReactNode;
  className?: string;
  tone?: TextTone;
} & Omit<React.ComponentProps<T>, "className">;

export function Text<T extends ElementType = "span">({
  as,
  children,
  className,
  tone = "primary",
  ...rest
}: TextProps<T>) {
  const { globalClasses } = useTheme();
  const Component = as ?? "span";
  const toneClass: Record<TextTone, string> = {
    primary: globalClasses.textPrimary,
    secondary: globalClasses.textSecondary,
    muted: globalClasses.textMuted,
    subtle: globalClasses.textSubtle,
  };

  return (
    <Component className={cx(toneClass[tone], className)} {...rest}>
      {children}
    </Component>
  );
}
