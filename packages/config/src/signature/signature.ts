import { Buffer } from 'node:buffer';
import type { KMSClient, MessageType, SigningAlgorithmSpec } from '@aws-sdk/client-kms';
import type { S3UploaderOptions } from '@webview-bundle/node';

export interface AwsKmsSignatureSignerConfig {
  keyId: string;
  algorithm: SigningAlgorithmSpec;
  messageType?: MessageType;
  accessKeyId?: string;
  secretAccessKey?: string;
  sessionToken?: string;
  region?: string;
  endpoint?: string;
  client?: KMSClient;
}

export function awsKmsSignatureSigner(
  config: AwsKmsSignatureSignerConfig
): NonNullable<S3UploaderOptions['signatureSigner']> {
  return async data => {
    const { keyId, algorithm, messageType = 'RAW', client } = config;
    const kms = client ?? (await getKMSClient(config));
    const { SignCommand } = await import('@aws-sdk/client-kms');
    const output = await kms.send(
      new SignCommand({
        KeyId: keyId,
        Message: data,
        MessageType: messageType,
        SigningAlgorithm: algorithm,
      })
    );
    const { Signature: signature } = output;
    if (signature == null) {
      throw new Error('signature not found');
    }
    const encoded = Buffer.from(signature).toString('base64');
    return encoded;
  };
}

async function getKMSClient(config: AwsKmsSignatureSignerConfig): Promise<KMSClient> {
  const { accessKeyId, secretAccessKey, sessionToken, region, endpoint } = config;
  const { KMSClient } = await import('@aws-sdk/client-kms');
  const kms = new KMSClient({
    region,
    credentials:
      accessKeyId != null && secretAccessKey != null
        ? {
            accessKeyId,
            secretAccessKey,
            sessionToken,
          }
        : undefined,
    endpoint,
  });
  return kms;
}
