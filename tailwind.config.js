/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["src/**/*.ts", "src/**/*.vue"],
  theme: { extend: {} },
  plugins: [require("@tailwindcss/forms")],
};
