import { createLazyRoute, useNavigate } from "@tanstack/react-router";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { useAppLockContext, useAppLockState } from "@/base/root/appLock";
import { useConfigsStore } from "@/base/store/configsStore";
import { createToast } from "@/base/store/toastStore";
import type { Defaults, TargetConfig } from "@/base/tauri/bindings";
import { ConfigForm } from "@/features/configs/components/ConfigForm";
import { useCreateConfig } from "@/features/configs/hooks/useCreateConfig";
import { Spinner } from "@/ui/components/Spinner";

function ConfigsCreateContent({ defaults }: { defaults: Defaults }) {
  const navigate = useNavigate();
  const { t } = useTranslation();
  const { releaseLock } = useAppLockContext();
  const { createConfig, isCreating } = useCreateConfig();

  const [targetConfig, setTargetConfig] = useState<TargetConfig>(() => ({
    ...defaults.newTargetConfig,
  }));

  useAppLockState({ isLocked: true, message: t("dirty.unsaved") });

  async function saveConfig() {
    const didCreate = await createConfig(targetConfig);

    if (!didCreate) {
      return;
    }

    releaseLock();
    navigate({
      params: { configId: targetConfig.id },
      replace: true,
      to: "/configs/$configId",
    });
  }

  function discardConfig() {
    releaseLock();
    createToast({ message: t("common.discarded") });
    navigate({ ignoreBlocker: true, replace: true, to: "/configs" });
  }

  return (
    <ConfigForm
      config={targetConfig}
      isCreating={isCreating}
      isDirty={true}
      onChange={setTargetConfig}
      onDiscard={discardConfig}
      onSave={saveConfig}
    />
  );
}

function ConfigsCreateView() {
  const defaults = useConfigsStore((state) => state.defaults);

  if (!defaults) {
    return (
      <div className="flex h-full items-center justify-center">
        <Spinner size="lg" />
      </div>
    );
  }

  return <ConfigsCreateContent defaults={defaults} />;
}

export const Route = createLazyRoute("/configs/new")({
  component: ConfigsCreateView,
});
