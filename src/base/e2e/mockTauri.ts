import type { Page } from "@playwright/test";

const mockConfig = {
  appName: "AISync",
  setupPath: "/Users/example/.aisync",
  skills: [
    {
      id: "audit",
      name: "Audit",
      description: "Review code for risks",
      enabled: true,
      tags: ["review"],
    },
    {
      id: "refactor",
      name: "Refactor",
      description: "Improve structure",
      enabled: true,
      tags: ["code"],
    },
  ],
  configs: [
    {
      id: "local",
      name: "Local project",
      skillsPath: "/Users/example/project/skills",
      instructionsPath: "/Users/example/project/AGENTS.md",
      enabled: true,
    },
    {
      id: "work",
      name: "Work project",
      skillsPath: "/Users/example/work/skills",
      instructionsPath: "/Users/example/work/AGENTS.md",
      enabled: true,
    },
  ],
};

const mockDefaults = {
  setupPath: "/Users/example/.aisync",
  newTargetConfig: {
    id: "new-project",
    name: "New project",
    skillsPath: "/Users/example/project/skills",
    instructionsPath: "/Users/example/project/AGENTS.md",
    enabled: true,
  },
};

export async function installTauriMock(page: Page) {
  await page.addInitScript(
    ({ config, defaults }) => {
      const skillContent: Record<string, string> = {
        audit: "# Audit\n\nReview code for risks.",
        refactor: "# Refactor\n\nImprove structure.",
      };

      (
        window as typeof window & { __TAURI_INTERNALS__: unknown }
      ).__TAURI_INTERNALS__ = {
        invoke: async (command: string, args?: Record<string, unknown>) => {
          switch (command) {
            case "get_globals":
              return { appName: config.appName, setupPath: config.setupPath };
            case "get_defaults":
              return defaults;
            case "get_skills":
              return config.skills;
            case "get_configs":
              return config.configs;
            case "get_skill": {
              const skillId = String(args?.skillId ?? "audit");
              const metadata =
                config.skills.find((skill) => skill.id === skillId) ??
                config.skills[0];

              return {
                body: skillContent[skillId] ?? "# Skill\n\n",
                frontmatterLines: [],
                metadata,
              };
            }
            case "get_config": {
              const configId = String(args?.configId ?? "local");
              return (
                config.configs.find((item) => item.id === configId) ??
                config.configs[0]
              );
            }
            case "read_instructions":
              return "# Project instructions\n\nUse safe, minimal changes.";
            case "create_config":
            case "update_config":
            case "delete_config":
            case "create_skill":
            case "update_skill":
            case "delete_skill":
            case "write_instructions":
              return null;
            default:
              throw new Error(`Unhandled invoke command: ${command}`);
          }
        },
      };
    },
    { config: mockConfig, defaults: mockDefaults },
  );
}
