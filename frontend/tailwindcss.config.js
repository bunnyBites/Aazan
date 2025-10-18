/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        indigo: {
          600: "#6366f1",
          700: "#6366f1",
        },
        coral: "#fb7185",
      },
    },
  },
  plugins: [],
};
