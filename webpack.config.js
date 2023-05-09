const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const path = require('path');

const mode = process.env.NODE_ENV || 'development';
const prod = mode === 'production';

module.exports = {
  entry: {
    bundle: ['./app/main.js']
  },
  resolve: {
    alias: {
      svelte: path.resolve('node_modules', 'svelte')
    },
    extensions: ['.mjs', '.js', '.svelte'],
    mainFields: ['svelte', 'browser', 'module', 'main']
  },
  output: {
    publicPath: 'build/',
    path: path.resolve(__dirname, 'app/build'),
    filename: '[name].js',
    chunkFilename: '[name].[id].js'
  },
  module: {
    rules: [
      {
        test: /\.svelte$/,
        use: {
          loader: 'svelte-loader',
          options: {
            emitCss: true,
            hotReload: true
          }
        }
      },
      {
        test: /\.css$/,
        use: [
          /**
           * MiniCssExtractPlugin doesn't support HMR.
           * For developing, use 'style-loader' instead.
           * */
          prod ? MiniCssExtractPlugin.loader : 'style-loader',
          'css-loader'
        ]
      }
    ]
  },
  mode,
  plugins: [
    new MiniCssExtractPlugin({
      filename: '[name].css'
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, 'app/wasm_modules/wasm_rust'),
      outDir: path.resolve(__dirname, 'app/wasm_modules/wasm_rust/pkg'),
    }),
  ],
  devServer: {
    watchOptions: {
      ignored: [/.#/]
    },
    headers: {
      'Cross-Origin-Opener-Policy': 'same-origin',
    },
  },
  devtool: prod ? false: 'eval-source-map'
};
