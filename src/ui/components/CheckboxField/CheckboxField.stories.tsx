import type { Meta, StoryObj } from "@storybook/react-vite";
import { CheckboxField } from "./CheckboxField";

const meta = {
  title: "UI/CheckboxField",
  component: CheckboxField,
  args: {
    children: "Checkbox",
  },
} satisfies Meta<typeof CheckboxField>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Checked: Story = {
  args: { defaultChecked: true },
};

export const Disabled: Story = {
  args: {
    disabled: true,
  },
};
