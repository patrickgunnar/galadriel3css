const chokidar = require("chokidar");
const { processPath, Blueprint, Configatron, processGatekeeper } = require("../../../index");

function watcherInit() {
    const blueprint = new Blueprint();
    const configatron = new Configatron();

    const { ignore, include } = JSON.parse(configatron.collectsFromJs(["ignore", "include"]));
    console.log("ignore: ", ignore);
    console.log("include: ", include);

    const watcher = chokidar.watch(
        ["**/*.js", "**/*.jsx", "**/*.ts", "**/*.tsx", "galadriel.json"],
        {
            cwd: process.cwd(),
            ignoreInitial: true,
            ignored: ["node_modules/**", "**/.*"],
        }
    );

    blueprint.title("Galadriel3CSS Development Environment");
    blueprint.info("the development environment started successfully");

    watcher.on("all", (_, path) => {
        if (path.includes("galadriel.json")) {
            blueprint.warn("modification in 'galadriel.json' detected");
            blueprint.info("restart the development environment to apply changes");
            watcher.close();
        } else {
            processPath(path);
            processGatekeeper();
        }
    });
}

module.exports.watcherInit = watcherInit;
