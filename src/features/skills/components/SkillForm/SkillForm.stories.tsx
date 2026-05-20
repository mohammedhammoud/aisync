import type { Meta, StoryObj } from "@storybook/react-vite";
import { SkillForm } from "./SkillForm";

const skill = {
  id: "audit",
  name: "Audit",
  description: "Review changed code",
  enabled: true,
  tags: [],
};

const meta = {
  title: "Features/Skills/SkillForm",
  component: SkillForm,
  args: {
    content: "# Audit\n\nReview staged and unstaged changes.",
    isDirty: false,
    metadata: skill,
    onChangeContent: () => undefined,
    onChangeMetadata: () => undefined,
    onDelete: () => undefined,
    onDiscard: () => undefined,
    onSave: () => undefined,
  },
} satisfies Meta<typeof SkillForm>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Dirty: Story = {
  args: {
    isDirty: true,
  },
};

export const LinkStatus: Story = {
  args: {
    linkStatuses: [
      {
        kind: "skill",
        configName: "Pi",
        skillId: "audit",
        state: "blocked",
        targetPath: "/Users/example/.pi/agent/skills/audit",
      },
    ],
    onFixLinkStatus: async () => undefined,
  },
};
