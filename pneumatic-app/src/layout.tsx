import { ParentProps } from "solid-js"
import { AppSidebar } from "./sidebar"
import { Sidebar, SidebarProvider, SidebarTrigger } from "./ui/sidebar"

export function Layout(p: ParentProps) {
  return (
    <SidebarProvider>
      <main class="flex h-full w-full absolute inset bg-sidebar">
        <AppSidebar />
        <div
          class={`
         w-full p-4 flex flex-col gap-2
        bg-background rounded-xl mx-2 my-2 shadow-sm
        `}
        >
          <SidebarTrigger />
          {p.children}
        </div>
      </main>
    </SidebarProvider>
  )
}
