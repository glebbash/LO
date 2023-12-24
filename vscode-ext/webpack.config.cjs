//@ts-check
"use strict";

const path = require("path");

/** @type {import('webpack').Configuration} */
const baseConfig = {
    mode: "none",
    target: "webworker",
    resolve: {
        mainFields: ["module", "main"],
        extensions: [".ts", ".js"],
        alias: {},
        fallback: {},
    },
    module: {
        rules: [
            {
                test: /\.ts$/,
                exclude: /node_modules/,
                use: [{ loader: "ts-loader" }],
            },
            {
                test: /\.wasm$/,
                type: "asset/inline",
            },
        ],
    },
    externals: { vscode: "commonjs vscode" },
    performance: { hints: false },
    devtool: "source-map",
    stats: "summary",
};

/** @type {import('webpack').Configuration} */
const browserClientConfig = {
    ...baseConfig,
    context: path.join(__dirname, "client"),
    entry: { browserClientMain: "./src/browserClientMain.ts" },
    output: {
        filename: "[name].js",
        path: path.join(__dirname, "client", "dist"),
        libraryTarget: "commonjs",
    },
    resolve: {
        ...baseConfig.resolve,
        fallback: { path: require.resolve("path-browserify") },
    },
};

/** @type {import('webpack').Configuration} */
const browserServerConfig = {
    ...baseConfig,
    context: path.join(__dirname, "server"),
    entry: { browserServerMain: "./src/browserServerMain.ts" },
    output: {
        filename: "[name].js",
        path: path.join(__dirname, "server", "dist"),
        libraryTarget: "var",
        library: "serverExportVar",
    },
};

module.exports = [browserClientConfig, browserServerConfig];
