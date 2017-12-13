//@flow
const webpack = require('webpack');
const path = require('path');
const findCacheDir = require('find-cache-dir');
const rendererPath = path.join(__dirname, '../renderer');
const distPath = path.join(__dirname, '../dist');
const DashboardPlugin = require('webpack-dashboard/plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const InlineManifestWebpackPlugin = require('inline-manifest-webpack-plugin');

const WebpackDevConfig = {
  devtool: 'inline-source-map',
  context: rendererPath,
  target: 'electron-renderer',
  resolve: {
    modules: [path.resolve(__dirname, '../renderer'), 'node_modules'],
  },
  entry: {
    app: [
      'react-hot-loader/patch',

      'react-dev-utils/webpackHotDevClient',
      './index.js',
    ],
  },
  output: {
    path: distPath,
    filename: '[name].bundle.js',
  },
  module: {
    rules: [
      {
        test: /\.(woff|svg|mkv|mp4)$/,
        loader: 'file-loader',
      },

      {
        test: /\.(js|jsx)$/,
        exclude: /node_modules/,
        loader: 'babel-loader',
        query: {
          cacheDirectory: findCacheDir({
            name: 'react-scripts',
          }),
        },
      },
    ],
  },
  plugins: [
    new webpack.optimize.CommonsChunkPlugin({
      name: 'vendor',
      minChunks: module =>
        module.context && module.context.indexOf('node_modules') !== -1,
    }),
    new webpack.optimize.CommonsChunkPlugin({
      name: 'manifest',
    }),
    new InlineManifestWebpackPlugin(),
    new HtmlWebpackPlugin({
      title: '[DEVELOPMENT] Spectrum',
      inject: false,
      appMountId: 'root',
      minify: {
        collapseWhitespace: true,
        conservativeCollapse: true,
        preserveLineBreaks: true,
      },
      inlineManifestWebpackName: 'webpackManifest',
      template: require('html-webpack-template'),
    }),
    new webpack.NamedModulesPlugin(),
    new DashboardPlugin(),
    new webpack.HotModuleReplacementPlugin(),
  ],
  stats: {
    colors: {
      green: '\u001B[32m',
    },
  },
  devServer: {
    contentBase: './src',
    historyApiFallback: true,
    port: 3000,
    compress: false,
    inline: true,
    hot: true,
    stats: {
      assets: true,
      children: false,
      chunks: false,
      hash: false,
      modules: false,
      publicPath: false,
      timings: true,
      version: false,
      warnings: true,
      colors: {
        green: '\u001B[32m',
      },
    },
  },
};

module.exports = WebpackDevConfig;
