const chokidar = require("chokidar");
const { processPath } = require("../../../index");

function watcherInit() {
    const watcher = chokidar.watch(
        ["**/*.js", "**/*.jsx", "**/*.ts", "**/*.tsx"],
        {
            cwd: process.cwd(),
            ignoreInitial: true,
            ignored: ["node_modules/**", "**/.*"],
        }
    );

    watcher.on("all", (_, path) => {
        processPath(path);
    });
}

module.exports.watcherInit = watcherInit;
