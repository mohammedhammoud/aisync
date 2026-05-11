import { Select } from "@headlessui/react";
import type { SelectHTMLAttributes } from "react";
import { cx } from "@/base/utils/cx";
import { useTheme } from "@/ui/theme/useTheme";

export function SelectInput({
  className,
  ...props
}: SelectHTMLAttributes<HTMLSelectElement>) {
  const { globalClasses, getVariant } = useTheme();
  const v = getVariant("neutral");

  return (
    <Select
      className={cx(
        "h-8 w-full rounded px-2.5 text-xs",
        v.outline.border,
        v.outline.base.background,
        v.outline.base.readableColor,
        v.outline.focus,
        "disabled:cursor-not-allowed",
        globalClasses.disabledOpacity,
        className,
      )}
      {...props}
    />
  );
}
