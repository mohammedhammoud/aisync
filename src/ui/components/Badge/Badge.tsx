import type { HTMLAttributes, ReactNode } from "react";
import { cx } from "@/base/utils/cx";
import { useTheme } from "@/ui/theme/useTheme";
import type { Variant, VariantMode } from "@/ui/theme/variants";

type BadgeSize = "sm" | "md";

type BadgeProps = HTMLAttributes<HTMLSpanElement> & {
  children: ReactNode;
  mode?: VariantMode;
  size?: BadgeSize;
  variant?: Variant;
};

const sizeClassNames: Record<BadgeSize, string> = {
  sm: "h-4 min-w-4 px-1.5 text-[0.6875rem]",
  md: "h-5 min-w-5 px-2 text-xs",
};

export function Badge({
  children,
  className,
  mode = "solid",
  size = "md",
  variant = "neutral",
  ...props
}: BadgeProps) {
  const { getVariant } = useTheme();
  const v = getVariant(variant)[mode];

  return (
    <span
      className={cx(
        "inline-flex shrink-0 items-center justify-center rounded-full font-semibold leading-none",
        sizeClassNames[size],
        v.base.background,
        v.base.readableColor,
        mode === "outline" && v.border,
        className,
      )}
      {...props}
    >
      {children}
    </span>
  );
}
