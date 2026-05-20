import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import {
  instructionLinkStatuses,
  skillIdsWithLinkStatus,
} from "@/core/link-status/utils/linkStatus";
import type { NavigationBadge } from "@/ui/components/Navigation";
import { useLinkStatuses } from "./useLinkStatuses";

export function useLinkStatusNavigationBadges() {
  const { t } = useTranslation();
  const { statuses } = useLinkStatuses();

  return useMemo(() => {
    const skillCount = skillIdsWithLinkStatus(statuses).size;
    const configCount = instructionLinkStatuses(statuses).length;

    const skillsBadge: NavigationBadge | undefined = skillCount
      ? {
          label: t("sync.skillsNeedAttention", { count: skillCount }),
          variant: "red",
        }
      : undefined;
    const configsBadge: NavigationBadge | undefined = configCount
      ? {
          label: t("sync.configsNeedAttention", { count: configCount }),
          variant: "red",
        }
      : undefined;

    return { configsBadge, skillsBadge };
  }, [statuses, t]);
}
