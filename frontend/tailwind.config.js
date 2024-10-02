/** @type {import('tailwindcss').Config} */
const svgToDataUri = require("mini-svg-data-uri");
const {
  default: flattenColorPalette,
} = require("tailwindcss/lib/util/flattenColorPalette");

module.exports = {
  content: ["*.html", "./src/**/*.rs"],
  theme: {
    extend: {
      screens: {
        "2xl": "2400px",
      },
      boxShadow: {
        explore: "0px 0px 2px 6px rgba(206,212,218, 0.3)",
      },
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
      fontSize: {
        "10xl": "11rem",
      },
      fontFamily: {
        jersey: ['jersey', 'sans-serif'],
        montserrat: ['montserrat', 'sans-serif'],
        rosmatika: ['rosmatika', 'sans-serif'],
      },
      lineHeight: {
        tighter: 1.15,
        relaxish: 1.8,
        p: "1.75rem",
        about: "2.05rem",
        mediump: "4.35rem",
        smallp: "4.75rem",
        p: "1.75rem",
        largep: "2.75rem",
        heading: "10.313rem",
        mediumheading: "5.313rem",
        smallheading: "3.2rem",
      },
      maxWidth: {
        "8xl": "88rem",
        "10xl": "104rem",
      },
      minHeight: {
        card_1_row: "22rem",
        card_1_row_mobile: "18rem",

        card_2_row: "39rem",
        card_2_row_mobile: "18rem",
      },
    },
  },
  plugins: [
    addVariablesForColors,
    function ({ matchUtilities, theme }) {
      matchUtilities(
        {
          "bg-grid": (value) => ({
            backgroundImage: `url("${svgToDataUri(
              `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32" width="32" height="32" fill="none" stroke="${value}"><path d="M0 .5H31.5V32"/></svg>`
            )}")`,
          }),
          "bg-grid-small": (value) => ({
            backgroundImage: `url("${svgToDataUri(
              `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32" width="8" height="8" fill="none" stroke="${value}"><path d="M0 .5H31.5V32"/></svg>`
            )}")`,
          }),
          "bg-dot": (value) => ({
            backgroundImage: `url("${svgToDataUri(
              `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32" width="16" height="16" fill="none"><circle fill="${value}" id="pattern-circle" cx="10" cy="10" r="1.6257413380501518"></circle></svg>`
            )}")`,
          }),
        },
        { values: flattenColorPalette(theme("backgroundColor")), type: "color" }
      );
    },
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
