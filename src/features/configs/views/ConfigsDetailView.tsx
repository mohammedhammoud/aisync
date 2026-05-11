import { createLazyRoute } from "@tanstack/react-router";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { useAppLockState } from "@/base/root/appLock";
import { createToast } from "@/base/store/toastStore";
import type { TargetConfig } from "@/base/tauri/bindings";
import { ConfigForm } from "@/features/configs/components/ConfigForm";
import { Spinner } from "@/ui/components/Spinner";
import { Alert } from "@/ui/components/Alert";
import { useDeleteConfig } from "@/features/configs/hooks/useDeleteConfig";
import { useGetConfig } from "@/features/configs/hooks/useGetConfig";
import { useUpdateConfig } from "@/features/configs/hooks/useUpdateConfig";

function ConfigsDetailView() {
  const { configId } = Route.useParams();
  const { t } = useTranslation();

  const { config, isLoading } = useGetConfig(configId);
  const { deleteConfig, isDeleting } = useDeleteConfig();
  const { isUpdating, updateConfig } = useUpdateConfig();

  const [currentConfig, setCurrentConfig] = useState<TargetConfig | null>(null);
  const [baselineConfig, setBaselineConfig] = useState<TargetConfig | null>(
    null,
  );

  const isDirty =
    JSON.stringify(currentConfig) !== JSON.stringify(baselineConfig);

  useEffect(() => {
    if (config) {
      setCurrentConfig({ ...config });
      setBaselineConfig({ ...config });
    }
  }, [config]);

  useAppLockState({
    isLocked: isDirty,
    message: isDirty ? t("dirty.unsaved") : null,
  });

  async function saveConfig() {
    if (!currentConfig) {
      return;
    }

    const didSave = await updateConfig(configId, currentConfig);

    if (didSave) {
      setBaselineConfig({ ...currentConfig });
    }
  }

  function discardConfig() {
    if (!baselineConfig) {
      return;
    }

    setCurrentConfig({ ...baselineConfig });
    createToast({ message: t("common.discarded") });
  }

  if (isLoading) {
    return (
      <div className="flex h-full items-center justify-center">
        <Spinner size="lg" />
      </div>
    );
  }

  if (!currentConfig) {
    return (
      <div className="p-4">
        <Alert variant="red">{t("configs.notFound")}</Alert>
      </div>
    );
  }

  return (
    <ConfigForm
      config={currentConfig}
      isDeleting={isDeleting}
      isDirty={isDirty}
      isUpdating={isUpdating}
      onChange={setCurrentConfig}
      onDelete={deleteConfig}
      onDiscard={discardConfig}
      onSave={saveConfig}
    />
  );
}

export const Route = createLazyRoute("/configs/$configId")({
  component: ConfigsDetailView,
});
