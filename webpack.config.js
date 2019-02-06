const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  entry: "./js/index.jsx",

  output: {
    path: dist,
    filename: "bundle.js"
  },

  devServer: {
    contentBase: dist
  },

  plugins: [
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "crate")
    })
  ],

  module: {
    rules: [
      {
        test: /\.jsx?$/,
        exclude: /node_modules/,
        use: {
          loader: "babel-loader",
          options: {
            presets: ["@babel/preset-react"],
            parserOpts: {
              plugins: ["dynamicImport"]
            }
          }
        }
      }
    ]
  }
};

// vim: set ts=2 sw=2 et:
