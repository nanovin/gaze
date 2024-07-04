import type { Config } from "tailwindcss";
const plugin = require("tailwindcss/plugin");

const config: Config = {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      typography: {
        white: {
          css: {
            color: "green",
            "a:hover": {
              color: "whatever",
            },
          },
        },
      },
    },
  },
  plugins: [
    plugin(({ addVariant }: any) => {
      addVariant("group-hover", [".group:hover &", ".group.hover &"]);
      addVariant("hover", ["&:hover", "&.hover"]);
      addVariant("active", ["&:active", "&.active"]);
    }),
  ],
};
export default config;
