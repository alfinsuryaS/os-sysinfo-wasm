const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    index: "./www/js/index.js",
  },
  module: {
    rules: [
      {
        test: /\.css$/i,
        use: [
          { loader: 'style-loader', options: { injectType: 'styleTag' } },
          'css-loader',
        ],
      },
    ],
  },
  output: {
    path: dist,
    filename: "[name].js"
  },
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new CopyPlugin([
      path.resolve(__dirname, "www")
    ]),

    new WasmPackPlugin({
      crateDirectory: __dirname,
      extraArgs: "--out-name index"
    }),
    
  ]
};
