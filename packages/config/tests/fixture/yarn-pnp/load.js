const { loadConfigFile } = require('@webview-bundle/config');

const [,,...args] = process.argv;
const configFile = args[0];

loadConfigFile(configFile).catch(e => {
  console.error(e);
  process.exit(1);
});
