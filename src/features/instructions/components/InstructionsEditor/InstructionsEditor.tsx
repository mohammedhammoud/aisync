import { RotateCcw, Save } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Button } from "@/ui/components/Button";
import { TextEditor } from "@/ui/components/TextEditor";
import { Toolbar } from "@/ui/components/Toolbar";

type InstructionsEditorProps = {
  content: string;
  isDirty: boolean;
  onChange: (value: string) => void;
  onDiscard: () => void;
  onSave: () => void;
};

export function InstructionsEditor({
  content,
  isDirty,
  onChange,
  onDiscard,
  onSave,
}: InstructionsEditorProps) {
  const { t } = useTranslation();

  return (
    <div className="flex h-full flex-col gap-4">
      <TextEditor
        fill={true}
        label={t("instructions.global")}
        value={content}
        onChange={onChange}
      />
      <Toolbar>
        {isDirty && (
          <Button
            icon={<RotateCcw size={15} />}
            onClick={onDiscard}
            variant="yellow"
          >
            {t("dirty.discard")}
          </Button>
        )}
        <div className="ml-auto flex items-center gap-2">
          <Button
            disabled={!isDirty}
            icon={<Save size={15} />}
            onClick={onSave}
            variant="violet"
          >
            {t("instructions.save")}
          </Button>
        </div>
      </Toolbar>
    </div>
  );
}
