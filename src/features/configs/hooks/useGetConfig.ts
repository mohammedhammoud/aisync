import { useConfigsStore } from "@/base/store/configsStore";

export function useGetConfig(configId: string) {
  const configs = useConfigsStore((state) => state.configs);
  const config = configs?.find((c) => c.id === configId) ?? null;
  const isLoading = configs === null;

  return { config, isLoading };
}
