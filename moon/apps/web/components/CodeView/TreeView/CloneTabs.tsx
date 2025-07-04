import React, { useEffect, useState } from 'react'
import { Button, Input, Popover, Space, Tabs, TabsProps } from 'antd'
import copy from 'copy-to-clipboard'
import { usePathname } from 'next/navigation'
import { CheckIcon, CopyIcon, DownloadIcon } from '@gitmono/ui/Icons'
import {LEGACY_API_URL} from '@gitmono/config'

const CloneTabs = ({ endpoint }: any) => {
  const pathname = usePathname()
  const [text, setText] = useState<string>(pathname || '')
  const [copied, setCopied] = useState<boolean>(false)
  const [active_tab, setActiveTab] = useState<string>('1')

  const onChange = (key: string) => {
    setActiveTab(key)
  }

  useEffect(() => {
    if (LEGACY_API_URL) {
      const url = new URL(LEGACY_API_URL)

      if (active_tab === '1') {
        setText(`${url.href}${pathname?.replace('/myorganization/code/tree/', '')}.git`)
      } else {
        setText(`ssh://git@${url.host}${pathname?.replace('/myorganization/code/tree', '')}.git`)
      }
    }
  }, [pathname, active_tab, endpoint])

  const handleCopy = () => {
    copy(text)
    setCopied(true)
    setTimeout(() => setCopied(false), 2000) // Reset after 2 seconds
  }

  const tab_items: TabsProps['items'] = [
    {
      key: '1',
      label: 'HTTP',
      children: (
        <Space style={{ width: '100%' }}>
          <Input value={text} />
          <Button onClick={handleCopy} icon={copied ? <CheckIcon /> : <CopyIcon />} size={'small'} />
        </Space>
      )
    },
    {
      key: '2',
      label: 'SSH',
      children: (
        <Space style={{ width: '100%' }}>
          <Input value={text} />
          <Button onClick={handleCopy} icon={copied ? <CheckIcon /> : <CopyIcon />} size={'small'} />
        </Space>
      )
    }
  ]

  return (
    <Popover
      placement='bottomRight'
      content={<Tabs defaultActiveKey='1' items={tab_items} onChange={onChange} />}
      trigger='click'
    >
      <Button icon={<DownloadIcon />}>Code</Button>
    </Popover>
  )
}

export default CloneTabs
