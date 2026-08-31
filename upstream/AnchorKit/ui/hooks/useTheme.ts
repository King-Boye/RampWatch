import { useEffect, useState } from "react";

const STORAGE_KEY = "theme";

function getInitialDark(): boolean {
  if (typeof localStorage !== "undefined") {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (saved === "dark") return true;
    if (saved === "light") return false;
  }
  if (typeof window !== "undefined" && typeof window.matchMedia === "function") {
    return window.matchMedia("(prefers-color-scheme: dark)").matches;
  }
  return false;
}

/**
 * Returns whether the current effective theme is dark.
 * Reads `prefers-color-scheme` and re-renders on change.
 * Components that have their own manual toggle can pass `override`
 * to bypass the media query.
 *
 * This hook also:
 * - Applies a `dark` CSS class and `data-theme` attribute to the document element
 * - Persists the resolved theme to `localStorage` under the key "theme"
 * - Restores the saved preference on initialization
 */
export function useTheme(override?: boolean): boolean {
  const [sysDark, setSysDark] = useState<boolean>(getInitialDark);

  const isDark = override !== undefined ? override : sysDark;

  useEffect(() => {
    if (typeof window === "undefined" || typeof window.matchMedia !== "function") {
      return;
    }

    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    setSysDark(mq.matches);
    const handler = (e: MediaQueryListEvent) => setSysDark(e.matches);
    mq.addEventListener("change", handler);
    return () => mq.removeEventListener("change", handler);
  }, []);

  useEffect(() => {
    if (typeof document !== "undefined") {
      document.documentElement.classList.toggle("dark", isDark);
      document.documentElement.setAttribute("data-theme", isDark ? "dark" : "light");
    }
    if (typeof localStorage !== "undefined") {
      localStorage.setItem(STORAGE_KEY, isDark ? "dark" : "light");
    }
  }, [isDark]);

  return isDark;
}
