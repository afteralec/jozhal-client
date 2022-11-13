/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      gridTemplateColumns: {
        layout: "2fr 3fr 2fr"
      }
    }
  },
  plugins: []
};
