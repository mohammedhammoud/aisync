import js from "@eslint/js";
import globals from "globals";
import tseslint from "typescript-eslint";
import react from "eslint-plugin-react";
import * as reactHooks from "eslint-plugin-react-hooks";
import reactRefresh from "eslint-plugin-react-refresh";
import jsxA11y from "eslint-plugin-jsx-a11y";
import unusedImports from "eslint-plugin-unused-imports";
import storybook from "eslint-plugin-storybook";
import stylistic from "@stylistic/eslint-plugin";
import prettier from "eslint-plugin-prettier/recommended";
import { localRules } from "./eslint-local-rules/index.js";

export default tseslint.config(
  {
    ignores: [
      "dist",
      "storybook-static",
      "src-tauri/target",
      "node_modules",
      "src/base/tauri/bindings.ts",
    ],
  },
  {
    extends: [js.configs.recommended, ...tseslint.configs.recommended],
    files: ["**/*.{ts,tsx}"],
    languageOptions: {
      globals: globals.browser,
    },
    plugins: {
      "unused-imports": unusedImports,
      "react-refresh": reactRefresh,
      "@stylistic": stylistic,
      local: { rules: localRules },
    },
    rules: {
      "@typescript-eslint/no-unused-vars": "off",
      "no-unused-vars": "off",
      "unused-imports/no-unused-imports": "error",
      "unused-imports/no-unused-vars": [
        "error",
        { argsIgnorePattern: "^_", varsIgnorePattern: "^_" },
      ],
      "no-console": ["warn", { allow: ["warn", "error"] }],
      "@stylistic/indent": ["error", 2],
      "@stylistic/jsx-indent": ["error", 2],
      "@stylistic/jsx-indent-props": ["error", 2],
      "@stylistic/comma-dangle": ["error", "always-multiline"],
      "react-refresh/only-export-components": [
        "warn",
        { allowConstantExport: true },
      ],
      "local/no-wildcard-exports": "error",
      "local/no-direct-text-color": "error",
      "local/no-arbitrary-text-size": "error",
      "local/no-arbitrary-px-classname": "error",
    },
  },
  {
    ...react.configs.flat.recommended,
    settings: { react: { version: "detect" } },
    rules: {
      ...react.configs.flat.recommended.rules,
      "react/prop-types": "off",
      "react/react-in-jsx-scope": "off",
      "react/jsx-uses-react": "off",
    },
  },
  react.configs.flat["jsx-runtime"],
  reactHooks.configs["recommended-latest"],
  jsxA11y.flatConfigs.recommended,
  {
    files: [
      "**/*.{test,spec}.{ts,tsx}",
      "src/**/*.e2e.spec.ts",
      "src/**/*.{test,spec}.{ts,tsx}",
      ".storybook/**/*.{js,jsx,ts,tsx}",
      "vite.config.ts",
      "playwright.config.ts",
    ],
    languageOptions: {
      globals: {
        ...globals.node,
      },
    },
  },
  prettier,
  ...storybook.configs["flat/recommended"],
);
