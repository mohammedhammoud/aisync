import { Save, Trash2 } from "lucide-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { Button } from "@/ui/components/Button";
import { Toolbar } from "./Toolbar";

const meta = {
  title: "UI/Toolbar",
  component: Toolbar,
  args: {
    children: (
      <>
        <Button icon={<Save size={15} />} variant="violet">
          Save
        </Button>
        <Button icon={<Trash2 size={15} />} variant="red">
          Delete
        </Button>
      </>
    ),
  },
} satisfies Meta<typeof Toolbar>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};
