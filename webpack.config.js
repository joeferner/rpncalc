var webpack = require('webpack')
var getConfig = require('hjs-webpack')

module.exports = {
  devtool: 'source-map',
  entry: './src/index.tsx',
  output: {
    path: 'build',
    filename: 'bundle.js',
    sourceMapFilename: 'bundle.js.map',
    publicPath: '/'
  },
  plugins: [
    new webpack.optimize.OccurenceOrderPlugin(),
    new webpack.NoErrorsPlugin()
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
        loader: 'ts-loader'
      }
    ]
  },
  resolve: {
    extensions: ['', '.js', '.jsx', '.ts', '.tsx']
  },
  externals: {
      "nw": "nw"
  }
};