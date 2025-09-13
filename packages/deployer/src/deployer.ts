import { Buffer } from 'node:buffer';
import fs from 'node:fs/promises';
import path from 'node:path';
import type { BundleData, DeployInfo, UploadReleaseInfo } from './types.js';

export abstract class Deployer {
  abstract readonly name: string;

  abstract uploadRelease(info: UploadReleaseInfo): Promise<void>;
  abstract deploy(info: DeployInfo): Promise<void>;

  protected async readBundleData(bundleData: BundleData): Promise<Buffer> {
    if (typeof bundleData === 'string') {
      const filepath = path.isAbsolute(bundleData) ? bundleData : path.join(process.cwd(), bundleData);
      return await fs.readFile(filepath);
    }
    return Buffer.from(bundleData);
  }

  protected async writeBundleVersion(
    bundleData: BundleData,
    versionFilePath: string | undefined,
    version: string
  ): Promise<Buffer> {
    throw new Error('TODO');
    // const data = await this.readBundleData(bundleData);
    // const bundle = await decode(data);
    // const files = await bundle.readAllFiles();
    // const versionIdx = files.findIndex(x => x.path === versionFilePath);
    // const versionFile: BundleFile = { path: versionFilePath ?? '__VERSION__', data: Buffer.from(version, 'utf8') };
    // if (versionIdx > -1) {
    //   files[versionIdx] = versionFile;
    // } else {
    //   files.push(versionFile);
    // }
    // const updatedBundle = await create(files);
    // const updatedData = await encode(updatedBundle);
    // return updatedData;
  }
}
