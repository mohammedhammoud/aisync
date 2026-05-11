import type { Meta, StoryObj } from "@storybook/react-vite";
import { ConfirmDialog } from "./ConfirmDialog";

const meta = {
  title: "UI/ConfirmDialog",
  component: ConfirmDialog,
  args: {
    open: true,
    onClose: () => undefined,
    onConfirm: () => undefined,
    heading: "Delete item",
    message: "This action cannot be undone.",
    cancelLabel: "Cancel",
    confirmLabel: "Delete",
    variant: "red",
  },
} satisfies Meta<typeof ConfirmDialog>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Neutral: Story = {
  args: {
    confirmLabel: "Confirm",
    heading: "Confirm action",
    message: "Proceed with this action?",
    variant: "violet",
  },
};
