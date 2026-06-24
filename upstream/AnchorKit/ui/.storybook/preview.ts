import type { Preview } from "@storybook/react";

const preview: Preview = {
  parameters: {
    controls: {
      matchers: {
        color: /(accent|color|background)/i,
      },
    },
    layout: "centered",
  },
};

export default preview;
