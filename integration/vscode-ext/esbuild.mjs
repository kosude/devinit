/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

import esbuild from "esbuild";

const buildDir = process.env["JS_BUILD_DIR"];
const isProd = process.env["NODE_ENV"] === "production";

console.log(buildDir)

const configBase = {
    bundle: true,
    minify: isProd,
    sourcemap: !isProd,
    logOverride: {
        "direct-eval": "silent"
    }
};
const configExtension = {
    ...configBase,
    platform: "node",
    format: "cjs",
    entryPoints: ["src/extension.ts"],
    outfile: `${buildDir}/extension.js`,
    external: ["vscode"]
};

try {
    await esbuild.build(configExtension);
    console.log("Build finished");
} catch (err) {
    // output build errors
    console.error(err);
    process.exit(1);
}
