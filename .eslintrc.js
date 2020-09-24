module.exports = {
  env: {
    browser: true,
    es6: true,
    node: true,
  },
  extends: [
    'airbnb-base',
    'prettier',
    'prettier/vue',
    'plugin:prettier/recommended',
    'plugin:gridsome/recommended',
  ],
  parserOptions: {
    ecmaVersion: 2019,
    sourceType: 'module',
  },
  plugins: ['gridsome', 'vue', 'prettier'],
  settings: {
    'import/resolver': {
      alias: {
        map: [
          ['@', './src'],
          ['~', './src'],
        ],
        extensions: ['.js', '.vue'],
      },
    },
  },
}
