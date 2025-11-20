import { loadConfigFile } from '@webview-bundle/config';

const [,,...args] = process.argv;
const configFile = args[0];

await loadConfigFile(configFile);
