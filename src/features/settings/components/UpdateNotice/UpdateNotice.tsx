import { Download } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Alert } from "@/ui/components/Alert";
import { Button } from "@/ui/components/Button";
import { Text } from "@/ui/components/Text";

type UpdateNoticeProps = {
  version: string;
  onDownload: () => void;
};

export function UpdateNotice({ version, onDownload }: UpdateNoticeProps) {
  const { t } = useTranslation();

  return (
    <Alert mode="outline" variant="violet">
      <div className="flex items-center justify-between gap-3">
        <div className="grid gap-1">
          <Text className="text-xs font-semibold">
            {t("updates.available", { version })}
          </Text>
          <Text className="text-xs" tone="muted">
            {t("updates.downloadHint")}
          </Text>
        </div>
        <Button
          icon={<Download size={15} />}
          onClick={onDownload}
          variant="violet"
        >
          {t("updates.download")}
        </Button>
      </div>
    </Alert>
  );
}
