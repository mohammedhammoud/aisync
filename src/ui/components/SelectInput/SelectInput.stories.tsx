import type { Meta, StoryObj } from "@storybook/react-vite";
import { SelectInput } from "./SelectInput";

const meta = {
  title: "UI/SelectInput",
  component: SelectInput,
  args: {
    children: (
      <>
        <option value="symlink">symlink</option>
        <option value="copy">copy</option>
      </>
    ),
  },
} satisfies Meta<typeof SelectInput>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Disabled: Story = {
  args: {
    disabled: true,
  },
};
