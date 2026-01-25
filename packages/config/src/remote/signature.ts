import { Buffer } from 'node:buffer';

export type SignatureAlgorithm = 'ecdsa' | 'ed25519' | 'rsa-pkcs1-v1.5' | 'rsa-pss';
export type SignatureEcdsaCurve = 'p256' | 'p384';
export type SignatureHash = 'sha256' | 'sha384' | 'sha512';
export type SigningKeyFormat = 'raw' | 'pkcs8' | 'spki' | 'jwk';

export type SignatureSigningKeyConfig =
  | {
      format: 'jwk';
      data: JsonWebKey;
    }
  | {
      format: Exclude<SigningKeyFormat, 'jwk'>;
      data: Buffer;
    };

export type SignatureSignConfig =
  | {
      algorithm: 'ecdsa';
      curve: SignatureEcdsaCurve;
      hash: SignatureHash;
      key: SignatureSigningKeyConfig;
    }
  | {
      algorithm: 'rsa-pkcs1-v1.5';
      hash: SignatureHash;
      key: SignatureSigningKeyConfig;
    }
  | {
      algorithm: 'rsa-pss';
      saltLength: number;
      key: SignatureSigningKeyConfig;
    }
  | {
      algorithm: Exclude<SignatureAlgorithm, 'ecdsa' | 'rsa-pkcs1-v1.5' | 'rsa-pss'>;
      key: SignatureSigningKeyConfig;
    };
export type SignatureSignFn = (params: { message: Buffer }) => Promise<string>;
export type SignatureSigner = SignatureSignConfig | SignatureSignFn;

export async function signSignature(signer: SignatureSigner, message: Buffer): Promise<string> {
  if (typeof signer === 'function') {
    return signer({ message });
  }
  const { key } = signer;
  const signingKey =
    key.format === 'jwk'
      ? await crypto.subtle.importKey(key.format, key.data, importKeyAlg(signer), true, ['sign'])
      : await crypto.subtle.importKey(key.format, new Uint8Array(key.data), importKeyAlg(signer), true, ['sign']);
  const signed = await crypto.subtle.sign(signAlg(signer), signingKey, new Uint8Array(message));
  const signedBuf = Buffer.from(signed);
  return signedBuf.toString('base64');
}

function importKeyAlg(config: SignatureSignConfig): AlgorithmIdentifier | RsaHashedImportParams | EcKeyAlgorithm {
  switch (config.algorithm) {
    case 'ecdsa':
      return {
        name: 'ECDSA',
        namedCurve: ecdsaCurveName(config.curve),
      };
    case 'ed25519':
      return { name: 'Ed25519' };
    case 'rsa-pkcs1-v1.5':
      return {
        name: 'RSASSA-PKCS1-v1_5',
        hash: hashAlg(config.hash),
      };
    case 'rsa-pss':
      return { name: 'RSA-PSS' };
  }
}

function ecdsaCurveName(curve: SignatureEcdsaCurve): string {
  switch (curve) {
    case 'p256':
      return 'P-256';
    case 'p384':
      return 'P-384';
  }
}

function hashAlg(rasHash: SignatureHash): string {
  switch (rasHash) {
    case 'sha256':
      return 'SHA-256';
    case 'sha384':
      return 'SHA-384';
    case 'sha512':
      return 'SHA-512';
  }
}

function signAlg(config: SignatureSignConfig): AlgorithmIdentifier | RsaPssParams | EcdsaParams {
  switch (config.algorithm) {
    case 'ecdsa':
      return {
        name: 'ECDSA',
        hash: hashAlg(config.hash),
      };
    case 'ed25519':
      return { name: 'Ed25519' };
    case 'rsa-pkcs1-v1.5':
      return { name: 'RSASSA-PKCS1-v1_5' };
    case 'rsa-pss':
      return {
        name: 'RSA-PSS',
        saltLength: config.saltLength,
      };
  }
}
