const path = require("path");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const HTMLWebpackPlugin = require("html-webpack-plugin");
const MiniCSSExtractPlugin = require("mini-css-extract-plugin");
const { VueLoaderPlugin } = require("vue-loader");

/**
 * @type {import("webpack").Configuration}
 */
const config = {
  mode: "production",
  entry: "./src/index.ts",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "[contenthash].js",
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        loader: "ts-loader",
        options: { appendTsSuffixTo: [/\.vue$/] },
      },
      {
        test: /\.vue$/,
        loader: "vue-loader",
      },
      {
        test: /\.css$/,
        use: [
          MiniCSSExtractPlugin.loader,
          { loader: "css-loader", options: { importLoaders: 1 } },
          "postcss-loader",
        ],
      },
      {
        test: /\.png$/,
        loader: "url-loader",
      },
    ],
  },
  resolve: { extensions: [".js", ".ts", ".vue"] },
  plugins: [
    new CleanWebpackPlugin(),
    new HTMLWebpackPlugin({ template: "public/index.html" }),
    new MiniCSSExtractPlugin({ filename: "[contenthash].css" }),
    new VueLoaderPlugin(),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
};

module.exports = config;
