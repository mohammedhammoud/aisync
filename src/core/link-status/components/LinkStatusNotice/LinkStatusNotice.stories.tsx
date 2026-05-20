import type { Meta, StoryObj } from "@storybook/react-vite";
import type { LinkStatus } from "@/base/tauri/bindings";
import { LinkStatusNotice } from "./LinkStatusNotice";

const statuses: LinkStatus[] = [
  {
    kind: "instructions",
    configName: "Pi",
    state: "blocked",
    targetPath: "/Users/example/.pi/agent/APPEND_SYSTEM.md",
  },
  {
    kind: "skill",
    configName: "Codex",
    skillId: "audit",
    state: "missing",
    targetPath: "/Users/example/.codex/skills/audit",
  },
];

const meta = {
  title: "Core/LinkStatus/LinkStatusNotice",
  component: LinkStatusNotice,
  args: {
    onFixLinkStatus: async () => undefined,
    statuses,
  },
} satisfies Meta<typeof LinkStatusNotice>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Disabled: Story = {
  args: {
    disabled: true,
  },
};

export const Empty: Story = {
  args: {
    statuses: [],
  },
};
