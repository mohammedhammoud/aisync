import { create } from "zustand";
import type { Variant } from "@/ui/theme/variants";

export type ToastInput = {
  message: string;
  variant?: Variant;
};

type ToastMessage = {
  id: number;
  message: string;
  variant: Variant;
};

type ToastState = {
  toasts: ToastMessage[];
};

export const useToastStore = create<ToastState>(() => ({
  toasts: [],
}));

export function createToast({ message, variant = "neutral" }: ToastInput) {
  const id = Date.now() + Math.floor(Math.random() * 1000);
  useToastStore.setState((s) => ({
    toasts: [...s.toasts, { id, message, variant }],
  }));
  window.setTimeout(() => dismissToast(id), 3500);
}

export function dismissToast(id: number) {
  useToastStore.setState((s) => ({
    toasts: s.toasts.filter((t) => t.id !== id),
  }));
}
