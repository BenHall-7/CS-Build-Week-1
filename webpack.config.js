const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

const distPath = path.resolve(__dirname, "dist");
module.exports = (env, argv) => {
  return {
    entry: './bootstrap.js',
    output: {
      path: distPath,
      filename: "conway/conway.js",
      webassemblyModuleFilename: "conway/conway.wasm"
    },
    plugins: [
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript",
      })
    ],
  };
};
