import { useEffect, useState } from "react";

/**
 * Returns whether the current effective theme is dark.
 * Reads `prefers-color-scheme` and re-renders on change.
 * Components that have their own manual toggle can pass `override`
 * to bypass the media query.
 * 
 * This hook also applies the appropriate data-theme attribute to the document
 * element to enable CSS custom property theming.
 */
export function useTheme(override?: boolean): boolean {
  const [sysDark, setSysDark] = useState<boolean>(false);

  const isDark = override !== undefined ? override : sysDark;

  useEffect(() => {
    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    setSysDark(mq.matches);
    const handler = (e: MediaQueryListEvent) => setSysDark(e.matches);
    mq.addEventListener("change", handler);
    return () => mq.removeEventListener("change", handler);
  }, []);

  // Apply theme to document element
  useEffect(() => {
    if (typeof document !== "undefined") {
      document.documentElement.setAttribute("data-theme", isDark ? "dark" : "light");
    }
  }, [isDark]);

  return isDark;
}
