import { useState } from "react";
import { Button } from "@/ui/components/Button";
import { ConfirmDialog } from "@/ui/components/ConfirmDialog";
import { type Variant } from "@/ui/theme/variants";

type ConfirmButtonProps = {
  heading: string;
  message: string;
  onConfirm: () => void;
  confirmLabel: string;
  cancelLabel: string;
  dialogVariant: Variant;
} & React.ComponentProps<typeof Button>;

export function ConfirmButton({
  heading,
  message,
  onConfirm,
  confirmLabel,
  cancelLabel,
  dialogVariant,
  onClick,
  ...buttonProps
}: ConfirmButtonProps) {
  const [open, setOpen] = useState(false);

  return (
    <>
      <Button
        onClick={(event) => {
          onClick?.(event);
          if (!event.defaultPrevented) {
            setOpen(true);
          }
        }}
        {...buttonProps}
      />
      <ConfirmDialog
        cancelLabel={cancelLabel}
        confirmLabel={confirmLabel}
        heading={heading}
        message={message}
        onClose={setOpen}
        onConfirm={() => {
          onConfirm();
          setOpen(false);
        }}
        open={open}
        variant={dialogVariant}
      />
    </>
  );
}
