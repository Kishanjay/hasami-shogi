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
    'plugin:gridsome/recommended',
    'plugin:vue/recommended',
  ],
  parserOptions: {
    ecmaVersion: 2019,
    sourceType: 'module',
  },
  plugins: [
    'gridsome',
    'vue',
  ],
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
};
