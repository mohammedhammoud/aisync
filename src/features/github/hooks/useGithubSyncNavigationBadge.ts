import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import { useGithubSyncStore } from "@/features/github/store/githubSyncStore";
import type { NavigationBadge } from "@/ui/components/Navigation";

export function useGithubSyncNavigationBadge() {
  const { t } = useTranslation();
  const status = useGithubSyncStore((state) => state.status);
  const lastResult = useGithubSyncStore((state) => state.lastResult);

  const githubSyncBadge = useMemo<NavigationBadge | undefined>(() => {
    if (lastResult?.status === "conflict") {
      return { label: t("github.badgeConflict"), variant: "red" };
    }

    if (status?.hasLocalChanges) {
      return { label: t("github.badgePending"), variant: "yellow" };
    }

    return undefined;
  }, [lastResult, status, t]);

  return { githubSyncBadge };
}
