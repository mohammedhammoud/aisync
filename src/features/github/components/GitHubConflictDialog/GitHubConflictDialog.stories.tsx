import type { Meta, StoryObj } from "@storybook/react-vite";
import { GitHubConflictDialog } from "./GitHubConflictDialog";

const meta = {
  title: "Features/GitHub/GitHubConflictDialog",
  component: GitHubConflictDialog,
  args: {
    conflict: {
      path: "instructions.md",
      message: "Conflict",
      localContent: "Local instructions\n\nKeep this.",
      remoteContent: "GitHub instructions\n\nOr keep this.",
    },
    onClose: () => undefined,
    onResolve: () => undefined,
  },
} satisfies Meta<typeof GitHubConflictDialog>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const DeletedRemote: Story = {
  args: {
    conflict: {
      path: "skills/audit/SKILL.md",
      message: "Conflict",
      localContent: "Local skill",
      remoteContent: null,
    },
  },
};

export const DeletedLocal: Story = {
  args: {
    conflict: {
      path: "skills/debug/SKILL.md",
      message: "Conflict",
      localContent: null,
      remoteContent: "Remote skill",
    },
  },
};
