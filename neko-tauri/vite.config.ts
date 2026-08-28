import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [react()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    // 【新增：反向代理，接管 Nginx 的工作】
    proxy: {
      // 记忆服务
      '/memory': {
        target: 'http://127.0.0.1:48912',
        changeOrigin: true,
      },
      // 智能体插件服务
      '/agent': {
        target: 'http://127.0.0.1:48915',
        changeOrigin: true,
      },
      // 主聊天服务 (通常的前端 API 请求路径)
      '/api': {
        target: 'http://127.0.0.1:48911',
        changeOrigin: true,
      },
      // 主聊天服务的 WebSocket 通信 (聊天流)
      '/ws': {
        target: 'http://127.0.0.1:48911',
        ws: true,
      }
    },
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));