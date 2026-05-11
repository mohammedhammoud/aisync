import { useContext } from "react";
import { AppLockContext } from "./AppLockContext";

export function useAppLockContext() {
  const value = useContext(AppLockContext);
  if (!value) {
    throw new Error("useAppLockContext must be used inside AppLockProvider");
  }
  return value;
}
