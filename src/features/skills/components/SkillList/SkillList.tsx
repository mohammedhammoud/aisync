import { Radio, RadioGroup } from "@headlessui/react";
import { Link, useNavigate, useRouterState } from "@tanstack/react-router";
import { Plus } from "lucide-react";
import { Fragment } from "react";
import { useTranslation } from "react-i18next";
import type { SkillMetadata } from "@/base/tauri/bindings";
import { Button } from "@/ui/components/Button";
import { ListRow } from "@/ui/components/ListRow";
import { Alert } from "@/ui/components/Alert";

type SkillListProps = {
  disabled?: boolean;
  onCreate: () => void;
  skills: SkillMetadata[];
};

export function SkillList({
  disabled = false,
  onCreate,
  skills,
}: SkillListProps) {
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
        {t("skills.new")}
      </Button>
      <div className="flex flex-col gap-2">
        {skills.length ? (
          <RadioGroup
            aria-label={t("skills.title")}
            as="div"
            className="flex flex-col gap-2"
            disabled={disabled}
            onChange={(to: string) => navigate({ to })}
            value={pathname}
          >
            {skills.map((skill) => {
              const to = `/skills/${skill.id}`;

              return (
                <Radio as={Fragment} key={skill.id} value={to}>
                  <Link
                    aria-disabled={disabled}
                    className="no-underline"
                    onClick={(event) => {
                      if (disabled) {
                        event.preventDefault();
                      }
                    }}
                    params={{ skillId: skill.id }}
                    to="/skills/$skillId"
                  >
                    {({ isActive }) => (
                      <ListRow
                        aria-current={isActive ? "true" : undefined}
                        description={skill.description || skill.id}
                        disabled={disabled}
                        selected={isActive}
                        title={skill.name}
                      />
                    )}
                  </Link>
                </Radio>
              );
            })}
          </RadioGroup>
        ) : (
          <Alert variant="yellow">{t("skills.noSkills")}</Alert>
        )}
      </div>
    </div>
  );
}
