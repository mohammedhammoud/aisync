import { createLazyRoute, Outlet, useNavigate } from "@tanstack/react-router";
import { useAppLockContext } from "@/base/root/appLock";
import { useConfigsStore } from "@/base/store/configsStore";
import { ConfigList } from "@/features/configs/components/ConfigList";
import { LinkStatusNotice } from "@/core/link-status/components/LinkStatusNotice";
import { useLinkStatuses } from "@/core/link-status/hooks/useLinkStatuses";
import { instructionLinkStatuses } from "@/core/link-status/utils/linkStatus";
import { SplitPane } from "@/ui/components/SplitPane";
import { Spinner } from "@/ui/components/Spinner";

function ConfigsRootView() {
  const navigate = useNavigate();
  const { isLocked } = useAppLockContext();
  const configs = useConfigsStore((state) => state.configs);
  const { fixLinkStatus, statuses } = useLinkStatuses();
  const instructionStatuses = instructionLinkStatuses(statuses);

  if (!configs) {
    return (
      <div className="flex h-full items-center justify-center">
        <Spinner size="lg" />
      </div>
    );
  }

  return (
    <div className="flex h-full flex-col gap-3">
      <LinkStatusNotice
        disabled={isLocked}
        onFixLinkStatus={fixLinkStatus}
        statuses={instructionStatuses}
      />
      <SplitPane
        list={
          <ConfigList
            configs={configs}
            disabled={isLocked}
            onCreate={() => navigate({ to: "/configs/new" })}
          />
        }
        detail={<Outlet />}
      />
    </div>
  );
}

export const Route = createLazyRoute("/configs")({
  component: ConfigsRootView,
});
