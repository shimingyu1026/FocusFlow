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
      },
      fontFamily: {
        'pixel': ['"VT323"', 'monospace'],
        'pixel-old': ['"Press Start 2P"', 'monospace'],
      }
    },
  },
  plugins: [],
}
