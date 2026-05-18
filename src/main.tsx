import React from "react";
import ReactDOM from "react-dom/client";
import App from "@/App";
import { initConfigsStore } from "@/base/store/configsStore";
import { initI18n } from "@/base/i18n/client";
import { initGlobalsStore } from "@/base/store/globalsStore";
import { initSkillsStore } from "@/base/store/skillsStore";
import { initGithubSyncStore } from "@/features/github/store/githubSyncStore";
import { ThemeProvider } from "@/ui/theme/ThemeProvider";
import "../tailwind.css";

void initI18n();
void initConfigsStore();
void initSkillsStore();
void initGlobalsStore();
void initGithubSyncStore();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ThemeProvider>
      <App />
    </ThemeProvider>
  </React.StrictMode>,
);
