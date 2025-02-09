/** @type {import('tailwindcssl').Config} */
module.exports = {
  mode: "jit",
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {},
  },
  plugins: [],
}