import { describe, expect, test } from "vitest";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import en from "@/base/i18n/locales/en.json";
import sv from "@/base/i18n/locales/sv.json";

type LocaleValue = string | Locale;
type Locale = {
  [key: string]: LocaleValue;
};

function flatten(locale: Locale, prefix = ""): Record<string, string> {
  return Object.fromEntries(
    Object.entries(locale).flatMap(([key, value]) => {
      const fullKey = prefix ? `${prefix}.${key}` : key;

      if (typeof value === "string") {
        return [[fullKey, value]];
      }

      return Object.entries(flatten(value, fullKey));
    }),
  );
}

function interpolationKeys(value: string): string[] {
  return [...value.matchAll(/{{\s*([^}\s]+)\s*}}/g)]
    .map((match) => match[1])
    .sort();
}

function sourceFiles(directory: string): string[] {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const fullPath = join(directory, entry.name);

    if (entry.isDirectory()) {
      return sourceFiles(fullPath);
    }

    return /\.(ts|tsx)$/.test(entry.name) ? [fullPath] : [];
  });
}

function usedTranslationKeys(): Set<string> {
  const source = sourceFiles(join(process.cwd(), "src"))
    .map((file) => readFileSync(file, "utf8"))
    .join("\n");
  const keys = new Set<string>();

  for (const match of source.matchAll(/\bt\(\s*[`'"]([^`'"]+)/g)) {
    const key = match[1];
    if (!key.includes("${")) keys.add(key);
  }

  for (const match of source.matchAll(/i18nTitleKey:\s*[`'"]([^`'"]+)/g)) {
    keys.add(match[1]);
  }

  return keys;
}

describe("locales", () => {
  const enKeys = flatten(en);
  const svKeys = flatten(sv);

  test("keeps locale keys in sync", () => {
    expect(Object.keys(svKeys).sort()).toEqual(Object.keys(enKeys).sort());
  });

  test("keeps interpolation placeholders in sync", () => {
    for (const [key, enValue] of Object.entries(enKeys)) {
      expect(interpolationKeys(svKeys[key]), key).toEqual(
        interpolationKeys(enValue),
      );
    }
  });

  test("does not keep stale locale keys", () => {
    const usedKeys = usedTranslationKeys();
    const staleKeys = Object.keys(enKeys).filter((key) => {
      if (usedKeys.has(key) || key.startsWith("errors.")) {
        return false;
      }

      const singularKey = key.replace(/_plural$/, "");
      return singularKey === key || !usedKeys.has(singularKey);
    });

    expect(staleKeys).toEqual([]);
  });
});
