import fs from 'node:fs/promises';
import path from 'node:path';

export interface PublishOptions {
  /** Name of bundle. */
  name: string;
  /** Bundle file path. */
  bundle: string;
  /** Version for bundle. */
  version: string;
}

export abstract class Publisher {
  public abstract readonly name: string;

  public abstract publish(options: PublishOptions): Promise<void>;

  protected async readBundle(bundle: string): Promise<Buffer> {
    const filepath = path.isAbsolute(bundle) ? bundle : path.join(process.cwd(), bundle);
    return fs.readFile(filepath);
  }
}
