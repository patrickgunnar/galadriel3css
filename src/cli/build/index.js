const { glob } = require("glob");
const { processPath } = require("../../../index");

async function builderInit() {
    const files = await glob("**/*.{js,jsx,ts,tsx}", {
        cwd: process.cwd(),
        ignore: ["node_modules/**", "**/.*"],
    });

    files.forEach((file) => {
        processPath(file);
    });
}

module.exports.builderInit = builderInit;
