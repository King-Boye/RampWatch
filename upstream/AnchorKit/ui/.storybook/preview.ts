/// <reference types="vite/client" />
import type { Preview } from "@storybook/react";
import { themes } from "storybook/theming";
import "./preview.css";

const preview: Preview = {
  parameters: {
    darkMode: {
      dark: { ...themes.dark, appBg: "#0f172a", appContentBg: "#1e293b" },
      light: { ...themes.normal, appBg: "#f8fafc", appContentBg: "#ffffff" },
      current: "light",
      stylePreview: true,
      darkClass: "dark",
      lightClass: "light",
      classTarget: "html",
    },
    backgrounds: { disable: true },
    controls: {
      matchers: {
        color: /(accent|color|background)/i,
      },
    },
    layout: "centered",
  },
};

export default preview;
