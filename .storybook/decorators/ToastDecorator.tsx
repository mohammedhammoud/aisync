import type { Decorator } from "@storybook/react-vite";
import { Toaster } from "@/base/root/toast/Toaster";

export const ToastDecorator: Decorator = (Story) => (
  <>
    <Toaster />
    <Story />
  </>
);
