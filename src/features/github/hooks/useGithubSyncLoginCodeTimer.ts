import { useEffect, useState } from "react";
import { useGithubSyncStore } from "@/features/github/store/githubSyncStore";

const MILLISECONDS_PER_SECOND = 1000;
const GITHUB_LOGIN_AUTO_OPEN_DELAY_SECONDS = 3;

function getGithubSyncLoginExpiresInSeconds(
  expiresAtMs: number | null,
): number {
  if (!expiresAtMs) return 0;
  return Math.max(
    0,
    Math.ceil((expiresAtMs - Date.now()) / MILLISECONDS_PER_SECOND),
  );
}

export function useGithubSyncLoginCodeTimer(onOpenGithubLogin: () => void) {
  const isConnecting = useGithubSyncStore((state) => state.isConnecting);
  const login = useGithubSyncStore((state) => state.login);
  const loginExpiresAtMs = useGithubSyncStore(
    (state) => state.loginExpiresAtMs,
  );
  const [openDelaySeconds, setOpenDelaySeconds] = useState(0);
  const [expiresInSeconds, setExpiresInSeconds] = useState(0);

  useEffect(() => {
    if (!login || !isConnecting) {
      setOpenDelaySeconds(0);
      setExpiresInSeconds(0);
      return;
    }

    const updateExpiresInSeconds = () => {
      setExpiresInSeconds(getGithubSyncLoginExpiresInSeconds(loginExpiresAtMs));
    };

    setOpenDelaySeconds(GITHUB_LOGIN_AUTO_OPEN_DELAY_SECONDS);
    updateExpiresInSeconds();

    const countdown = window.setInterval(() => {
      setOpenDelaySeconds((seconds) => Math.max(0, seconds - 1));
      updateExpiresInSeconds();
    }, MILLISECONDS_PER_SECOND);
    const opener = window.setTimeout(
      onOpenGithubLogin,
      GITHUB_LOGIN_AUTO_OPEN_DELAY_SECONDS * MILLISECONDS_PER_SECOND,
    );

    return () => {
      window.clearInterval(countdown);
      window.clearTimeout(opener);
    };
  }, [isConnecting, login, loginExpiresAtMs, onOpenGithubLogin]);

  return { expiresInSeconds, openDelaySeconds };
}
