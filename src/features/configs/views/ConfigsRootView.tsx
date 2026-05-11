import { createLazyRoute, Outlet, useNavigate } from "@tanstack/react-router";
import { useAppLockContext } from "@/base/root/appLock";
import { useConfigsStore } from "@/base/store/configsStore";
import { ConfigList } from "@/features/configs/components/ConfigList";
import { SplitPane } from "@/ui/components/SplitPane";
import { Spinner } from "@/ui/components/Spinner";

function ConfigsRootView() {
  const navigate = useNavigate();
  const { isLocked } = useAppLockContext();
  const configs = useConfigsStore((state) => state.configs);

  if (!configs) {
    return (
      <div className="flex h-full items-center justify-center">
        <Spinner size="lg" />
      </div>
    );
  }

  return (
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
  );
}

export const Route = createLazyRoute("/configs")({
  component: ConfigsRootView,
});
