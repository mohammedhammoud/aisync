import type { Meta, StoryObj } from "@storybook/react-vite";
import { Pane } from "./Pane";

const meta = {
  title: "UI/Pane",
  component: Pane,
  args: {
    children: <p>Pane content</p>,
  },
} satisfies Meta<typeof Pane>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};
