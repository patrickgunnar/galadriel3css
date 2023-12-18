const yargs = require("yargs");
const { galadrielInit } = require("./init");
const { watcherInit } = require("./dev");
const { builderInit } = require("./build");

yargs
    .command({
        command: "init",
        describe: "Configure the Galadriel3CSS environment.",
        handler: (_) => {
            console.clear();
            galadrielInit();
        },
    })
    .command({
        command: "dev",
        describe: "Watch the Galadriel3CSS environment",
        handler: async (_) => {
            console.clear();
            await builderInit();
            console.clear();
            watcherInit();
        },
    })
    .command({
        command: "build",
        describe: "Build the Galadriel3CSS environment.",
        handler: async (_) => {
            console.clear();
            await builderInit();
        },
    });

yargs.parse();
