import type { Meta, StoryObj } from "@storybook/react-vite";
import { AppHeader } from "./AppHeader";

const meta = {
  title: "Base/Root/AppHeader",
  component: AppHeader,
  args: {
    lockMessage: null,
    title: "Skills",
  },
} satisfies Meta<typeof AppHeader>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Dirty: Story = {
  args: {
    lockMessage: "You have unsaved changes.",
  },
};
