import type { LinkStatus } from "@/base/tauri/bindings";

export function skillIdsWithLinkStatus(statuses: LinkStatus[]): Set<string> {
  return new Set(
    statuses
      .filter((status) => status.kind === "skill")
      .map((status) => status.skillId),
  );
}

export function linkStatusesForSkill(
  statuses: LinkStatus[],
  skillId: string,
): LinkStatus[] {
  return statuses.filter(
    (status) => status.kind === "skill" && status.skillId === skillId,
  );
}

export function instructionLinkStatuses(statuses: LinkStatus[]): LinkStatus[] {
  return statuses.filter((status) => status.kind === "instructions");
}
