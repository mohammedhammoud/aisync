import { AlertCircle } from "lucide-react";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import type { LinkStatus } from "@/base/tauri/bindings";
import { Alert } from "@/ui/components/Alert";
import { Button } from "@/ui/components/Button";
import { ConfirmButton } from "@/ui/components/ConfirmButton";
import { Dialog } from "@/ui/components/Dialog";
import { Badge } from "@/ui/components/Badge";
import { Indicator } from "@/ui/components/Indicator";
import { Text } from "@/ui/components/Text";

type LinkStatusNoticeProps = {
  disabled?: boolean;
  onFixLinkStatus?: (status: LinkStatus) => Promise<void>;
  statuses: LinkStatus[];
};

type LinkStatusRowProps = {
  onFix?: (status: LinkStatus) => Promise<void>;
  status: LinkStatus;
};

function LinkStatusRow({ onFix, status }: LinkStatusRowProps) {
  const { t } = useTranslation();

  return (
    <div className="flex items-center justify-between gap-3 rounded-md border border-neutral-200 bg-neutral-50/60 px-4 py-3 dark:border-neutral-800 dark:bg-neutral-950/60">
      <div className="flex min-w-0 flex-1 items-start gap-2">
        <Indicator label={t("sync.blocked")} variant="red" />
        <div className="grid min-w-0 gap-2">
          <Text as="div" className="truncate text-sm font-semibold">
            {status.targetPath}
          </Text>
          <div className="flex min-w-0 items-center gap-1">
            <Badge variant="red">
              {status.state === "blocked"
                ? t("sync.targetBlocked")
                : t("sync.missing")}
            </Badge>
            <Badge>{status.configName}</Badge>
          </div>
        </div>
      </div>
      {onFix && status.state === "blocked" ? (
        <ConfirmButton
          cancelLabel={t("common.cancel")}
          confirmLabel={t("sync.replace")}
          dialogVariant="red"
          heading={t("sync.replaceHeading")}
          message={t("sync.replaceMessage", { path: status.targetPath })}
          onConfirm={() => void onFix(status)}
          variant="red"
        >
          {t("sync.fix")}
        </ConfirmButton>
      ) : null}
      {onFix && status.state === "missing" ? (
        <Button onClick={() => void onFix(status)} variant="red">
          {t("sync.fix")}
        </Button>
      ) : null}
    </div>
  );
}

export function LinkStatusNotice({
  disabled = false,
  onFixLinkStatus,
  statuses,
}: LinkStatusNoticeProps) {
  const { t } = useTranslation();
  const [open, setOpen] = useState(false);

  if (!statuses.length) {
    return null;
  }

  const singleStatus = statuses[0];

  if (statuses.length === 1 && singleStatus && !open) {
    return (
      <LinkStatusRow
        onFix={disabled ? undefined : onFixLinkStatus}
        status={singleStatus}
      />
    );
  }

  return (
    <>
      <Dialog
        description={t("sync.blockedDescription")}
        heading={t("sync.blocked")}
        onClose={setOpen}
        open={open}
      >
        <div className="space-y-2">
          {statuses.map((status) => (
            <LinkStatusRow
              key={status.targetPath}
              onFix={
                onFixLinkStatus
                  ? async (item) => {
                      await onFixLinkStatus(item);
                    }
                  : undefined
              }
              status={status}
            />
          ))}
        </div>
      </Dialog>
      <Alert mode="outline" variant="red">
        <div className="flex items-center justify-between gap-3">
          <div className="flex min-w-0 items-start gap-2">
            <AlertCircle className="mt-0.5 shrink-0" size={15} />
            <div className="grid gap-1">
              <Text className="text-xs font-semibold">
                {t("sync.linkStatusTitle", { count: statuses.length })}
              </Text>
              <Text className="text-xs" tone="muted">
                {t("sync.linkStatusHint")}
              </Text>
            </div>
          </div>
          <Button
            disabled={disabled}
            onClick={() => setOpen(true)}
            variant="red"
          >
            {t("sync.review")}
          </Button>
        </div>
      </Alert>
    </>
  );
}
