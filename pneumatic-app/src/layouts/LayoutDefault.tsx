import "./style.css";

import "./tailwind.css";
import type { JSX } from "solid-js";
import logoUrl from "../assets/logo.svg";
import { Link } from "../components/Link.js";
import { SidebarProvider, SidebarTrigger } from "~/ui/sidebar.jsx";
import { AppSidebar } from "~/sidebar.jsx";

export default function LayoutDefault(p: { children?: JSX.Element }) {
  return (
    <SidebarProvider>
      <main class="flex absolute top-0 left-0 right-0 bottom-0 bg-sidebar">
        <AppSidebar />
        <div class={`
           w-full p-4 flex flex-col gap-2
          bg-background rounded-xl m-2 mt-10 shadow-sm
          `}>
          <SidebarTrigger />
          {p.children}
        </div>
      </main>
    </SidebarProvider>
  );
}

function Sidebar(props: { children: JSX.Element }) {
  return (
    <div id="sidebar" class={"p-5 flex flex-col shrink-0 border-r-2 border-r-gray-200"}>
      {props.children}
    </div>
  );
}

function Content(props: { children: JSX.Element }) {
  return (
    <div id="page-container">
      <div id="page-content" class={"p-5 pb-12 min-h-screen"}>
        {props.children}
      </div>
    </div>
  );
}

function Logo() {
  return (
    <div class={"p-5 mb-2"}>
      <a href="/">
        <img src={logoUrl} height={64} width={64} alt="logo" />
      </a>
    </div>
  );
}
