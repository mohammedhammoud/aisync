import { Textarea as HeadlessTextarea } from "@headlessui/react";
import { Maximize2, Minimize2 } from "lucide-react";
import { forwardRef, useCallback, useEffect, useRef, useState } from "react";
import type { ChangeEvent, KeyboardEvent, TextareaHTMLAttributes } from "react";
import { useTranslation } from "react-i18next";
import { cx } from "@/base/utils/cx";
import { Button } from "@/ui/components/Button";
import { useTheme } from "@/ui/theme/useTheme";

type TextareaProps = {
  autoResize: boolean;
} & TextareaHTMLAttributes<HTMLTextAreaElement>;

export const Textarea = forwardRef<HTMLTextAreaElement, TextareaProps>(
  function Textarea(
    { autoResize, className, onChange, value, id, ...props },
    forwardedRef,
  ) {
    const [expanded, setExpanded] = useState(false);
    const innerRef = useRef<HTMLTextAreaElement | null>(null);
    const { t } = useTranslation();
    const { globalClasses, getVariant } = useTheme();
    const v = getVariant("neutral");

    const focusTextarea = () => {
      requestAnimationFrame(() => innerRef.current?.focus());
    };

    const setRefs = (element: HTMLTextAreaElement | null) => {
      innerRef.current = element;

      if (!forwardedRef) {
        return;
      }

      if (typeof forwardedRef === "function") {
        forwardedRef(element);
        return;
      }

      forwardedRef.current = element;
    };

    const resize = useCallback(() => {
      if (!autoResize || !innerRef.current) {
        return;
      }

      innerRef.current.style.height = "auto";
      innerRef.current.style.height = `${innerRef.current.scrollHeight}px`;
    }, [autoResize]);

    useEffect(() => {
      resize();
    }, [resize, value, expanded]);

    useEffect(() => {
      if (!expanded) {
        return;
      }

      const handleKeyDown = (event: globalThis.KeyboardEvent) => {
        if (event.key === "Escape") {
          setExpanded(false);
        }
      };

      document.addEventListener("keydown", handleKeyDown);
      return () => document.removeEventListener("keydown", handleKeyDown);
    }, [expanded]);

    const toggleExpanded = () => {
      setExpanded((current) => !current);
      focusTextarea();
    };

    const handleTextareaChange = (event: ChangeEvent<HTMLTextAreaElement>) => {
      onChange?.(event);
      resize();
    };

    const handleTextareaKeyDown = (
      event: KeyboardEvent<HTMLTextAreaElement>,
    ) => {
      props.onKeyDown?.(event);

      if (
        !event.defaultPrevented &&
        event.key === "Enter" &&
        (event.metaKey || event.ctrlKey)
      ) {
        event.preventDefault();
        toggleExpanded();
      }
    };

    const textareaClassName = cx(
      "w-full resize-none rounded px-2.5 py-2 pr-10 text-xs disabled:cursor-not-allowed",
      globalClasses.disabledOpacity,
      v.outline.border,
      v.outline.base.background,
      v.outline.base.readableColor,
      v.outline.focus,
      "focus-visible:ring-2",
      globalClasses.focusRing,
      expanded ? "h-full min-h-full" : className,
    );

    return (
      <>
        {expanded && (
          <div aria-hidden="true" className="fixed inset-0 z-40 bg-black/60" />
        )}
        <div
          className={expanded ? "fixed inset-4 z-50" : "relative h-full w-full"}
        >
          <HeadlessTextarea
            className={textareaClassName}
            id={id}
            onChange={handleTextareaChange}
            onKeyDown={handleTextareaKeyDown}
            ref={setRefs}
            value={value}
            {...props}
          />
          <Button
            aria-label={expanded ? t("common.collapse") : t("common.expand")}
            className="absolute right-2 top-2"
            iconOnly
            mode="outline"
            onClick={toggleExpanded}
            size="sm"
            variant="neutral"
          >
            {expanded ? <Minimize2 size={14} /> : <Maximize2 size={14} />}
          </Button>
        </div>
      </>
    );
  },
);
