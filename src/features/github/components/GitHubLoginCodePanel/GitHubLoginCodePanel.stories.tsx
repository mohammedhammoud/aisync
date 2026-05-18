import type { Meta, StoryObj } from "@storybook/react-vite";
import { GitHubLoginCodePanel } from "./GitHubLoginCodePanel";

const meta = {
  title: "Features/GitHub/GitHubLoginCodePanel",
  component: GitHubLoginCodePanel,
  args: {
    expiresInSeconds: 900,
    openDelaySeconds: 3,
    userCode: "ABCD-1234",
    onCopyCode: () => undefined,
    onOpenLogin: () => undefined,
  },
} satisfies Meta<typeof GitHubLoginCodePanel>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Countdown: Story = {};

export const Opened: Story = {
  args: {
    openDelaySeconds: 0,
  },
};
