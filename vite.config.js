import { defineConfig } from "vite";
import { resolve } from 'path';

export default defineConfig(async () => ({
  root: resolve(__dirname, 'dist'), // Указываем, что корень — это папка dist
  build: {
    minify: false, // ОТКЛЮЧАЕМ минификацию (чтобы не ломались скрипты)
    target: 'esnext',
    rollupOptions: {
      input: {
        main: resolve(__dirname, 'dist/index.html'), // Говорим, какой файл брать
      },
    },
  },
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
}));
