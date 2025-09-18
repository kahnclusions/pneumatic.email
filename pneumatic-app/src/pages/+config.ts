import type { Config } from "vike/types";
import vikeSolid from "vike-solid/config";
import Layout from "../layouts/LayoutDefault.js";

export default {
  Layout,
  title: "My Vike App",
  description: "Demo showcasing Vike",
  extends: vikeSolid,

  // Ensure Vike doesn't use SSR and only SSG for Tauri.
  prerender: true,
  ssr: false,

  bodyAttributes: {
    class: "bg-zinc-100 text-foreground"
  }
} satisfies Config;
