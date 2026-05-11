import type { HTMLAttributes, PropsWithChildren } from "react";
import { cx } from "@/base/utils/cx";
import { useTheme } from "@/ui/theme/useTheme";
import type { Variant, VariantMode } from "@/ui/theme/variants";

type PaneProps = PropsWithChildren<HTMLAttributes<HTMLElement>> & {
  variant?: Variant;
  mode?: VariantMode;
};

export function Pane({
  children,
  className = "",
  variant = "neutral",
  mode = "outline",
  ...props
}: PaneProps) {
  const { globalClasses, getVariant } = useTheme();
  const v = getVariant(variant)[mode];

  return (
    <section
      className={cx(
        "overflow-auto rounded-md p-4 [&>*]:shrink-0",
        globalClasses.surfaceBackground,
        v.base.readableColor,
        className,
      )}
      {...props}
    >
      {children}
    </section>
  );
}
