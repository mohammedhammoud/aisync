import { createLazyRoute } from "@tanstack/react-router";
import { InstructionsEditor } from "@/features/instructions/components/InstructionsEditor";
import { useInstructionsEditor } from "@/features/instructions/hooks/useInstructionsEditor";

function InstructionsView() {
  const {
    discardChanges,
    instructions,
    isDirty,
    saveInstructions,
    setInstructions,
  } = useInstructionsEditor();

  return (
    <InstructionsEditor
      content={instructions}
      isDirty={isDirty}
      onChange={setInstructions}
      onDiscard={discardChanges}
      onSave={saveInstructions}
    />
  );
}

export const Route = createLazyRoute("/instructions")({
  component: InstructionsView,
});
