export declare class Bundle {
  constructor();
  encode(): Buffer;
}

export declare function parse(buf: Buffer): Promise<Bundle>;
