import type { Meta, StoryObj } from "@storybook/react-vite";
import { ListRow } from "@/ui/components/ListRow";
import { SplitPane } from "./SplitPane";

const meta = {
  title: "UI/SplitPane",
  component: SplitPane,
  args: {
    detail: (
      <div className="flex h-full items-center justify-center text-sm text-white/50">
        Select an item to view details
      </div>
    ),
    list: (
      <div className="flex flex-col gap-2">
        {Array.from({ length: 10 }).map((_, i) => (
          <ListRow
            selected={i === 2}
            key={i}
            title={`Item ${i + 1}`}
            description="This is a description"
          />
        ))}
      </div>
    ),
  },
} satisfies Meta<typeof SplitPane>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};
