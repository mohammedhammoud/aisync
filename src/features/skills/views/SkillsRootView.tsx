import { createLazyRoute, Outlet, useNavigate } from "@tanstack/react-router";
import { useAppLockContext } from "@/base/root/appLock";
import { useSkillsStore } from "@/base/store/skillsStore";
import { SkillList } from "@/features/skills/components/SkillList";
import { SplitPane } from "@/ui/components/SplitPane";
import { Spinner } from "@/ui/components/Spinner";

function SkillsRootView() {
  const navigate = useNavigate();
  const { isLocked } = useAppLockContext();
  const skills = useSkillsStore((state) => state.skills);

  if (!skills) {
    return (
      <div className="flex h-full items-center justify-center">
        <Spinner size="lg" />
      </div>
    );
  }

  return (
    <SplitPane
      list={
        <SkillList
          disabled={isLocked}
          onCreate={() => navigate({ to: "/skills/new" })}
          skills={skills}
        />
      }
      detail={<Outlet />}
    />
  );
}

export const Route = createLazyRoute("/skills")({
  component: SkillsRootView,
});
