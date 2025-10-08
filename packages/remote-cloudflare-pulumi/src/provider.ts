import path from 'node:path';
import { fileURLToPath } from 'node:url';
import * as cloudflare from '@pulumi/cloudflare';
import type {
  WorkerObservability,
  WorkerSubdomain,
  WorkerTailConsumer,
  WorkerVersionAssets,
  WorkerVersionBinding,
  WorkerVersionLimits,
  WorkerVersionPlacement,
} from '@pulumi/cloudflare/types/input.js';
import * as pulumi from '@pulumi/pulumi';

const dirname =
  typeof import.meta.dirname === 'string' ? import.meta.dirname : path.dirname(fileURLToPath(import.meta.url));

export interface WebviewBundleRemoteProviderConfig {
  accountId: string;
  bucketName?: string;
  bucketJurisdiction?: 'default' | 'eu' | 'fedramp';
  bucketLocation?: 'apac' | 'eeur' | 'enam' | 'weur' | 'wnam' | 'oc';
  bucketStorageClass?: 'Standard' | 'InfrequentAccess';
  kvTitle?: string;
  workerName?: string;
  workerLogpush?: boolean;
  workerObservability?: WorkerObservability;
  workerSubdomain?: WorkerSubdomain;
  workerTags?: string[];
  workerTailConsumers?: WorkerTailConsumer[];
  workerCompatibilityDate?: string;
  workerCompatibilityFlags?: string[];
  workerAssets?: WorkerVersionAssets;
  workerLimits?: WorkerVersionLimits;
  workerPlacement?: WorkerVersionPlacement;
  workerFile?: string;
  workerSourcemap?: boolean;
  workerSourcemapFile?: string;
  workerKvBindingName?: string;
  workerR2BucketBindingName?: string;
  workerBindings?: WorkerVersionBinding[];
}

export class WebviewBundleRemoteProvider extends pulumi.ComponentResource {
  public readonly bucketName: pulumi.Output<string>;
  public readonly kvNamespaceId: pulumi.Output<string>;
  public readonly workerId: pulumi.Output<string>;
  public readonly workerVersionId: pulumi.Output<string>;
  public readonly workerDeploymentId: pulumi.Output<string>;

  constructor(name: string, config: WebviewBundleRemoteProviderConfig, opts?: pulumi.ComponentResourceOptions) {
    super('webview-bundle:cloudflare:RemoteProvider', name, {}, opts);

    const {
      accountId,
      bucketName = 'webview-bundle',
      bucketJurisdiction,
      bucketLocation,
      bucketStorageClass,
      kvTitle = 'webview-bundle',
      workerName = 'webview-bundle',
      workerLogpush,
      workerObservability,
      workerSubdomain,
      workerTags,
      workerTailConsumers,
      workerCompatibilityDate = new Date().toISOString().split('T')[0]!,
      workerCompatibilityFlags,
      workerAssets,
      workerLimits,
      workerPlacement,
      workerFile,
      workerSourcemap = true,
      workerSourcemapFile,
      workerKvBindingName = 'KV',
      workerR2BucketBindingName = 'BUCKET',
      workerBindings = [],
    } = config;

    const bucket = new cloudflare.R2Bucket(
      'bucket',
      {
        accountId,
        name: bucketName,
        jurisdiction: bucketJurisdiction,
        location: bucketLocation,
        storageClass: bucketStorageClass,
      },
      { parent: this }
    );

    const kv = new cloudflare.WorkersKvNamespace(
      'kv',
      {
        accountId,
        title: kvTitle,
      },
      { parent: this }
    );

    const worker = new cloudflare.Worker(
      'worker',
      {
        accountId,
        name: workerName,
        logpush: workerLogpush,
        observability: workerObservability,
        subdomain: workerSubdomain,
        tags: workerTags,
        tailConsumers: workerTailConsumers,
      },
      { parent: this, dependsOn: [bucket, kv] }
    );

    const scriptName = `${workerName}.mjs`;
    const defaultWorkerFile = path.join(dirname, 'worker-script.js');
    const defaultWorkerSourcemapFile = path.join(dirname, 'worker-script.js.map');
    const workerVersion = new cloudflare.WorkerVersion(
      'worker_version',
      {
        workerId: worker.id,
        accountId,
        mainModule: scriptName,
        compatibilityDate: workerCompatibilityDate,
        compatibilityFlags: workerCompatibilityFlags,
        assets: workerAssets,
        limits: workerLimits,
        bindings: [
          {
            type: 'r2_bucket',
            bucketName: bucketName,
            name: workerR2BucketBindingName,
          },
          {
            type: 'kv_namespace',
            namespaceId: kv.id,
            name: workerKvBindingName,
          },
          ...workerBindings,
        ],
        modules: [
          {
            name: scriptName,
            contentType: 'application/javascript+module',
            contentFile: workerFile ?? defaultWorkerFile,
          },
          workerSourcemap
            ? {
                name: `${scriptName}.map`,
                contentType: 'application/source-map',
                contentFile: workerSourcemapFile ?? defaultWorkerSourcemapFile,
              }
            : null,
        ].filter(x => x != null),
        placement: workerPlacement,
      },
      { parent: this, dependsOn: [worker] }
    );

    const workerDeployment = new cloudflare.WorkersDeployment(
      'worker_deployment',
      {
        accountId,
        scriptName: workerName,
        strategy: 'percentage',
        versions: [
          {
            percentage: 100,
            versionId: workerVersion.id,
          },
        ],
      },
      { parent: this, dependsOn: [worker, workerVersion] }
    );

    this.bucketName = bucket.name;
    this.kvNamespaceId = kv.id;
    this.workerId = worker.id;
    this.workerVersionId = workerVersion.id;
    this.workerDeploymentId = workerDeployment.id;
    this.registerOutputs({
      bucketName: this.bucketName,
      kvNamespaceId: this.kvNamespaceId,
      workerId: this.workerId,
      workerVersionId: this.workerVersionId,
      workerDeploymentId: this.workerDeploymentId,
    });
  }
}

export const WvbRemoteProvider = WebviewBundleRemoteProvider;
