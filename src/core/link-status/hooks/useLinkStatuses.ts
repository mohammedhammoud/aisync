import { listen } from "@tauri-apps/api/event";
import { useCallback, useEffect, useMemo, useState } from "react";
import { createToast } from "@/base/store/toastStore";
import { commands, type LinkStatus } from "@/base/tauri/bindings";
import {
  CONFIGS_CHANGED_EVENT,
  SKILLS_CHANGED_EVENT,
} from "@/base/tauri/consts";
import { translateTauriError } from "@/base/tauri/utils/translateTauriError";

export function useLinkStatuses(initialStatuses?: LinkStatus[]) {
  const [statuses, setStatuses] = useState<LinkStatus[]>(initialStatuses ?? []);

  const loadStatuses = useCallback(async () => {
    try {
      setStatuses(initialStatuses ?? (await commands.getLinkStatus()));
    } catch (error) {
      createToast({ message: translateTauriError(error), variant: "red" });
    }
  }, [initialStatuses]);

  useEffect(() => {
    if (initialStatuses) {
      setStatuses(initialStatuses);
      return;
    }

    const cleanups = [
      listen(CONFIGS_CHANGED_EVENT, loadStatuses),
      listen(SKILLS_CHANGED_EVENT, loadStatuses),
    ];
    void loadStatuses();
    return () => {
      void Promise.all(cleanups).then((items) => {
        for (const cleanup of items) {
          cleanup();
        }
      });
    };
  }, [initialStatuses, loadStatuses]);

  const fixLinkStatus = useCallback(
    async (status: LinkStatus) => {
      await commands.forceLinkTarget(status);
      await loadStatuses();
    },
    [loadStatuses],
  );

  return useMemo(
    () => ({ fixLinkStatus, loadStatuses, statuses }),
    [fixLinkStatus, loadStatuses, statuses],
  );
}
