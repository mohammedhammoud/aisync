import type { Meta, StoryObj } from "@storybook/react-vite";
import { TextInput } from "@/ui/components/TextInput";
import { FormField } from "./FormField";

const meta = {
  title: "UI/FormField",
  component: FormField,
  args: {
    children: <TextInput defaultValue="Audit" />,
    label: "Name",
  },
} satisfies Meta<typeof FormField>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};
