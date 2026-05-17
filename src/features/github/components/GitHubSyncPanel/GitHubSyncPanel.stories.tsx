import type { Meta, StoryObj } from "@storybook/react-vite";
import { GitHubSyncPanel } from "./GitHubSyncPanel";

const disconnectedStatus = {
  connected: false,
  repoOwner: null,
  repoName: null,
  defaultBranch: null,
  lastSyncedCommitSha: null,
  lastSyncedAt: null,
  hasToken: false,
  hasLocalChanges: false,
};

const connectedStatus = {
  connected: true,
  repoOwner: "octocat",
  repoName: "aisync-config",
  defaultBranch: "main",
  lastSyncedCommitSha: "abc123",
  lastSyncedAt: "2026-05-16T20:41:06Z",
  hasToken: true,
  hasLocalChanges: false,
};

const meta = {
  title: "Features/GitHub/GitHubSyncPanel",
  component: GitHubSyncPanel,
  args: {
    loginCodeExpiresInSeconds: 0,
    loginCodeOpenDelaySeconds: 0,
    loginCodeUserCode: null,
    conflicts: [],
    syncActivity: "idle",
    syncStatus: disconnectedStatus,
    onConnect: () => undefined,
    onCreateRepo: () => undefined,
    onDisconnect: () => undefined,
    onLoginCodeCopy: () => undefined,
    onLoginCodeOpen: () => undefined,
    onOpenConflict: () => undefined,
    onSync: () => undefined,
  },
} satisfies Meta<typeof GitHubSyncPanel>;

export default meta;
type Story = StoryObj<typeof meta>;

export const NotConnected: Story = {};

export const LoginCode: Story = {
  args: {
    loginCodeExpiresInSeconds: 900,
    loginCodeOpenDelaySeconds: 3,
    loginCodeUserCode: "ABCD-1234",
    syncActivity: "connecting",
  },
};

export const Authorized: Story = {
  args: {
    syncStatus: {
      ...disconnectedStatus,
      hasToken: true,
    },
  },
};

export const Connected: Story = {
  args: {
    syncStatus: connectedStatus,
  },
};

export const ReconnectRequired: Story = {
  args: {
    syncStatus: {
      ...connectedStatus,
      connected: false,
      hasToken: false,
    },
  },
};

export const Conflict: Story = {
  args: {
    conflicts: [
      {
        path: "instructions.md",
        message: "Conflict",
        localContent: "local",
        remoteContent: "remote",
      },
    ],
    syncStatus: connectedStatus,
  },
};
