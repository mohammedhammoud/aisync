type ClassValue =
  | string
  | false
  | null
  | undefined
  | ClassValue[]
  | Record<string, boolean | null | undefined>;

export function cx(...values: ClassValue[]): string {
  const classes: string[] = [];

  for (const value of values) {
    if (!value) {
      continue;
    }

    if (typeof value === "string") {
      classes.push(value);
      continue;
    }

    if (Array.isArray(value)) {
      const nested = cx(...value);
      if (nested) {
        classes.push(nested);
      }
      continue;
    }

    for (const [className, enabled] of Object.entries(value)) {
      if (enabled) {
        classes.push(className);
      }
    }
  }

  return classes.join(" ");
}
