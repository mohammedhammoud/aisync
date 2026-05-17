import type { Meta, StoryObj } from "@storybook/react-vite";
import { variants } from "@/ui/theme/variants";
import { Badge } from "./Badge";

const meta = {
  title: "UI/Badge",
  component: Badge,
  args: {
    children: "2",
    variant: "violet",
  },
} satisfies Meta<typeof Badge>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Variants: Story = {
  render: () => (
    <div className="flex flex-wrap gap-2">
      {variants.map((variant) => (
        <Badge key={variant} variant={variant}>
          {variant}
        </Badge>
      ))}
    </div>
  ),
};

export const Sizes: Story = {
  render: () => (
    <div className="flex items-center gap-2">
      <Badge size="sm" variant="violet">
        1
      </Badge>
      <Badge size="md" variant="violet">
        12
      </Badge>
    </div>
  ),
};
