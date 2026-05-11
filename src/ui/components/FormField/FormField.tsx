import { Field, FieldProps, Label } from "@headlessui/react";
import type { PropsWithChildren } from "react";
import { cx } from "@/base/utils/cx";
import { Text } from "@/ui/components/Text";
import { useTheme } from "@/ui/theme/useTheme";

type FormFieldProps = PropsWithChildren<{
  label: string;
  className?: string;
}> &
  Omit<FieldProps, "className">;

export function FormField({
  children,
  label,
  className,
  ...rest
}: FormFieldProps) {
  const { getVariant } = useTheme();
  const v = getVariant("neutral");

  return (
    <Field
      className={cx("flex flex-col items-start gap-2", className)}
      {...rest}
    >
      <Label
        as={Text}
        className={cx(
          "w-fit max-w-full shrink-0 self-start text-xs font-medium",
          v.outline.base.readableColor,
        )}
      >
        {label}
      </Label>
      {children}
    </Field>
  );
}
