'use client'

import { Avatar } from '@/components/catalyst/avatar'
import {
  Dropdown,
  DropdownButton,
  DropdownDivider,
  DropdownItem,
  DropdownLabel,
  DropdownMenu,
} from '@/components/catalyst/dropdown'
import { Navbar, NavbarItem, NavbarSection, NavbarSpacer } from '@/components/catalyst/navbar'
import {
  Sidebar,
  SidebarBody,
  SidebarFooter,
  SidebarHeader,
  SidebarHeading,
  SidebarItem,
  SidebarLabel,
  SidebarSection,
  SidebarSpacer,
} from '@/components/catalyst/sidebar'
import { SidebarLayout } from '@/components/catalyst/sidebar-layout'
import {
  ArrowRightStartOnRectangleIcon,
  ChevronDownIcon,
  ChevronUpIcon,
  Cog8ToothIcon,
  LightBulbIcon,
  PlusIcon,
  ShieldCheckIcon,
  UserCircleIcon,
} from '@heroicons/react/16/solid'
import {
  Cog6ToothIcon,
  HomeIcon,
  QuestionMarkCircleIcon,
  SparklesIcon,
  TicketIcon,
  ChatBubbleLeftRightIcon,
  CodeBracketSquareIcon,
  ArchiveBoxArrowDownIcon,
} from '@heroicons/react/20/solid'
import { invoke } from '@tauri-apps/api/tauri'
import { usePathname } from 'next/navigation'
import { useState, useEffect } from 'react'
import { Badge, Alert, Skeleton } from 'antd/lib'

function AccountDropdownMenu({ anchor }: { anchor: 'top start' | 'bottom end' }) {
  return (
    <DropdownMenu className="min-w-64" anchor={anchor}>
      <DropdownItem href="#">
        <UserCircleIcon />
        <DropdownLabel>My account</DropdownLabel>
      </DropdownItem>
      <DropdownDivider />
      <DropdownItem href="#">
        <ShieldCheckIcon />
        <DropdownLabel>Privacy policy</DropdownLabel>
      </DropdownItem>
      <DropdownItem href="#">
        <LightBulbIcon />
        <DropdownLabel>Share feedback</DropdownLabel>
      </DropdownItem>
      <DropdownDivider />
      <DropdownItem href="#">
        <ArrowRightStartOnRectangleIcon />
        <DropdownLabel>Sign out</DropdownLabel>
      </DropdownItem>
    </DropdownMenu>
  )
}

export function ApplicationLayout({
  // events,
  children,
}: {
  // events: Awaited<ReturnType<typeof getEvents>>
  children: React.ReactNode
}) {
  let pathname = usePathname()

  const [mega_status, setMegaStatus] = useState(false)
  const [ztm_status, setZtmStatus] = useState(true)

  useEffect(() => {
    const fetchStatus = () => {
      invoke('mega_service_status')
        .then((status: boolean[]) => {
          setMegaStatus(status[0]);
          setZtmStatus(status[1]);
          console.log(`Service Status: ${status}`);
        })
        .catch((error) => {
          console.error(`Failed to get service status: ${error}`);
        });
    };
    fetchStatus();
    // Set up interval to fetch status every 10 seconds
    const interval = setInterval(fetchStatus, 10000);
    // Clean up interval on unmount
    return () => clearInterval(interval);
  }, [])

  return (
    <SidebarLayout
      navbar={
        <Navbar>
          <NavbarSpacer />

          <NavbarSection>
            <Dropdown>
              <DropdownButton as={NavbarItem}>
                <Avatar src={"/images/megaLogo.png"} />
              </DropdownButton>
              <AccountDropdownMenu anchor="bottom end" />
            </Dropdown>
          </NavbarSection>

        </Navbar>
      }
      sidebar={
        <Sidebar>
          <SidebarHeader>
            <Dropdown>
              <DropdownButton as={SidebarItem}>
                <Avatar src="/images/megaLogo.png" />
                <SidebarLabel>Mega Status:</SidebarLabel>
                <Badge status={mega_status ? "success" : "default"} text={mega_status ? "On" : "Off"} />
                <ChevronDownIcon />
              </DropdownButton>
              <DropdownMenu className="min-w-80 lg:min-w-64" anchor="bottom start">
                <DropdownItem href="/settings">
                  <Cog8ToothIcon />
                  <DropdownLabel>Settings</DropdownLabel>
                </DropdownItem>
                <DropdownDivider />
                <DropdownItem href="#">
                  <Avatar slot="icon" initials="AD" className="bg-purple-500 text-white" />
                  <DropdownLabel>Admin</DropdownLabel>
                </DropdownItem>
                <DropdownItem href="#">
                  <Avatar slot="icon" initials="BE" className="bg-purple-500 text-white" />
                  <DropdownLabel>Big Events</DropdownLabel>
                </DropdownItem>
                <DropdownDivider />
                <DropdownItem href="#">
                  <PlusIcon />
                  <DropdownLabel>New team&hellip;</DropdownLabel>
                </DropdownItem>
              </DropdownMenu>
            </Dropdown>
          </SidebarHeader>

          <SidebarBody>
            <SidebarSection>
              <SidebarItem href="/" current={pathname === '/'}>
                <HomeIcon />
                <SidebarLabel>Code & Issue</SidebarLabel>
              </SidebarItem>
              <SidebarItem href="/chat" current={pathname.startsWith('/chat')}>
                <ChatBubbleLeftRightIcon />
                <SidebarLabel>AI Chat</SidebarLabel>
              </SidebarItem>
              <SidebarItem href="/repo" current={pathname.startsWith('/repo')}>
                <ArchiveBoxArrowDownIcon />
                <SidebarLabel>Repos</SidebarLabel>
              </SidebarItem>
              <SidebarItem href="/reminder" current={pathname.startsWith('/reminder')}>
                <TicketIcon />
                <SidebarLabel>Reminder</SidebarLabel>
              </SidebarItem>
              <SidebarItem href="/logs" current={pathname.startsWith('/logs')}>
                <CodeBracketSquareIcon />
                <SidebarLabel>Logs</SidebarLabel>
              </SidebarItem>
              <SidebarItem href="/settings" current={pathname.startsWith('/settings')}>
                <Cog6ToothIcon />
                <SidebarLabel>Settings</SidebarLabel>
              </SidebarItem>
            </SidebarSection>
            <SidebarSpacer />

            <SidebarSection>
              <SidebarItem href="#">
                <QuestionMarkCircleIcon />
                <SidebarLabel>Support</SidebarLabel>
              </SidebarItem>
              <SidebarItem href="#">
                <SparklesIcon />
                <SidebarLabel>Changelog</SidebarLabel>
              </SidebarItem>
            </SidebarSection>
          </SidebarBody>


          {/* <SidebarFooter className="max-lg:hidden">
            {token &&
              <Dropdown>
                <DropdownButton as={SidebarItem}>
                  <span className="flex min-w-0 items-center gap-3">
                    <Avatar src={"" || user.avatar_url} slot="icon" initials="AD" className="size-10 bg-purple-500 text-white" />
                    <span className="min-w-0">
                      <span className="block truncate text-sm/5 font-medium text-zinc-950 dark:text-white">{user.login}</span>
                      <span className="block truncate text-xs/5 font-normal text-zinc-500 dark:text-zinc-400">
                        {user.email}
                      </span>
                    </span>
                  </span>
                  <ChevronUpIcon />
                </DropdownButton>
                <AccountDropdownMenu anchor="top start" />
              </Dropdown>
            }
          </SidebarFooter> */}
        </Sidebar>
      }
    >
      {
        !ztm_status &&
        <Alert
          banner
          message={
            "Relay server is not connected, Some functions are not available"
          }
        />
      }
      {children}
    </SidebarLayout>
  )
}
