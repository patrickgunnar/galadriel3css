const { glob } = require("glob");

async function builderInit() {
    const files = await glob("**/*.{js,jsx,ts,tsx}", {
        cwd: process.cwd(),
        ignore: ["node_modules/**", "**/.*"],
    });

    files.forEach((file) => {
        console.log(file);
    });
}

module.exports.builderInit = builderInit;
