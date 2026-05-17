export const variants = ["neutral", "red", "yellow", "violet"] as const;

export type Variant = (typeof variants)[number];
export type VariantMode = "solid" | "outline";

type VariantState = {
  background: string;
  readableColor: string;
};

type VariantProps = {
  base: VariantState;
  hover: VariantState;
  active: VariantState;
  selected: VariantState;
  border: string;
  checked: string;
  focus: string;
  text: string;
};

export type VariantType = Record<VariantMode, VariantProps>;

export const VARIANT: Record<Variant, VariantType> = {
  neutral: {
    outline: {
      base: {
        background: "bg-neutral-50 dark:bg-neutral-950",
        readableColor: "text-black dark:text-white",
      },
      hover: {
        background: "hover:bg-neutral-200 dark:hover:bg-neutral-900",
        readableColor: "hover:text-black dark:text-white",
      },
      active: {
        background: "active:bg-neutral-300 dark:active:bg-neutral-900",
        readableColor: "active:text-black dark:active:text-white",
      },
      selected: {
        background: "bg-neutral-300 dark:bg-neutral-900",
        readableColor: "text-black dark:text-white",
      },
      border: "border border-neutral-300 dark:border-neutral-600",
      checked:
        "data-[checked]:border-neutral-500 data-[checked]:bg-neutral-500 data-[checked]:text-white dark:data-[checked]:border-neutral-400 dark:data-[checked]:bg-neutral-400 dark:data-[checked]:text-black",
      focus: "focus:outline-neutral-500 focus:border-neutral-400",
      text: "text-neutral-600 dark:text-neutral-400",
    },
    solid: {
      base: {
        background: "bg-neutral-100 dark:bg-neutral-800",
        readableColor: "text-black dark:text-white",
      },
      hover: {
        background: "hover:bg-neutral-200 dark:hover:bg-neutral-700",
        readableColor: "hover:text-black dark:text-white",
      },
      active: {
        background: "active:bg-neutral-300 dark:active:bg-neutral-700",
        readableColor: "active:text-black dark:active:text-white",
      },
      selected: {
        background: "bg-neutral-300 dark:bg-neutral-700",
        readableColor: "text-black dark:text-white",
      },
      border: "border border-neutral-200 dark:border-neutral-700",
      checked:
        "data-[checked]:border-neutral-700 data-[checked]:bg-neutral-700 data-[checked]:text-white dark:data-[checked]:border-neutral-200 dark:data-[checked]:bg-neutral-200 dark:data-[checked]:text-black",
      focus: "focus:outline-neutral-500 focus:border-neutral-300",
      text: "text-neutral-600 dark:text-neutral-300",
    },
  },
  red: {
    outline: {
      base: {
        background: "bg-red-50 dark:bg-red-950",
        readableColor: "text-black dark:text-white",
      },
      hover: {
        background: "hover:bg-red-200 dark:hover:bg-red-900",
        readableColor: "hover:text-black dark:hover:text-white",
      },
      active: {
        background: "active:bg-red-300 dark:active:bg-red-800",
        readableColor: "active:text-black dark:active:text-white",
      },
      selected: {
        background: "bg-red-300 dark:bg-red-900",
        readableColor: "text-black dark:text-white",
      },
      border: "border border-red-300 dark:border-red-700",
      checked:
        "data-[checked]:border-red-600 data-[checked]:bg-red-600 data-[checked]:text-white dark:data-[checked]:border-red-500 dark:data-[checked]:bg-red-500 dark:data-[checked]:text-white",
      focus: "focus:outline-red-500 focus:border-red-400",
      text: "text-red-600 dark:text-red-400",
    },
    solid: {
      base: {
        background: "bg-red-100 dark:bg-red-900",
        readableColor: "text-black dark:text-white",
      },
      hover: {
        background: "hover:bg-red-200 dark:hover:bg-red-800",
        readableColor: "hover:text-black dark:text-white",
      },
      active: {
        background: "active:bg-red-300 dark:active:bg-red-800",
        readableColor: "active:text-black dark:active:text-white",
      },
      selected: {
        background: "bg-red-300 dark:bg-red-800",
        readableColor: "text-black dark:text-white",
      },
      border: "border border-red-200 dark:border-red-800",
      checked:
        "data-[checked]:border-red-700 data-[checked]:bg-red-700 data-[checked]:text-white dark:data-[checked]:border-red-400 dark:data-[checked]:bg-red-400 dark:data-[checked]:text-black",
      focus: "focus:outline-red-500 focus:border-red-300",
      text: "text-red-600 dark:text-red-300",
    },
  },
  yellow: {
    outline: {
      base: {
        background: "bg-yellow-50 dark:bg-yellow-950",
        readableColor: "text-black dark:text-white",
      },
      hover: {
        background: "hover:bg-yellow-200 dark:hover:bg-yellow-900",
        readableColor: "hover:text-black dark:hover:text-white",
      },
      active: {
        background: "active:bg-yellow-300 dark:active:bg-yellow-800",
        readableColor: "active:text-black dark:active:text-white",
      },
      selected: {
        background: "bg-yellow-300 dark:bg-yellow-900",
        readableColor: "text-black dark:text-white",
      },
      border: "border border-yellow-300 dark:border-yellow-700",
      checked:
        "data-[checked]:border-yellow-500 data-[checked]:bg-yellow-500 data-[checked]:text-black dark:data-[checked]:border-yellow-400 dark:data-[checked]:bg-yellow-400 dark:data-[checked]:text-black",
      focus: "focus:outline-yellow-500 focus:border-yellow-400",
      text: "text-yellow-600 dark:text-yellow-400",
    },
    solid: {
      base: {
        background: "bg-yellow-100 dark:bg-yellow-900",
        readableColor: "text-black dark:text-white",
      },
      hover: {
        background: "hover:bg-yellow-200 dark:hover:bg-yellow-800",
        readableColor: "hover:text-black dark:text-white",
      },
      active: {
        background: "active:bg-yellow-300 dark:active:bg-yellow-800",
        readableColor: "active:text-black dark:active:text-white",
      },
      selected: {
        background: "bg-yellow-300 dark:bg-yellow-800",
        readableColor: "text-black dark:text-white",
      },
      border: "border border-yellow-200 dark:border-yellow-800",
      checked:
        "data-[checked]:border-yellow-600 data-[checked]:bg-yellow-600 data-[checked]:text-black dark:data-[checked]:border-yellow-300 dark:data-[checked]:bg-yellow-300 dark:data-[checked]:text-black",
      focus: "focus:outline-yellow-500 focus:border-yellow-300",
      text: "text-yellow-600 dark:text-yellow-300",
    },
  },
  violet: {
    outline: {
      base: {
        background: "bg-violet-50 dark:bg-violet-950",
        readableColor: "text-black dark:text-white",
      },
      hover: {
        background: "hover:bg-violet-200 dark:hover:bg-violet-900",
        readableColor: "hover:text-black dark:hover:text-white",
      },
      active: {
        background: "active:bg-violet-300 dark:active:bg-violet-800",
        readableColor: "active:text-black dark:active:text-white",
      },
      selected: {
        background: "bg-violet-300 dark:bg-violet-900",
        readableColor: "text-black dark:text-white",
      },
      border: "border border-violet-300 dark:border-violet-700",
      checked:
        "data-[checked]:border-violet-600 data-[checked]:bg-violet-600 data-[checked]:text-white dark:data-[checked]:border-violet-500 dark:data-[checked]:bg-violet-500 dark:data-[checked]:text-white",
      focus: "focus:outline-violet-500 focus:border-violet-400",
      text: "text-violet-700 dark:text-violet-400",
    },
    solid: {
      base: {
        background: "bg-violet-600 dark:bg-violet-900",
        readableColor: "text-white",
      },
      hover: {
        background: "hover:bg-violet-700 dark:hover:bg-violet-800",
        readableColor: "hover:text-white",
      },
      active: {
        background: "active:bg-violet-800 dark:active:bg-violet-800",
        readableColor: "active:text-white",
      },
      selected: {
        background: "bg-violet-600 dark:bg-violet-900",
        readableColor: "text-white",
      },
      border: "border border-violet-600 dark:border-violet-800",
      checked:
        "data-[checked]:border-violet-600 data-[checked]:bg-violet-600 data-[checked]:text-white",
      focus:
        "focus:outline-violet-500 focus:border-violet-500 dark:focus:border-violet-300",
      text: "text-violet-700 dark:text-violet-300",
    },
  },
};
