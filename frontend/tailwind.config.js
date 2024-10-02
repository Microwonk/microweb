/** @type {import('tailwindcss').Config} */
const svgToDataUri = require("mini-svg-data-uri");
const {
  default: flattenColorPalette,
} = require("tailwindcss/lib/util/flattenColorPalette");

module.exports = {
  content: ["*.html", "./src/**/*.rs"],
  theme: {
    extend: {
      backgroundColor: {
        "nf-dark": "#0e0306",
        "nf-color": "#047857",
        "nf-white": "#dad6ca",
      },
      colors: {
        "nf-dark": "#0e0306",
        "nf-color": "#047857",
        "nf-white": "#dad6ca",
      },
      fontFamily: {
        jersey: ['jersey', 'sans-serif'],
        montserrat: ['montserrat', 'sans-serif'],
        rosmatika: ['rosmatika', 'sans-serif'],
      },
    },
  },
  plugins: [
    addVariablesForColors,
  ],
};

function addVariablesForColors({ addBase, theme }) {
  let allColors = flattenColorPalette(theme("colors"));
  let newVars = Object.fromEntries(
    Object.entries(allColors).map(([key, val]) => [`--${key}`, val])
  );

  addBase({
    ":root": newVars,
  });
}
