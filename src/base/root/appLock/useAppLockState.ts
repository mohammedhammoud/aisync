import { useEffect } from "react";
import type { AppLockState } from "./AppLockContext";
import { useAppLockContext } from "./useAppLockContext";

export function useAppLockState({ isLocked, message }: AppLockState) {
  const { setLockState } = useAppLockContext();

  useEffect(() => {
    setLockState({ isLocked, message });
    return () => setLockState({ isLocked: false, message: null });
  }, [isLocked, message, setLockState]);
}
