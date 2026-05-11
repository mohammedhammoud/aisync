import type { Meta, StoryObj } from "@storybook/react-vite";
import { InstructionsEditor } from "./InstructionsEditor";

const meta = {
  title: "Features/Instructions/InstructionsEditor",
  component: InstructionsEditor,
  decorators: [
    (Story) => (
      <div className="h-[70vh] min-h-[28rem]">
        <Story />
      </div>
    ),
  ],
  args: {
    content: "# AGENTS.md\n\n- Keep changes small.\n- Run focused checks.",
    isDirty: false,
    onChange: () => undefined,
    onDiscard: () => undefined,
    onSave: () => undefined,
  },
} satisfies Meta<typeof InstructionsEditor>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Dirty: Story = {
  args: {
    isDirty: true,
  },
};
