import { Alert } from "@/ui/components/Alert";
import { Text } from "@/ui/components/Text";
import { cx } from "@/base/utils/cx";
import { useTheme } from "@/ui/theme/useTheme";

type AppHeaderProps = {
  lockMessage: string | null;
  title: string;
};

export function AppHeader({ lockMessage, title }: AppHeaderProps) {
  const { getVariant } = useTheme();

  const v = getVariant("neutral");

  return (
    <header className="mb-4 flex h-11 items-start justify-between">
      <Text
        as="h1"
        className={cx(
          "m-0 text-xl font-semibold tracking-normal",
          v.solid.base.readableColor,
        )}
      >
        {title}
      </Text>
      {lockMessage ? (
        <div className="max-w-sm overflow-hidden">
          <Alert aria-live="polite" role="status" variant="yellow">
            {lockMessage}
          </Alert>
        </div>
      ) : null}
    </header>
  );
}
