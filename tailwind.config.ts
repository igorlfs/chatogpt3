import { Config } from "tailwindcss";

const text = "#cdd6f4";
const subtext = "#a6adc8";

const lavender = "#b4befe";

const base = "#1e1e2e";
const mantle = "#181825";
const crust = "#11111b";

const surface0 = "#313244";
const surface1 = "#45475a";
const surface2 = "#585b70";

const overlay0 = "#6c7086";

const blue = "#89b4fa";

export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    colors: {
      primary: lavender,
      accent: blue,
      text,
      subtext,
      lavender,
      surface0,
      surface1,
      surface2,
      overlay0,
      base,
      mantle,
      crust,
      blue,
    },
    extend: {},
  },
  plugins: [],
} satisfies Config;
