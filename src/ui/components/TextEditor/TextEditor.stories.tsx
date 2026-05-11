import type { Meta, StoryObj } from "@storybook/react-vite";
import { TextEditor } from "./TextEditor";

const meta = {
  title: "UI/TextEditor",
  component: TextEditor,
  args: {
    label: "Instructions",
    onChange: () => undefined,
    value: "# AGENTS.md\n\n- Keep changes small.",
  },
} satisfies Meta<typeof TextEditor>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Disabled: Story = {
  args: {
    disabled: true,
    label: "Instructions",
  },
};
