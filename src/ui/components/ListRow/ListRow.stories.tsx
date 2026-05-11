import type { Meta, StoryObj } from "@storybook/react-vite";
import { ListRow } from "./ListRow";

const meta = {
  title: "UI/ListRow",
  component: ListRow,
  args: {
    description: "Review changed code",
    selected: true,
    title: "Audit",
  },
} satisfies Meta<typeof ListRow>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Disabled: Story = {
  args: {
    disabled: true,
  },
};
