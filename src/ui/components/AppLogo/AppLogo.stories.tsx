import type { Meta, StoryObj } from "@storybook/react-vite";
import { AppLogo } from "./AppLogo";

const meta = {
  title: "UI/AppLogo",
  component: AppLogo,
} satisfies Meta<typeof AppLogo>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};
