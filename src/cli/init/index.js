const fs = require("fs");
const path = require("path");

const galadrielJSON = `{
    "module": true
}`;

function galadrielInit() {
    fs.writeFileSync(path.join(process.cwd(), "galadriel.json"), galadrielJSON);
    console.info("'galadriel.json' created successfully!\n");
}

module.exports.galadrielInit = galadrielInit;
