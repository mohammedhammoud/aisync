import type { Meta, StoryObj } from "@storybook/react-vite";
import { variants } from "@/ui/theme/variants";
import { Alert } from "./Alert";

const meta = {
  title: "UI/Alert",
  component: Alert,
  args: {
    children: "Status message",
    variant: "neutral",
  },
} satisfies Meta<typeof Alert>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Variants: Story = {
  render: () => (
    <div className="grid gap-2">
      {variants.map((variant) => (
        <Alert key={variant} variant={variant}>
          {variant} status message
        </Alert>
      ))}
    </div>
  ),
};
