import { createContext } from "react";

export type AppLockState = {
  isLocked: boolean;
  message: string | null;
};

export type AppLockContextValue = {
  isLocked: boolean;
  lockMessage: string | null;
  setLockState: (lockState: AppLockState) => void;
  releaseLock: () => void;
};

export const AppLockContext = createContext<AppLockContextValue | null>(null);
