import { useEffect, useMemo, useState, type PropsWithChildren } from "react";
import { AppLockContext, type AppLockState } from "./AppLockContext";

export function AppLockProvider({ children }: PropsWithChildren) {
  const [lockState, setLockState] = useState<AppLockState>({
    isLocked: false,
    message: null,
  });

  useEffect(() => {
    function onBeforeUnload(event: BeforeUnloadEvent) {
      if (!lockState.isLocked) {
        return;
      }
      event.preventDefault();
      event.returnValue = "";
    }

    window.addEventListener("beforeunload", onBeforeUnload);
    return () => window.removeEventListener("beforeunload", onBeforeUnload);
  }, [lockState.isLocked]);

  const value = useMemo(
    () => ({
      isLocked: lockState.isLocked,
      lockMessage: lockState.message,
      setLockState,
      releaseLock: () => setLockState({ isLocked: false, message: null }),
    }),
    [lockState.isLocked, lockState.message],
  );

  return (
    <AppLockContext.Provider value={value}>{children}</AppLockContext.Provider>
  );
}
