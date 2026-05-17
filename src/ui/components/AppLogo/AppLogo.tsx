import type { ImgHTMLAttributes } from "react";
import { cx } from "@/base/utils/cx";

export function AppLogo({
  alt = "AISync",
  className,
  ...props
}: ImgHTMLAttributes<HTMLImageElement>) {
  return (
    <img
      alt={alt}
      className={cx(
        "h-16 w-16 rounded-2xl shadow-sm ring-1 ring-black/10",
        className,
      )}
      src="/logo.png"
      {...props}
    />
  );
}
