import type { Meta, StoryObj } from "@storybook/react-vite";
import { variants } from "@/ui/theme/variants";
import { Indicator } from "./Indicator";

const meta = {
  title: "UI/Indicator",
  component: Indicator,
  args: {
    label: "Pending changes",
    variant: "yellow",
    size: "sm",
  },
} satisfies Meta<typeof Indicator>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Variants: Story = {
  render: () => (
    <div className="grid gap-3">
      {variants.map((variant) => (
        <div key={variant} className="flex items-center gap-2 text-sm">
          <Indicator label={`${variant} notification`} variant={variant} />
          <span>{variant}</span>
        </div>
      ))}
    </div>
  ),
};

const _sizes = ["sm", "md"] as const;

export const Sizes: Story = {
  render: () => (
    <div className="grid gap-3">
      {variants.map((variant) => (
        <div key={variant} className="flex items-center gap-4 text-sm">
          {_sizes.map((size) => (
            <div key={size} className="flex items-center gap-2">
              <Indicator
                label={`${size} notification`}
                size={size}
                variant={variant}
              />
              <span>
                {variant} ({size})
              </span>
            </div>
          ))}
        </div>
      ))}
    </div>
  ),
};
