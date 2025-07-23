import { defineConfig } from 'vite'
import { resolve } from 'path'

export default defineConfig({
  build: {
    target: 'node18',
    outDir: 'dist',
    lib: {
      entry: resolve(__dirname, 'src/index.ts'),
      name: 'kaeru',
      fileName: 'index',
      formats: ['cjs']
    },
    rollupOptions: {
      output: {
        format: 'cjs'
      }
    },
    minify: 'esbuild',
    sourcemap: true
  }
}) 