import React from "react";
import ReactDOM from "react-dom/client";
import App from "@/App";
import { initConfigsStore } from "@/base/store/configsStore";
import { initGlobalsStore } from "@/base/store/globalsStore";
import { initSkillsStore } from "@/base/store/skillsStore";
import { ThemeProvider } from "@/ui/theme/ThemeProvider";
import "../tailwind.css";
import "@/i18n";

void initConfigsStore();
void initSkillsStore();
void initGlobalsStore();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ThemeProvider>
      <App />
    </ThemeProvider>
  </React.StrictMode>,
);
