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
        gray: {
          100: "#f3f4f6",
          200: "#e5e7eb",
          500: "#6b7280",
          800: "#1f2937",
        },
      },
    },
  },
  plugins: [],
};
