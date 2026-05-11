import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";
import { createToast, dismissToast, useToastStore } from "./toastStore";

describe("toastStore", () => {
  beforeEach(() => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2026-01-01T00:00:00.000Z"));
    vi.spyOn(Math, "random").mockReturnValue(0.123);
    useToastStore.setState({ toasts: [] });
  });

  afterEach(() => {
    vi.useRealTimers();
    vi.restoreAllMocks();
  });

  test("creates neutral toast by default", () => {
    createToast({ message: "Saved" });

    expect(useToastStore.getState().toasts).toEqual([
      {
        id: Date.now() + 123,
        message: "Saved",
        variant: "neutral",
      },
    ]);
  });

  test("creates toast with custom variant", () => {
    createToast({ message: "Nope", variant: "red" });

    expect(useToastStore.getState().toasts[0]).toMatchObject({
      message: "Nope",
      variant: "red",
    });
  });

  test("dismisses toast by id", () => {
    createToast({ message: "Saved" });
    const id = useToastStore.getState().toasts[0].id;

    dismissToast(id);

    expect(useToastStore.getState().toasts).toEqual([]);
  });

  test("auto-dismisses toast after timeout", () => {
    createToast({ message: "Saved" });

    vi.advanceTimersByTime(3499);
    expect(useToastStore.getState().toasts).toHaveLength(1);

    vi.advanceTimersByTime(1);
    expect(useToastStore.getState().toasts).toEqual([]);
  });
});
