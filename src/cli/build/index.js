const { glob } = require("glob");
const { processPath, configatronInitializer, Blueprint, Configatron } = require("../../../index");

async function builderInit() {
    configatronInitializer();

    const blueprint = new Blueprint();
    const configatron = new Configatron();

    const { ignore, include } = JSON.parse(configatron.collectsFromJs(["ignore", "include"]));
    console.log("ignore: ", ignore);
    console.log("include: ", include);
    
    const files = await glob("**/*.{js,jsx,ts,tsx}", {
        cwd: process.cwd(),
        ignore: ["node_modules/**", "**/.*"],
    });

    blueprint.title("Galadriel3CSS Build Environment");
    blueprint.info("the build environment started successfully");

    files.forEach((file) => {
        processPath(file);
    });

    blueprint.info("the build environment just finished");
}

module.exports.builderInit = builderInit;
