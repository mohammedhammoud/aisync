import { useTranslation } from "react-i18next";

export function useLanguage() {
  const { i18n } = useTranslation();

  function changeLanguage(language: string) {
    i18n.changeLanguage(language);
  }

  const language = (i18n.resolvedLanguage ?? i18n.language).split("-")[0];

  return { changeLanguage, language };
}
