import { Checkbox, Field, Label } from "@headlessui/react";
import { Check } from "lucide-react";
import type { ReactNode } from "react";
import { cx } from "@/base/utils/cx";
import { useTheme } from "@/ui/theme/useTheme";

type CheckboxFieldProps = {
  checked?: boolean;
  children: ReactNode;
  defaultChecked?: boolean;
  disabled?: boolean;
  name?: string;
  onChange?: (checked: boolean) => void;
  value?: string;
};

export function CheckboxField({
  checked,
  children,
  defaultChecked,
  disabled,
  name,
  onChange,
  value,
}: CheckboxFieldProps) {
  const { globalClasses, getVariant } = useTheme();
  const v = {
    field: getVariant("neutral"),
    checked: getVariant("violet"),
  };

  return (
    <Field
      className={cx(
        "flex flex-row items-center gap-2 text-xs font-medium",
        v.field.outline.text,
        disabled
          ? cx("cursor-not-allowed", globalClasses.disabledOpacityStatic)
          : "cursor-pointer",
      )}
      disabled={disabled}
    >
      <Checkbox
        checked={checked}
        className={cx(
          "group flex size-4 items-center justify-center rounded",
          v.field.outline.border,
          v.field.outline.base.background,
          v.field.outline.base.readableColor,
          v.field.outline.focus,
          "focus-visible:ring-2",
          globalClasses.focusRing,
          v.checked.solid.checked,
          "disabled:cursor-not-allowed",
          globalClasses.disabledOpacity,
        )}
        defaultChecked={defaultChecked}
        name={name}
        onChange={onChange}
        value={value}
      >
        <Check className="hidden size-3 group-data-[checked]:block" />
      </Checkbox>
      <Label>{children}</Label>
    </Field>
  );
}
