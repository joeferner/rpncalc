var webpack = require('webpack')
var getConfig = require('hjs-webpack')
var WebpackNotifierPlugin = require('webpack-notifier');

module.exports = {
  devtool: 'eval',
  entry: './src/index.tsx',
  output: {
    path: 'build',
    filename: 'bundle.js',
    sourceMapFilename: 'bundle.js.map',
    publicPath: '/'
  },
  plugins: [
    new webpack.optimize.OccurenceOrderPlugin(),
    new webpack.NoErrorsPlugin(),
    new WebpackNotifierPlugin({alwaysNotify: true})
  ],
  module: {
    loaders: [
      {
        test: /\.js|\.jsx$/,
        exclude: /node_modules/,
        loader: 'babel',
        query: {
          "presets": ["react", "es2015", "stage-0"]
        }
      },
      {
        test: /\.ts|\.tsx$/,
        loader: 'awesome-typescript-loader'
      },
      {
        test: /nw/,
        loader: 'nw-loader'
      }
    ]
  },
  resolve: {
    extensions: ['', '.js', '.jsx', '.ts', '.tsx']
  },
  externals: {
    "fs": "fs"
  }
};