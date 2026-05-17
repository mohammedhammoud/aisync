import { format } from "date-fns";
import { Copy, ExternalLink } from "lucide-react";
import { useTranslation } from "react-i18next";
import { cx } from "@/base/utils/cx";
import { Alert } from "@/ui/components/Alert";
import { Button } from "@/ui/components/Button";
import { Text } from "@/ui/components/Text";
import { Toolbar } from "@/ui/components/Toolbar";
import { useTheme } from "@/ui/theme/useTheme";

type GitHubLoginCodePanelProps = {
  expiresInSeconds: number;
  openDelaySeconds: number;
  userCode: string;
  onCopyCode: () => void;
  onOpenLogin: () => void;
};

export function GitHubLoginCodePanel({
  expiresInSeconds,
  openDelaySeconds,
  userCode,
  onCopyCode,
  onOpenLogin,
}: GitHubLoginCodePanelProps) {
  const { t } = useTranslation();
  const { getVariant } = useTheme();
  const v = {
    code: getVariant("neutral").solid,
  };
  const expiresIn = format(expiresInSeconds * 1000, "m:ss");

  return (
    <Alert mode="outline">
      <div className="grid gap-4 sm:grid-cols-[auto_1fr_auto] sm:items-center">
        <Text
          as="span"
          className={cx(
            "w-fit whitespace-nowrap rounded px-2 py-1.5 font-mono text-xl font-semibold leading-none tracking-wide",
            v.code.base.background,
            v.code.base.readableColor,
          )}
        >
          {userCode}
        </Text>
        {expiresInSeconds > 0 ? (
          <Text as="span" className="text-xs font-medium" tone="muted">
            {t("github.expiresIn", { time: expiresIn })}
          </Text>
        ) : null}
        <Toolbar className="shrink-0 justify-end self-center">
          <Button
            icon={<Copy size={15} />}
            mode="outline"
            size="sm"
            onClick={onCopyCode}
          >
            {t("github.copy")}
          </Button>
          <Button
            icon={<ExternalLink size={15} />}
            mode="outline"
            size="sm"
            onClick={onOpenLogin}
          >
            {openDelaySeconds > 0
              ? t("github.openInSeconds", { seconds: openDelaySeconds })
              : t("github.openGithub")}
          </Button>
        </Toolbar>
      </div>
    </Alert>
  );
}
