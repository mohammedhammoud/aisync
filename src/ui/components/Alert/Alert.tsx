import type { HTMLAttributes, PropsWithChildren } from "react";
import { cx } from "@/base/utils/cx";
import { useTheme } from "@/ui/theme/useTheme";
import type { Variant, VariantMode } from "@/ui/theme/variants";

type AlertProps = PropsWithChildren<{
  variant?: Variant;
  className?: string;
  mode?: VariantMode;
}> &
  HTMLAttributes<HTMLDivElement>;

export function Alert({
  children,
  variant = "neutral",
  className,
  mode = "solid",
  ...props
}: AlertProps) {
  const { getVariant } = useTheme();
  const v = getVariant(variant)[mode];
  return (
    <div
      className={cx(
        "rounded px-3 py-2 text-xs",
        v.base.background,
        v.border,
        v.base.readableColor,
        className,
      )}
      role="status"
      {...props}
    >
      {children}
    </div>
  );
}
