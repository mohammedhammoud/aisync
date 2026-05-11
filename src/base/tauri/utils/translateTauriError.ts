import i18n from "@/i18n";
import type { AppError, AppErrorCode } from "../bindings";

const fallbackCode: AppErrorCode = "unknown";

function isAppError(error: unknown): error is AppError {
  return (
    typeof error === "object" &&
    error !== null &&
    "code" in error &&
    typeof (error as { code?: unknown }).code === "string" &&
    "message" in error &&
    typeof (error as { message?: unknown }).message === "string"
  );
}

export function translateTauriError(error: unknown): string {
  if (!isAppError(error)) {
    return String(error);
  }

  const translated = i18n.t(`errors.${error.code}`, { defaultValue: "" });
  if (translated) return translated;

  return i18n.t(`errors.${fallbackCode}`);
}
