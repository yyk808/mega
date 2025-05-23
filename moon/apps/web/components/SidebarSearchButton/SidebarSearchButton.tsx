import { Button, SearchIcon } from '@gitmono/ui'

import { useScope } from '@/contexts/scope'

export function SidebarSearchButton() {
  const { scope } = useScope()

  return (
    <Button
      iconOnly={<SearchIcon />}
      accessibilityLabel='Search'
      tooltip='Search'
      tooltipShortcut='/'
      href={`/${scope}/search`}
      variant='plain'
    />
  )
}
