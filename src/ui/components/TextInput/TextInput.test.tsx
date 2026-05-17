import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { ThemeProvider } from "@/ui/theme/ThemeProvider";
import { TextInput } from "@/ui/components/TextInput";

describe("TextInput", () => {
  it("accepts typed text", async () => {
    const user = userEvent.setup();

    render(
      <ThemeProvider>
        <TextInput aria-label="Name" />
      </ThemeProvider>,
    );

    const input = screen.getByLabelText("Name");
    await user.type(input, "Audit");

    expect(input).toHaveValue("Audit");
  });
});
