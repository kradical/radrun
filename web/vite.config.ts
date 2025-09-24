import viteReact from "@vitejs/plugin-react";
import { defineConfig } from "vite";

// biome-ignore lint/style/noDefaultExport: need to default export vite config
export default defineConfig({
  plugins: [viteReact()],
});
