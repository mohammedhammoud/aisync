export const globalClasses = {
  shellBackground:
    "bg-[#f1ebff] bg-[radial-gradient(circle_at_top_left,rgba(125,78,255,0.24),transparent_34%),linear-gradient(135deg,#fff_0%,#f3ecff_44%,#e6dcff_100%)] dark:bg-[#090b12] dark:bg-[radial-gradient(circle_at_top_left,rgba(125,78,255,0.16),transparent_32%),linear-gradient(135deg,#11131d_0%,#090b12_46%,#07080d_100%)]",
  shellText: "text-zinc-950 dark:text-[#f4f1ff]",
  textPrimary: "text-zinc-950 dark:text-[#f4f1ff]",
  textSecondary: "text-zinc-700 dark:text-[#d8d2ea]",
  textMuted: "text-zinc-600 dark:text-[#9b93ad]",
  textSubtle: "text-zinc-600 dark:text-[#81778f]",
  sidebarBackground: "bg-white/55 backdrop-blur dark:bg-[#0d1018]/75",
  sidebarBorder: "border-violet-200/70 dark:border-white/10",
  surfaceBackground:
    "bg-white/75 ring-1 ring-violet-200/45 shadow-[0_1rem_3rem_rgba(91,33,182,0.08)] dark:bg-white/5 dark:ring-0 dark:shadow-none",
  focusRing:
    "focus-visible:ring-violet-500/45 dark:focus-visible:ring-[#a987ff]/65",
  focusRingOffsetShell:
    "focus-visible:ring-offset-[#f7f4ff] dark:focus-visible:ring-offset-[#0d1018]",
  disabledOpacity: "disabled:opacity-60",
  disabledOpacityStatic: "opacity-60",
} as const;
