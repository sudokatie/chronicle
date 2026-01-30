/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  darkMode: 'class',
  theme: {
    extend: {
      fontFamily: {
        mono: ['JetBrains Mono', 'monospace'],
      },
      colors: {
        bg: {
          primary: '#1e1e2e',
          secondary: '#181825',
          tertiary: '#11111b',
        },
        fg: {
          primary: '#cdd6f4',
          secondary: '#a6adc8',
          muted: '#6c7086',
        },
        accent: {
          blue: '#89b4fa',
          green: '#a6e3a1',
          yellow: '#f9e2af',
          red: '#f38ba8',
          purple: '#cba6f7',
        },
      },
    },
  },
  plugins: [],
}
