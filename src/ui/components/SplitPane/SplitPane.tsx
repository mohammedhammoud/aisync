import type { ReactNode } from "react";
import { Pane } from "@/ui/components/Pane";

type SplitPaneProps = {
  detail: ReactNode;
  list: ReactNode;
};

export function SplitPane({ detail, list }: SplitPaneProps) {
  return (
    <section className="grid h-full grid-cols-[280px_minmax(0,1fr)] gap-4 overflow-hidden">
      <Pane>{list}</Pane>
      <Pane>{detail}</Pane>
    </section>
  );
}
