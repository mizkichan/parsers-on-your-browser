const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const $ = path.resolve.bind(null, __dirname);

module.exports = {
  entry: "./js/index.jsx",

  output: {
    path: $("dist"),
    filename: "bundle.js"
  },

  devServer: {
    contentBase: $("dist")
  },

  plugins: [
    new WasmPackPlugin({
      crateDirectory: $("crate")
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
