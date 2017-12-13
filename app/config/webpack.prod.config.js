const webpack = require('webpack');
const path = require('path');
const rendererPath = path.join(__dirname, '../renderer');
const distPath = path.join(__dirname, '../dist');
const HtmlWebpackPlugin = require('html-webpack-plugin');

const WebpackProdConfig = {
  devtool: 'source-map',
  context: rendererPath,
  target: 'electron-renderer',

  entry: {
    app: ['./index.js'],
  },
  output: {
    path: distPath,
    filename: '[name].bundle.js',
  },
  resolve: {
    modules: [path.resolve(__dirname, '../renderer'), 'node_modules'],
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
      },
    ],
  },

  plugins: [
    new webpack.optimize.CommonsChunkPlugin({
      name: 'vendor',
      minChunks: module =>
        module.context && module.context.indexOf('node_modules') !== -1,
    }),
    new webpack.EnvironmentPlugin({
      NODE_ENV: 'production',
      DEBUG: false,
    }),
    new HtmlWebpackPlugin({
      title: 'Spectrum',
      inject: false,
      minify: {
        collapseWhitespace: true,
        conservativeCollapse: true,
        preserveLineBreaks: true,
      },
      appMountId: 'root',
      template: require('html-webpack-template'),
    }),
  ],
  stats: {
    colors: {
      green: '\u001B[32m',
    },
  },
};
module.exports = WebpackProdConfig;
