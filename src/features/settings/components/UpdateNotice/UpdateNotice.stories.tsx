import type { Meta, StoryObj } from "@storybook/react-vite";
import { UpdateNotice } from "./UpdateNotice";

const meta = {
  title: "Features/Settings/UpdateNotice",
  component: UpdateNotice,
  args: {
    version: "aisync-v0.2.3",
    onDownload: () => undefined,
  },
} satisfies Meta<typeof UpdateNotice>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};
