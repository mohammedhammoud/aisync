import { describe, expect, test } from "vitest";
import { cx } from "./cx";

describe("cx", () => {
  test("joins string classes in order", () => {
    expect(cx("one", "two", "three")).toBe("one two three");
  });

  test("ignores falsey values", () => {
    expect(cx("one", false, null, undefined, "two")).toBe("one two");
  });

  test("includes object keys with truthy flags", () => {
    expect(cx({ one: true, two: false, three: null, four: undefined })).toBe(
      "one",
    );
  });

  test("flattens nested arrays", () => {
    expect(cx("one", ["two", ["three", false]], "four")).toBe(
      "one two three four",
    );
  });

  test("handles mixed inputs", () => {
    expect(
      cx("one", ["two", { three: true, four: false }], { five: true }),
    ).toBe("one two three five");
  });
});
