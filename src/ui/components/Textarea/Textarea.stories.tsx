import { useState } from "react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { Textarea } from "./Textarea";

const meta = {
  title: "UI/Textarea",
  component: Textarea,
  args: {
    autoResize: true,
    placeholder: "Write something...",
    value: "First line",
  },
} satisfies Meta<typeof Textarea>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: (args) => {
    const [value, setValue] = useState(String(args.value ?? ""));
    return (
      <Textarea
        {...args}
        value={value}
        onChange={(event) => setValue(event.target.value)}
      />
    );
  },
};

export const FixedHeight: Story = {
  args: {
    autoResize: false,
  },
  render: (args) => {
    const [value, setValue] = useState(String(args.value ?? ""));
    return (
      <Textarea
        {...args}
        value={value}
        onChange={(event) => setValue(event.target.value)}
      />
    );
  },
};

export const Disabled: Story = {
  args: {
    disabled: true,
  },
  render: (args) => {
    const [value, setValue] = useState(String(args.value ?? ""));
    return (
      <Textarea
        {...args}
        value={value}
        onChange={(event) => setValue(event.target.value)}
      />
    );
  },
};
