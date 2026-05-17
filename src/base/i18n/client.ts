import { setDefaultOptions } from "date-fns";
import { enUS, sv } from "date-fns/locale";
import i18next from "i18next";
import LanguageDetector from "i18next-browser-languagedetector";
import { initReactI18next } from "react-i18next";
import enTranslations from "@/base/i18n/locales/en.json";
import svTranslations from "@/base/i18n/locales/sv.json";

const localeByLanguage = {
  en: enUS,
  sv,
  "sv-SE": sv,
} as const;

function resolveLocale(language: string | undefined) {
  if (!language) {
    return enUS;
  }

  return (
    localeByLanguage[language as keyof typeof localeByLanguage] ??
    localeByLanguage[language.split("-")[0] as keyof typeof localeByLanguage] ??
    enUS
  );
}

function applyLanguage(language: string | undefined) {
  const locale = resolveLocale(language);
  const documentLanguage = locale === enUS ? "en" : "sv";

  setDefaultOptions({ locale });

  if (typeof document !== "undefined") {
    document.documentElement.lang = documentLanguage;
  }
}

export const i18n = i18next;

let initPromise: Promise<typeof i18n> | undefined;

export function initI18n() {
  initPromise ??= i18n
    .use(LanguageDetector)
    .use(initReactI18next)
    .init({
      resources: {
        en: { translation: enTranslations },
        sv: { translation: svTranslations },
      },
      supportedLngs: ["en", "sv"],
      fallbackLng: "en",
      detection: {
        order: ["localStorage", "navigator"],
        caches: ["localStorage"],
      },
      interpolation: {
        escapeValue: false,
      },
    })
    .then(() => {
      i18n.on("languageChanged", applyLanguage);
      applyLanguage(i18n.resolvedLanguage ?? i18n.language);

      return i18n;
    });

  return initPromise;
}
