import {
  Description,
  Dialog as HeadlessDialog,
  DialogPanel,
  DialogTitle,
} from "@headlessui/react";
import { X } from "lucide-react";
import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { cx } from "@/base/utils/cx";
import { Button } from "@/ui/components/Button";
import { Text } from "@/ui/components/Text";
import { useTheme } from "@/ui/theme/useTheme";
import type { Variant, VariantMode } from "@/ui/theme/variants";

type DialogSize = "sm" | "md" | "lg" | "xl";

type DialogProps = {
  open: boolean;
  onClose: (open: boolean) => void;
  heading?: ReactNode;
  description?: ReactNode;
  actions?: ReactNode;
  children?: ReactNode;
  showCloseButton?: boolean;
  size?: DialogSize;
  variant?: Variant;
  mode?: VariantMode;
};

const sizeClassName: Record<DialogSize, string> = {
  sm: "max-w-md",
  md: "max-w-xl",
  lg: "max-w-3xl",
  xl: "max-w-5xl",
};

export function Dialog({
  open,
  onClose,
  heading,
  description,
  actions,
  children,
  showCloseButton = true,
  size = "md",
  variant = "neutral",
  mode = "outline",
}: DialogProps) {
  const { t } = useTranslation();
  const { getVariant } = useTheme();

  const v = {
    panel: getVariant(variant)[mode],
    text: getVariant("neutral"),
  };

  return (
    <HeadlessDialog className="relative z-50" onClose={onClose} open={open}>
      <div aria-hidden="true" className="fixed inset-0 bg-black/60" />
      <div className="fixed inset-0 grid p-4 [place-items:center]">
        <DialogPanel
          className={cx(
            v.panel.base.background,
            v.panel.border,
            v.panel.base.readableColor,
            v.text.outline.border,
            "relative flex max-h-[calc(100vh-2rem)] w-full max-w-[calc(100vw-2rem)] flex-col overflow-hidden rounded p-4 shadow-[0_1.5rem_5rem_rgba(0,0,0,0.55)]",
            sizeClassName[size],
          )}
        >
          {(heading || showCloseButton) && (
            <div className="mb-2 flex items-start justify-between gap-3">
              {heading ? (
                <DialogTitle
                  className={cx(
                    "m-0 text-sm font-semibold",
                    v.text.solid.base.readableColor,
                  )}
                >
                  {heading}
                </DialogTitle>
              ) : (
                <span />
              )}
              {showCloseButton && (
                <Button
                  aria-label={t("common.close")}
                  className="h-7 px-2"
                  onClick={() => onClose(false)}
                >
                  <X size={14} />
                </Button>
              )}
            </div>
          )}
          {description && (
            <Description
              as={Text}
              className={cx("mb-3 text-xs", v.text.outline.text)}
            >
              {description}
            </Description>
          )}
          <div className="min-h-0 min-w-0 overflow-auto">{children}</div>
          {actions && (
            <div className="mt-4 flex shrink-0 justify-end gap-2">
              {actions}
            </div>
          )}
        </DialogPanel>
      </div>
    </HeadlessDialog>
  );
}
