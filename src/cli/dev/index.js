const chokidar = require("chokidar");

function watcherInit() {
    const watcher = chokidar.watch(
        ["**/*.js", "**/*.jsx", "**/*.ts", "**/*.tsx"],
        {
            cwd: process.cwd(),
            ignoreInitial: true,
            ignored: ["node_modules/**", "**/.*"],
        }
    );

    watcher.on("all", (event, path) => {
        console.log(event, path);
    });
}

module.exports.watcherInit = watcherInit;
