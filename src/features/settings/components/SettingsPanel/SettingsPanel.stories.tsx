import type { Meta, StoryObj } from "@storybook/react-vite";
import { SettingsPanel } from "./SettingsPanel";

const meta = {
  title: "Features/Settings/SettingsPanel",
  component: SettingsPanel,
  args: {
    colorScheme: "system",
    language: "en",
    localRoot: "~/.aisync",
    onChangeColorScheme: () => undefined,
    onChangeLanguage: () => undefined,
  },
} satisfies Meta<typeof SettingsPanel>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const WithoutLocalRoot: Story = {
  args: {
    localRoot: "",
  },
};

export const Swedish: Story = {
  args: { language: "sv" },
};

export const DarkTheme: Story = {
  args: { colorScheme: "dark" },
};
