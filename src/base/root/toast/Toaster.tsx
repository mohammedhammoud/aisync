import { Transition } from "@headlessui/react";
import { Alert } from "@/ui/components/Alert";
import { dismissToast, useToastStore } from "@/base/store/toastStore";

export function Toaster() {
  const toasts = useToastStore((state) => state.toasts);

  return (
    <div className="pointer-events-none fixed bottom-4 left-1/2 z-[60] flex -translate-x-1/2 flex-col items-center gap-2 px-4">
      {toasts.map((toast) => (
        <Transition
          appear
          as="div"
          className="pointer-events-auto w-fit max-w-[calc(100vw-2rem)]"
          enter="transition-all duration-150"
          enterFrom="opacity-0 scale-95"
          enterTo="opacity-100 scale-100"
          key={toast.id}
          leave="transition-all duration-150"
          leaveFrom="opacity-100 scale-100"
          leaveTo="opacity-0 scale-95"
          show
        >
          <Alert
            onClick={() => dismissToast(toast.id)}
            role="status"
            variant={toast.variant}
          >
            {toast.message}
          </Alert>
        </Transition>
      ))}
    </div>
  );
}
