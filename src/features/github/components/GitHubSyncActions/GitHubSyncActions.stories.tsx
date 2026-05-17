import type { Meta, StoryObj } from "@storybook/react-vite";
import { GitHubSyncActions } from "./GitHubSyncActions";

const baseStatus = {
  connected: false,
  repoOwner: null,
  repoName: null,
  defaultBranch: null,
  lastSyncedCommitSha: null,
  lastSyncedAt: null,
  hasToken: false,
  hasLocalChanges: false,
};

const meta = {
  title: "Features/GitHub/GitHubSyncActions",
  component: GitHubSyncActions,
  args: {
    activity: "idle",
    syncStatus: baseStatus,
    onConnect: () => undefined,
    onCreateRepo: () => undefined,
    onDisconnect: () => undefined,
    onSync: () => undefined,
  },
} satisfies Meta<typeof GitHubSyncActions>;

export default meta;
type Story = StoryObj<typeof meta>;

export const NotConnected: Story = {};

export const Authorized: Story = {
  args: {
    syncStatus: { ...baseStatus, hasToken: true },
  },
};

export const Connected: Story = {
  args: {
    syncStatus: {
      ...baseStatus,
      connected: true,
      repoOwner: "octocat",
      repoName: "aisync-config",
      defaultBranch: "main",
      hasToken: true,
    },
  },
};

export const Syncing: Story = {
  args: {
    activity: "syncing",
    syncStatus: { ...baseStatus, hasToken: true },
  },
};
