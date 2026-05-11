import type { Meta, StoryObj } from "@storybook/react-vite";
import { Button } from "@/ui/components/Button";
import { Dialog } from "./Dialog";

const meta = {
  title: "UI/Dialog",
  component: Dialog,
  args: {
    open: true,
    onClose: () => undefined,
    actions: <Button variant="violet">Done</Button>,
    description: "Use this dialog for confirm/prompt flows.",
    heading: "Dialog heading",
    children: <p className="m-0 text-xs">Dialog content</p>,
  },
} satisfies Meta<typeof Dialog>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const WithoutCloseButton: Story = {
  args: {
    showCloseButton: false,
  },
};
