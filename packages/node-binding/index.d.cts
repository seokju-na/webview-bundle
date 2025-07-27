export interface BundleFile {
  path: string;
  data: Buffer;
}

export declare class Bundle {
  constructor();
  readAllFiles(): Promise<Array<BundleFile>>;
  readFile(path: string): Promise<BundleFile>;
  readFileData(path: string): Promise<Buffer>;
}

export declare function decode(buf: Buffer): Promise<Bundle>;
export declare function encode(bundle: Bundle): Promise<Buffer>;

export declare function create(files: BundleFile[]): Promise<Bundle>;
