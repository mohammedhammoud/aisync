import { format } from "date-fns";
import { useTranslation } from "react-i18next";
import { Alert } from "@/ui/components/Alert";
import { Indicator } from "@/ui/components/Indicator";
import { Text } from "@/ui/components/Text";
import type { Variant } from "@/ui/theme/variants";

export type GitHubStatusCardVariant = "loading" | "success" | "failed";

type GitHubStatusCardProps = {
  detail: string | null;
  lastSyncedAt: string | null;
  text: string;
  variant: GitHubStatusCardVariant;
};

const indicatorVariantByCardVariant = {
  failed: "red",
  loading: "yellow",
  success: "green",
} satisfies Record<GitHubStatusCardVariant, Variant>;

export function GitHubStatusCard({
  detail,
  lastSyncedAt,
  text,
  variant,
}: GitHubStatusCardProps) {
  const { t } = useTranslation();

  return (
    <div className="grid gap-2">
      <Text as="div" className="text-xs font-medium">
        {t("github.status")}
      </Text>
      <Alert mode="outline" className="flex items-start gap-2 p-4">
        <Indicator
          className="mt-1.5 shrink-0"
          size="md"
          variant={indicatorVariantByCardVariant[variant]}
        />
        <div className="grid gap-1">
          <Text as="div" className="text-sm font-semibold">
            {text}
          </Text>
          {detail ? (
            <Text as="p" className="text-xs" tone="muted">
              {detail}
            </Text>
          ) : null}
        </div>
      </Alert>
      {lastSyncedAt ? (
        <Text as="p" className="text-xs" tone="muted">
          {t("github.lastSynced", {
            value: format(new Date(lastSyncedAt), "yyyy-MM-dd HH:mm"),
          })}
        </Text>
      ) : null}
    </div>
  );
}
