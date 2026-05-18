import { Radio, RadioGroup } from "@headlessui/react";
import { Link, useNavigate, useRouterState } from "@tanstack/react-router";
import { Plus } from "lucide-react";
import { Fragment } from "react";
import { useTranslation } from "react-i18next";
import type { TargetConfig } from "@/base/tauri/bindings";
import { Button } from "@/ui/components/Button";
import { ListRow } from "@/ui/components/ListRow";
import { Alert } from "@/ui/components/Alert";

type ConfigListProps = {
  configs: TargetConfig[];
  disabled?: boolean;
  onCreate: () => void;
};

export function ConfigList({
  configs,
  disabled = false,
  onCreate,
}: ConfigListProps) {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const pathname = useRouterState({
    select: (state) => state.location.pathname,
  });

  return (
    <div className="flex flex-col gap-4">
      <Button
        disabled={disabled}
        full
        icon={<Plus size={15} />}
        onClick={onCreate}
        variant="violet"
      >
        {t("configs.new")}
      </Button>
      <div className="flex flex-col gap-2">
        {configs.length ? (
          <RadioGroup
            aria-label={t("configs.listLabel")}
            as="div"
            className="flex flex-col gap-2"
            disabled={disabled}
            onChange={(to: string) => navigate({ to })}
            value={pathname}
          >
            {configs.map((item) => {
              const to = `/configs/${item.id}`;

              return (
                <Radio as={Fragment} key={item.id} value={to}>
                  <Link
                    aria-disabled={disabled}
                    className="no-underline"
                    onClick={(event) => {
                      if (disabled) {
                        event.preventDefault();
                      }
                    }}
                    params={{ configId: item.id }}
                    to="/configs/$configId"
                  >
                    {({ isActive }) => (
                      <ListRow
                        aria-current={isActive ? "true" : undefined}
                        aria-label={t("configs.selectConfig", {
                          name: item.name,
                        })}
                        description={item.skillsPath}
                        disabled={disabled}
                        selected={isActive}
                        title={item.name}
                      />
                    )}
                  </Link>
                </Radio>
              );
            })}
          </RadioGroup>
        ) : (
          <Alert variant="yellow">{t("configs.noConfigs")}</Alert>
        )}
      </div>
    </div>
  );
}
