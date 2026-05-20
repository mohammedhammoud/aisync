import type { HTMLAttributes, ReactNode } from "react";
import { cx } from "@/base/utils/cx";
import { useTheme } from "@/ui/theme/useTheme";

type ListRowProps = HTMLAttributes<HTMLDivElement> & {
  disabled?: boolean;
  selected?: boolean;
  title: ReactNode;
  description?: ReactNode;
  indicator?: ReactNode;
};

export function ListRow({
  className = "",
  description,
  disabled = false,
  indicator,
  selected = false,
  title,
  ...props
}: ListRowProps) {
  const { globalClasses, getVariant } = useTheme();
  const v = {
    neutral: getVariant("neutral"),
    violet: getVariant("violet"),
  };

  return (
    <div
      aria-disabled={disabled || undefined}
      data-selected={selected || undefined}
      className={cx(
        "grid w-full gap-1 rounded bg-transparent p-3 text-left",
        v.neutral.outline.base.readableColor,
        v.neutral.outline.focus,
        "focus-visible:ring-2",
        globalClasses.focusRing,
        disabled
          ? ["cursor-not-allowed", globalClasses.disabledOpacityStatic]
          : "cursor-pointer",
        !disabled &&
          !selected && [
            v.violet.solid.hover.background,
            v.violet.solid.hover.readableColor,
          ],
        selected && [
          v.violet.solid.selected.background,
          v.violet.solid.selected.readableColor,
        ],
        className,
      )}
      {...props}
    >
      <div className="flex items-center gap-2">
        {indicator ? <span className="shrink-0">{indicator}</span> : null}
        <strong className="min-w-0 truncate text-inherit">{title}</strong>
      </div>
      {description ? (
        <span className="text-inherit text-xs">{description}</span>
      ) : null}
    </div>
  );
}
