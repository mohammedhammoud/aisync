import { Trash2 } from "lucide-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { ConfirmButton } from "./ConfirmButton";

const meta = {
  title: "UI/ConfirmButton",
  component: ConfirmButton,
  args: {
    children: "Delete",
    heading: "Delete item",
    message: "This action cannot be undone.",
    confirmLabel: "Delete",
    cancelLabel: "Cancel",
    dialogVariant: "red",
    variant: "red",
    icon: <Trash2 size={15} />,
    onConfirm: () => undefined,
  },
} satisfies Meta<typeof ConfirmButton>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};
