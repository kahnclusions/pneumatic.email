import tailwindcss from "@tailwindcss/vite"
import vikeSolid from "vike-solid/vite"
import vike from "vike/plugin"
import { defineConfig } from "vite"
import tsconfigPaths from "vite-tsconfig-paths"

const host = process.env.TAURI_DEV_HOST

export default defineConfig(async () => ({
  plugins: [vike(), vikeSolid(), tailwindcss(), tsconfigPaths()],
  build: {
    target: "es2022"
  },
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421
        }
      : undefined,
    watch: {
      ignored: ["**/pneumatic-tauri/**"]
    }
  }
}))
