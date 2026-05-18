import { Radio, RadioGroup } from "@headlessui/react";
import {
  Link,
  type LinkProps,
  useMatchRoute,
  useNavigate,
  useRouterState,
} from "@tanstack/react-router";
import {
  Children,
  Fragment,
  isValidElement,
  type ComponentType,
  type MouseEvent,
  type ReactElement,
  type ReactNode,
} from "react";
import { cx } from "@/base/utils/cx";
import { Indicator } from "@/ui/components/Indicator";
import { useTheme } from "@/ui/theme/useTheme";

type NavigationIcon = ComponentType<{ className?: string }>;

type NavigationProps = {
  "aria-label": string;
  children: ReactNode;
  className?: string;
};

type NavigationItemTo = NonNullable<LinkProps["to"]>;

type NavigationBadgeVariant = "red" | "yellow";

export type NavigationBadge = {
  label?: string;
  variant: NavigationBadgeVariant;
};

type NavigationItemProps = Omit<
  LinkProps,
  "activeProps" | "children" | "className" | "onClick" | "to"
> & {
  badge?: NavigationBadge;
  disabled?: boolean;
  icon: NavigationIcon;
  label: string;
  onClick?: (event: MouseEvent<HTMLAnchorElement>) => void;
  to: NavigationItemTo;
};

function isNavigationItemElement(
  child: ReactNode,
): child is ReactElement<{ to: NavigationItemTo }> {
  return (
    isValidElement(child) &&
    typeof child.props === "object" &&
    child.props !== null &&
    "to" in child.props
  );
}

function NavigationItem({
  badge,
  disabled = false,
  icon: Icon,
  label,
  onClick,
  ...linkProps
}: NavigationItemProps) {
  const { globalClasses, getVariant } = useTheme();
  const v = {
    neutral: getVariant("neutral"),
    violet: getVariant("violet"),
  };

  function handleClick(event: MouseEvent<HTMLAnchorElement>) {
    if (disabled) {
      event.preventDefault();
      return;
    }
    onClick?.(event);
  }

  return (
    <Radio as={Fragment} disabled={disabled} value={linkProps.to}>
      {({ checked }) => (
        <Link
          aria-current={checked ? "page" : undefined}
          aria-disabled={disabled}
          data-selected={checked || undefined}
          className={cx(
            "flex w-full items-center gap-2 rounded p-3 text-left no-underline border-0",
            v.neutral.outline.base.readableColor,
            v.neutral.outline.focus,
            "focus-visible:ring-2",
            globalClasses.focusRing,
            checked && [
              "font-bold",
              v.violet.solid.selected.background,
              v.violet.solid.border,
              v.violet.solid.selected.readableColor,
            ],
            !disabled &&
              !checked && [
                v.violet.solid.hover.background,
                v.violet.solid.hover.readableColor,
              ],
            disabled && [
              "cursor-not-allowed",
              globalClasses.disabledOpacityStatic,
            ],
          )}
          onClick={handleClick}
          {...linkProps}
        >
          <Icon className="h-4 w-4" />
          <span className="min-w-0 flex-1">{label}</span>
          {badge ? (
            <Indicator label={badge.label} variant={badge.variant} />
          ) : null}
        </Link>
      )}
    </Radio>
  );
}

export function Navigation({
  children,
  className,
  "aria-label": ariaLabel,
}: NavigationProps) {
  const navigate = useNavigate();
  const matchRoute = useMatchRoute();
  const pathname = useRouterState({
    select: (state) => state.location.pathname,
  }) as NavigationItemTo;
  const activeValue = Children.toArray(children).find((child) => {
    if (!isNavigationItemElement(child)) {
      return false;
    }
    return Boolean(matchRoute({ fuzzy: true, to: child.props.to }));
  }) as ReactElement<{ to: NavigationItemTo }> | undefined;

  return (
    <nav aria-label={ariaLabel}>
      <RadioGroup
        as="div"
        className={cx("grid gap-2", className)}
        onChange={(to: NavigationItemTo) => navigate({ to })}
        value={activeValue?.props.to ?? pathname}
      >
        {children}
      </RadioGroup>
    </nav>
  );
}

Navigation.Item = NavigationItem;
