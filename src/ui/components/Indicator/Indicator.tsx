import { cx } from "@/base/utils/cx";
import { useTheme } from "@/ui/theme/useTheme";
import type { Variant } from "@/ui/theme/variants";

type IndicatorSize = "sm" | "md";

type IndicatorProps = {
  className?: string;
  label?: string;
  size?: IndicatorSize;
  variant?: Variant;
};

const sizeClassNames: Record<IndicatorSize, string> = {
  sm: "h-2 w-2",
  md: "h-2.5 w-2.5",
};

export function Indicator({
  className,
  label,
  size = "sm",
  variant = "neutral",
}: IndicatorProps) {
  const { getVariant } = useTheme();
  const v = getVariant(variant).solid;

  return (
    <span className={cx("inline-flex items-center", className)} title={label}>
      {label ? <span className="sr-only">{label}</span> : null}
      <span
        aria-hidden="true"
        className={cx("rounded-full", sizeClassNames[size], v.accent)}
      />
    </span>
  );
}
