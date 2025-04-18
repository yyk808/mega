const config = {
  rules: {
    'no-restricted-imports': [
      'error',
      {
        paths: [
          {
            name: 'next/link',
            message: "Please import it from '@gitmono/ui/Link' instead."
          },
          {
            name: 'framer-motion',
            importNames: ['useInView'],
            message: "Please import it from 'react-intersection-observer' instead."
          },
          {
            name: 'react-hotkeys-hook',
            importNames: ['useHotkeys'],
            message: "Please import it from '@gitmono/ui' instead."
          },
          {
            name: 'react-error-boundary',
            message: "Please import it from '@gitmono/ui' instead."
          }
        ]
      }
    ]
  }
}

module.exports = config
