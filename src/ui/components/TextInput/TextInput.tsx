import { Input } from "@headlessui/react";
import { forwardRef } from "react";
import type { InputHTMLAttributes } from "react";
import { cx } from "@/base/utils/cx";
import { useTheme } from "@/ui/theme/useTheme";

export const TextInput = forwardRef<
  HTMLInputElement,
  InputHTMLAttributes<HTMLInputElement>
>(function TextInput({ className, readOnly, ...props }, ref) {
  const { globalClasses, getVariant } = useTheme();
  const v = getVariant("neutral");
  const fieldVariant = readOnly ? v.solid : v.outline;

  return (
    <Input
      className={cx(
        "h-8 w-full rounded px-2.5 text-xs",
        fieldVariant.border,
        fieldVariant.base.background,
        fieldVariant.base.readableColor,
        fieldVariant.focus,
        "focus-visible:ring-2",
        globalClasses.focusRing,
        readOnly && "cursor-not-allowed",
        "disabled:cursor-not-allowed",
        globalClasses.disabledOpacity,
        className,
      )}
      readOnly={readOnly}
      ref={ref}
      {...props}
    />
  );
});
