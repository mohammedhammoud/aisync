import { cx } from "@/base/utils/cx";
import type { JSX, PropsWithChildren } from "react";

type ToolbarProps = PropsWithChildren & JSX.IntrinsicElements["div"];

export function Toolbar({ children, className, ...props }: ToolbarProps) {
  return (
    <div
      className={cx("flex flex-wrap items-center gap-2", className)}
      {...props}
    >
      {children}
    </div>
  );
}
