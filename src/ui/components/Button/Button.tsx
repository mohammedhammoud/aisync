import { Button as HeadlessButton } from "@headlessui/react";
import type { ButtonHTMLAttributes, ReactNode } from "react";
import { cx } from "@/base/utils/cx";
import { useTheme } from "@/ui/theme/useTheme";
import type { Variant, VariantMode } from "@/ui/theme/variants";

type ButtonSize = "sm" | "md";

type ButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & {
  icon?: ReactNode;
  iconOnly?: boolean;
  variant?: Variant;
  mode?: VariantMode;
  full?: boolean;
  size?: ButtonSize;
};

const sizeClassNames: Record<ButtonSize, string> = {
  md: "h-9 px-3 text-xs",
  sm: "h-8 px-2 text-xs",
};

const iconOnlySizeClassNames: Record<ButtonSize, string> = {
  md: "h-9 w-9",
  sm: "h-6 w-6",
};

export function Button({
  children,
  className = "",
  full = false,
  icon,
  iconOnly = false,
  size = "md",
  type = "button",
  variant = "neutral",
  mode = "solid",
  ...props
}: ButtonProps) {
  const { globalClasses, getVariant } = useTheme();
  const v = getVariant(variant)[mode];

  return (
    <HeadlessButton
      className={cx(
        "inline-flex items-center justify-center gap-2 rounded",
        v.focus,
        "cursor-pointer disabled:cursor-not-allowed font-semibold",
        globalClasses.disabledOpacity,
        v.base.background,
        v.border,
        v.base.readableColor,
        iconOnly ? iconOnlySizeClassNames[size] : sizeClassNames[size],
        !props.disabled && v.hover.background,
        !props.disabled && [v.active.background, v.active.readableColor],
        full && "w-full",
        className,
      )}
      type={type}
      {...props}
    >
      {icon}
      {children}
    </HeadlessButton>
  );
}
