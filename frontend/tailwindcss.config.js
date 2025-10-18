// File: frontend/tailwind.config.js

/** @type {import('tailwindcss').Config} */
export const content = [
  // Tell Tailwind to scan all .rs files in your src folder
  "./src/**/*.rs",
];
export const theme = {
  extend: {
    colors: {
      indigo: {
        600: "#6366f1",
      },
      coral: "#fb7185",
    },
  },
};
export const plugins = [];
