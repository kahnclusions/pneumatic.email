import { A } from "@solidjs/router"
import { IconCalendar, IconHome, IconMail, IconSearch, IconSettings } from "~/ui/icons"
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem
} from "~/ui/sidebar"
import { For } from "solid-js"
import { Link } from "./components/Link"
import { Button } from "./ui/button"
import { Popover, PopoverContent, PopoverTrigger } from "./ui/popover"

const items = [
  {
    title: "General",
    url: "/settings"
  },
  {
    title: "Accounts",
    url: "/settings/accounts"
  },
  {
    title: "Appearance",
    url: "/settings/appearance"
  },
  {
    title: "Privacy & Security",
    url: "/settings/security"
  }
]

export function AppSidebar() {
  return (
    <Sidebar>
      <SidebarHeader>
        <Popover>
          <PopoverTrigger as={Button} variant="outline" class="justify-start rounded-xl mt-8">
            Contacts
          </PopoverTrigger>
          <PopoverContent>
            <ul>
              <li>Mail</li>
              <li>Contacts</li>
              <li>Calendar</li>
            </ul>
          </PopoverContent>
        </Popover>
      </SidebarHeader>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel>Settings</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu class="pl-2">
              <For each={items}>
                {item => (
                  <SidebarMenuItem>
                    <SidebarMenuButton as={Link} href={item.url}>
                      <span class="inline-flex flex-row gap-2 text-base items-center">
                        {item.title}
                      </span>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                )}
              </For>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
      <SidebarFooter class="p-4">
        <Button as={Link} href="/settings">
          <span class="inline-flex flex-row items-center gap-2 text-xl">
            <IconSettings class="w-6 h-6" />
            Settings
          </span>
        </Button>
      </SidebarFooter>
    </Sidebar>
  )
}
