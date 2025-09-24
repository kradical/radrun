import path from "node:path";
import viteReact from "@vitejs/plugin-react";
import { defineConfig } from "vite";

// biome-ignore lint/style/noDefaultExport: need to default export vite config
export default defineConfig({
  plugins: [viteReact()],
  resolve: {
    alias: {
      "@api": path.resolve(__dirname, "./src/api"),
    },
  },
  server: {
    proxy: {
      "/api": "http://localhost:3000",
    },
  },
});
