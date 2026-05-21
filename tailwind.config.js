/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'pixel-bg': 'var(--pixel-bg)',
        'pixel-primary': 'var(--pixel-primary)',
        'pixel-primary-dark': 'var(--pixel-primary-dark)',
        'pixel-secondary': 'var(--pixel-secondary)',
        'pixel-text': 'var(--pixel-text)',
        'pixel-text-muted': 'var(--pixel-text-muted)',
        'pixel-green': 'var(--pixel-primary)',
        'pixel-blue': 'var(--pixel-primary)',
        'pixel-yellow': 'var(--pixel-warning)',
        'pixel-pink': 'var(--pixel-secondary)',
      },
      fontFamily: {
        'pixel': ['var(--app-font-serif)'],
        'pixel-old': ['var(--app-font-serif)'],
      }
    },
  },
  plugins: [],
}
