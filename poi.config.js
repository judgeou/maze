const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  output: {
    html: {
      template: './index.html'
    }
  },
  configureWebpack: {
    plugins: [
      new WasmPackPlugin({
        crateDirectory: __dirname
      })
    ]
  }
}