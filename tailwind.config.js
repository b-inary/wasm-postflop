module.exports = {
  content: ["public/index.html", "src/**/*.{ts,vue}"],
  theme: { extend: {} },
  plugins: [require("@tailwindcss/forms")],
};
