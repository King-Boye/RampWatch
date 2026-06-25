import type { StorybookConfig } from "@storybook/react-vite";

const config: StorybookConfig = {
  stories: ["../components/**/*.stories.@(ts|tsx)"],
  addons: ["@storybook/addon-essentials", "storybook-dark-mode"],
  framework: {
    name: "@storybook/react-vite",
    options: {},
  },
};

export default config;
