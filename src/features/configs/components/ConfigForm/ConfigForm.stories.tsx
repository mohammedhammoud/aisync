import type { Meta, StoryObj } from "@storybook/react-vite";
import type { TargetConfig } from "@/base/tauri/bindings";
import { ConfigForm } from "./ConfigForm";

const mockConfig: TargetConfig = {
  id: "mock-target",
  name: "Mock Target",
  skillsPath: "/mock/skills",
  instructionsPath: "/mock/instructions.md",
  enabled: true,
};

const meta = {
  title: "Features/Configs/ConfigForm",
  component: ConfigForm,
  args: {
    config: mockConfig,
    isDirty: false,
    onChange: () => undefined,
    onDelete: () => undefined,
    onDiscard: () => undefined,
    onSave: () => undefined,
  },
} satisfies Meta<typeof ConfigForm>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Dirty: Story = {
  args: {
    isDirty: true,
  },
};
