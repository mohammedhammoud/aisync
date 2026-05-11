import type { PropsWithChildren } from "react";
import { cx } from "@/base/utils/cx";
import { useTheme } from "@/ui/theme/useTheme";

type ShellSurfaceProps = PropsWithChildren<{
  className?: string;
}>;

export function ShellSurface({ children, className }: ShellSurfaceProps) {
  const { globalClasses } = useTheme();

  return (
    <div
      className={cx(
        globalClasses.shellBackground,
        globalClasses.shellText,
        className,
      )}
    >
      {children}
    </div>
  );
}
