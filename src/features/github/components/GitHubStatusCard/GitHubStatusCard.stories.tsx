import type { Meta, StoryObj } from "@storybook/react-vite";
import { GitHubStatusCard } from "./GitHubStatusCard";

const meta = {
  title: "Features/GitHub/GitHubStatusCard",
  component: GitHubStatusCard,
  args: {
    detail: null,
    variant: "success",
    lastSyncedAt: "2026-05-16T20:41:06Z",
    text: "Connected to octocat/aisync-config",
  },
} satisfies Meta<typeof GitHubStatusCard>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Connected: Story = {};

export const Syncing: Story = {
  args: {
    detail: "Uploading local changes.",
    variant: "loading",
    lastSyncedAt: "2026-05-16T20:41:06Z",
    text: "Syncing changes...",
  },
};

export const Connecting: Story = {
  args: {
    detail: "Once you approve, we’ll finish the setup automatically.",
    variant: "loading",
    lastSyncedAt: null,
    text: "Connecting to GitHub...",
  },
};

export const SyncFailed: Story = {
  args: {
    detail: null,
    variant: "failed",
    lastSyncedAt: "2026-05-16T20:41:06Z",
    text: "Failed to sync changes",
  },
};

export const ReconnectRequired: Story = {
  args: {
    detail: "Reconnect GitHub to push changes.",
    variant: "failed",
    lastSyncedAt: "2026-05-16T20:41:06Z",
    text: "Repo configured: octocat/aisync-config",
  },
};

export const NotConnected: Story = {
  args: {
    detail: null,
    variant: "failed",
    lastSyncedAt: null,
    text: "Not connected",
  },
};
