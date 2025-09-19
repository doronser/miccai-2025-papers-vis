import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => ({
  plugins: [react()],
  server: {
    port: 5173,
    host: '0.0.0.0', // Allow external connections for Docker
    proxy: {
      '/api': {
        target: process.env.DOCKER === 'true'
          ? 'http://backend:8000'  // Docker service name
          : 'http://localhost:8000', // Local development
        changeOrigin: true,
      },
    },
  },
}))
