import { cx } from "@/base/utils/cx";
import { useTheme } from "@/ui/theme/useTheme";

type SpinnerSize = "sm" | "md" | "lg";

type SpinnerProps = {
  size?: SpinnerSize;
  className?: string;
};

const sizeClasses: Record<SpinnerSize, string> = {
  sm: "h-4 w-4 border-2",
  md: "h-6 w-6 border-2",
  lg: "h-8 w-8 border-[3px]",
};

export function Spinner({ size = "md", className }: SpinnerProps) {
  const { globalClasses } = useTheme();

  return (
    <div
      className={cx(
        "animate-spin rounded-full border-transparent",
        "border-t-current",
        globalClasses.textMuted,
        sizeClasses[size],
        className,
      )}
      role="status"
      aria-label="Loading"
    />
  );
}
