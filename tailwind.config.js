/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'pixel-bg': '#2d1b4e',
        'pixel-green': '#39ff14',
        'pixel-pink': '#ff6ec7',
        'pixel-yellow': '#ffff00',
        'pixel-blue': '#00d9ff',
        'pixel-purple': '#b14eff',
      },
      fontFamily: {
        'pixel': ['"Press Start 2P"', 'monospace'],
      }
    },
  },
  plugins: [],
}
