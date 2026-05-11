import { Save } from "lucide-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { variants } from "@/ui/theme/variants";
import { Button } from "./Button";

const meta = {
  title: "UI/Button",
  component: Button,
  args: {
    children: "Button",
    icon: <Save size={15} />,
  },
} satisfies Meta<typeof Button>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Variants: Story = {
  render: () => (
    <div className="flex flex-wrap gap-2">
      {variants.map((variant) => (
        <Button key={variant} icon={<Save size={15} />} variant={variant}>
          {variant}
        </Button>
      ))}
    </div>
  ),
};

export const Disabled: Story = {
  args: {
    children: "Save",
    icon: <Save size={15} />,
    disabled: true,
  },
};
