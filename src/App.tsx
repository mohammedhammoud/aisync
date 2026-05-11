import { RouterProvider } from "@tanstack/react-router";
import { AppLockProvider } from "@/base/root/appLock";
import { Toaster } from "@/base/root/toast/Toaster";
import { router } from "@/base/root/router";

function App() {
  return (
    <AppLockProvider>
      <RouterProvider router={router} />
      <Toaster />
    </AppLockProvider>
  );
}

export default App;
