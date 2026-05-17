import type { Meta, StoryObj } from "@storybook/react-vite";
import { GitHubConflictList } from "./GitHubConflictList";

const meta = {
  title: "Features/GitHub/GitHubConflictList",
  component: GitHubConflictList,
  args: {
    conflicts: [
      {
        path: "instructions.md",
        message: "Local and remote changes conflict",
        localContent: "local",
        remoteContent: "remote",
      },
      {
        path: "skills/audit/SKILL.md",
        message: "Local and remote changes conflict",
        localContent: null,
        remoteContent: "remote skill",
      },
    ],
    onOpenConflict: () => undefined,
  },
} satisfies Meta<typeof GitHubConflictList>;

export default meta;
type Story = StoryObj<typeof meta>;

export const WithConflicts: Story = {};

export const Empty: Story = {
  args: {
    conflicts: [],
  },
};
