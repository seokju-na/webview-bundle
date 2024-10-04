export declare class Bundle {
  constructor();
  readFile(path: string): Promise<Buffer>;
}

export declare function decode(buf: Buffer): Promise<Bundle>;
export declare function encode(bundle: Bundle): Promise<Buffer>;

export interface File {
  path: string;
  data: Buffer;
}
export declare function create(files: File[]): Promise<Bundle>;
