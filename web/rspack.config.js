import { defineConfig } from '@rspack/cli';
import { VueLoaderPlugin } from 'vue-loader';
import { HtmlRspackPlugin } from '@rspack/core';

export default defineConfig({
  entry: './src/main.js',
  output: {
    filename: 'main.js',
    path: new URL('./dist', import.meta.url).pathname,
  },
  resolve: {
    extensions: ['.js', '.vue', '.json', '.wasm'],
  },
  module: {
    rules: [
      {
        test: /\.vue$/,
        loader: 'vue-loader',
      },
      {
        test: /\.css$/,
        use: [
          {
            loader: 'vue-style-loader',
          },
          {
            loader: 'css-loader',
          },
        ],
        type: 'javascript/auto',
      },
    ],
  },
  plugins: [
    new VueLoaderPlugin(),
    new HtmlRspackPlugin({
      template: './index.html',
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
  devServer: {
    port: 3000,
    hot: true,
    open: true,
  },
});
