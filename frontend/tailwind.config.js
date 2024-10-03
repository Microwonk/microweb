/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
      files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
      extend: {
        fontFamily: {
			'sans': ['FiraSans', 'ui-sans-serif', 'system-ui', 'sans-serif', "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"],
			'mono': ['Iosevka Custom', 'ui-monospace', 'SFMono-Regular', 'Menlo', 'Monaco', 'Consolas', "Liberation Mono", "Courier New", 'monospace'],
		},
      },
  },
  corePlugins: {
      preflight: false,
  },
  plugins: [
      require('@tailwindcss/forms'),
  ],
}