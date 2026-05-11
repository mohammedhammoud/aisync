import { Button } from "@/ui/components/Button";
import { Dialog } from "@/ui/components/Dialog";

import type { Variant } from "@/ui/theme/variants";

type ConfirmDialogProps = {
  open: boolean;
  onClose: (open: boolean) => void;
  onConfirm: () => void;
  heading: string;
  message: string;
  confirmLabel: string;
  cancelLabel: string;
  variant: Variant;
};

export function ConfirmDialog({
  open,
  onClose,
  onConfirm,
  heading,
  message,
  confirmLabel,
  cancelLabel,
  variant,
}: ConfirmDialogProps) {
  return (
    <Dialog
      actions={
        <>
          <Button onClick={() => onClose(false)}>{cancelLabel}</Button>
          <Button onClick={onConfirm} variant={variant}>
            {confirmLabel}
          </Button>
        </>
      }
      description={message}
      heading={heading}
      onClose={onClose}
      open={open}
      size="sm"
    />
  );
}
