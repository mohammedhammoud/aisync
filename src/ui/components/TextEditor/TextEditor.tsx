import { cx } from "@/base/utils/cx";
import { Textarea } from "@/ui/components/Textarea";
import { FormField } from "@/ui/components/FormField";

type TextEditorProps = {
  value: string;
  onChange: (value: string) => void;
  label: string;
  disabled?: boolean;
  autoResize?: boolean;
  fill?: boolean;
};

export function TextEditor({
  value,
  onChange,
  label,
  disabled,
  autoResize = false,
  fill = false,
}: TextEditorProps) {
  return (
    <FormField
      label={label}
      className={fill ? "h-full min-h-0" : undefined}
      disabled={disabled}
    >
      <Textarea
        autoResize={autoResize}
        className={cx(fill ? "h-full" : "h-56", "font-mono")}
        disabled={disabled}
        onChange={(event) => onChange(event.target.value)}
        spellCheck={false}
        value={value}
      />
    </FormField>
  );
}
