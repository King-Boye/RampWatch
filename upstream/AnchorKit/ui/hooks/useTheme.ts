import { useEffect, useState } from "react";

/**
 * Returns whether the current effective theme is dark.
 * Reads `prefers-color-scheme` and re-renders on change.
 * Components that have their own manual toggle can pass `override`
 * to bypass the media query.
 */
export function useTheme(override?: boolean): boolean {
  const [sysDark, setSysDark] = useState<boolean>(() => {
    if (typeof window === "undefined") return false;
    return window.matchMedia("(prefers-color-scheme: dark)").matches;
  });

  useEffect(() => {
    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    const handler = (e: MediaQueryListEvent) => setSysDark(e.matches);
    mq.addEventListener("change", handler);
    return () => mq.removeEventListener("change", handler);
  }, []);

  return override !== undefined ? override : sysDark;
}
