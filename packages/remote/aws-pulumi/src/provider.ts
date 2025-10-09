import path from 'node:path';
import { fileURLToPath } from 'node:url';
import * as aws from '@pulumi/aws';
import type * as inputs from '@pulumi/aws/types/input.js';
import * as pulumi from '@pulumi/pulumi';
import type { AssetArchive } from '@pulumi/pulumi/asset/archive.js';
import type { WebviewBundleRemoteConfig } from '@webview-bundle/remote-aws';
import { uniq } from 'es-toolkit';
import { generateCode } from './code.js';
import { getLambdaRuntimeTarget, type LambdaRuntime } from './types.js';

const dirname =
  typeof import.meta.dirname === 'string' ? import.meta.dirname : path.dirname(fileURLToPath(import.meta.url));

export interface WebviewBundleRemoteProviderConfig {
  name?: string;
  region?: pulumi.Input<string>;
  bucketName?: string;
  bucketForceDestroy?: pulumi.Input<boolean>;
  bucketTags?: pulumi.Input<{ [key: string]: pulumi.Input<string> }>;
  lambdaName?: pulumi.Input<string>;
  lambdaArchitecture?: pulumi.Input<pulumi.Input<'x8664' | 'arm64'>[]>;
  lambdaEnvironment?: pulumi.Input<inputs.lambda.FunctionEnvironment>;
  lambdaDescription?: pulumi.Input<string>;
  lambdaLayers?: pulumi.Input<pulumi.Input<string>[]>;
  lambdaCode?: pulumi.Input<AssetArchive>;
  lambdaCodeEsm?: pulumi.Input<boolean>;
  lambdaCodeSourcemap?: pulumi.Input<boolean>;
  lambdaCodeMinify?: pulumi.Input<boolean>;
  lambdaCodeTreeshake?: pulumi.Input<boolean>;
  lambdaRuntime?: pulumi.Input<LambdaRuntime>;
  lambdaHandler?: pulumi.Input<string>;
  lambdaRoleName?: pulumi.Input<string>;
  lambdaPolicyActions?: string[];
  lambdaRolePolicyName?: pulumi.Input<string>;
  lambdaTimeout?: pulumi.Input<number>;
  lambdaMemorySize?: pulumi.Input<number>;
  lambdaTags?: pulumi.Input<{ [key: string]: pulumi.Input<string> }>;
  cloudfrontAliases?: pulumi.Input<pulumi.Input<string>[]>;
  cloudfrontComment?: pulumi.Input<string>;
  cloudfrontViewerCertificate?: aws.cloudfront.DistributionArgs['viewerCertificate'];
  cloudfrontTags?: pulumi.Input<{ [key: string]: pulumi.Input<string> }>;
  cloudfrontEnabled?: pulumi.Input<boolean>;
  cloudfrontHttpVersion?: pulumi.Input<'http1.1' | 'http2' | 'http2and3' | 'http3'>;
  cloudfrontWaitForDeployment?: pulumi.Input<boolean>;
  allowOnlyLatest?: boolean;
}

export class WebviewBundleRemoteProvider extends pulumi.ComponentResource {
  public readonly bucketId: pulumi.Output<string>;
  public readonly bucketName: pulumi.Output<string>;
  public readonly bucketDomainName: pulumi.Output<string>;
  public readonly lambdaArn: pulumi.Output<string>;
  public readonly cloudfrontDistributionId: pulumi.Output<string>;
  public readonly cloudfrontDistributionArn: pulumi.Output<string>;
  public readonly cloudfrontDistributionDomainName: pulumi.Output<string>;

  constructor(name: string, config: WebviewBundleRemoteProviderConfig = {}, opts?: pulumi.ComponentResourceOptions) {
    super('webview-bundle:aws:RemoteProvider', name, {}, opts);

    const {
      name: baseName = 'webview-bundle',
      region,
      bucketName = `${baseName}-bucket`,
      bucketForceDestroy,
      bucketTags,
      lambdaName = `${baseName}-lambda`,
      lambdaArchitecture,
      lambdaRuntime = 'nodejs22.x',
      lambdaDescription = `${baseName} lambda function (${lambdaRuntime})`,
      lambdaEnvironment,
      lambdaLayers,
      lambdaCodeEsm = true,
      lambdaCodeSourcemap = true,
      lambdaCodeMinify = true,
      lambdaCodeTreeshake = true,
      lambdaCode,
      lambdaHandler = 'origin-request.handler',
      lambdaRoleName = `${baseName}-lambda-role`,
      lambdaPolicyActions = [],
      lambdaRolePolicyName = `${baseName}-lambda-role-policy`,
      lambdaTimeout,
      lambdaTags,
      lambdaMemorySize,
      cloudfrontAliases,
      cloudfrontEnabled = true,
      cloudfrontHttpVersion = 'http2and3',
      cloudfrontComment = `${baseName}-cdn`,
      cloudfrontViewerCertificate = {
        cloudfrontDefaultCertificate: true,
      },
      cloudfrontWaitForDeployment = true,
      cloudfrontTags,
      allowOnlyLatest,
    } = config;

    const provider = new aws.Provider(
      'provider',
      {
        region,
      },
      { parent: this }
    );
    const usEast1Provider = new aws.Provider(
      'provider_us-east-1',
      {
        region: 'us-east-1',
      },
      { parent: this }
    );

    const bucket = new aws.s3.Bucket(
      'bucket',
      {
        bucket: bucketName,
        forceDestroy: bucketForceDestroy,
        tags: bucketTags,
      },
      {
        provider,
        parent: this,
      }
    );

    const originAccessIdentity = new aws.cloudfront.OriginAccessIdentity(
      'origin_access_identity',
      {
        comment: bucketName,
      },
      { provider: usEast1Provider, parent: this }
    );
    new aws.s3.BucketPolicy(
      'bucket_policy',
      {
        bucket: bucket.id,
        policy: pulumi.all([bucket.arn, originAccessIdentity.iamArn]).apply(([bucketArn, oaiArn]) =>
          JSON.stringify({
            Version: '2012-10-17',
            Statement: [
              {
                Sid: 'AllowCloudfrontGetObject',
                Effect: 'Allow',
                Principal: {
                  AWS: oaiArn,
                },
                Action: 's3:GetObject',
                Resource: `${bucketArn}/*`,
              },
              {
                Sid: 'AllowCloudfrontListBuckets',
                Effect: 'Allow',
                Principal: {
                  AWS: oaiArn,
                },
                Action: 's3:ListBucket',
                Resource: bucketArn,
              },
            ],
          })
        ),
      },
      { provider, parent: this, dependsOn: [bucket] }
    );

    const lambdaAssumeRolePolicy = aws.iam.getPolicyDocumentOutput({
      statements: [
        {
          effect: 'Allow',
          principals: [
            {
              type: 'Service',
              identifiers: ['lambda.amazonaws.com', 'edgelambda.amazonaws.com'],
            },
          ],
          actions: ['sts:AssumeRole'],
        },
      ],
    });

    const lambdaRole = new aws.iam.Role(
      'lambda_role',
      {
        name: lambdaRoleName,
        assumeRolePolicy: lambdaAssumeRolePolicy.json,
      },
      { provider: usEast1Provider, parent: this }
    );

    const lambdaPolicy = aws.iam.getPolicyDocumentOutput({
      statements: [
        {
          effect: 'Allow',
          actions: uniq([
            // https://docs.aws.amazon.com/ko_kr/AmazonCloudFront/latest/DeveloperGuide/lambda-edge-permissions.html#lambda-edge-permissions-required
            'lambda:GetFunction',
            'lambda:EnableReplication',
            'lambda:DisableReplication',
            'iam:CreateServiceLinkedRole',
            'cloudfront:UpdateDistribution',
            // log into cloudwatch
            'logs:CreateLogGroup',
            'logs:CreateLogStream',
            'logs:PutLogEvents',
            // access to s3 bucket
            's3:GetObject',
            's3:ListBucket',
            ...lambdaPolicyActions,
          ]),
          resources: ['*'],
        },
      ],
    });

    const lambdaRolePolicy = new aws.iam.RolePolicy(
      'lambda_role_policy',
      {
        name: lambdaRolePolicyName,
        role: lambdaRole.id,
        policy: lambdaPolicy.json,
      },
      { provider: usEast1Provider, parent: this }
    );

    const code =
      lambdaCode ??
      pulumi
        .all([
          bucket.bucket,
          region,
          lambdaRuntime,
          lambdaCodeEsm,
          lambdaCodeSourcemap,
          lambdaCodeMinify,
          lambdaCodeTreeshake,
        ])
        .apply(async ([bucketName, region, runtime, esm, sourcemap, minify, treeshake]) => {
          const config: WebviewBundleRemoteConfig = {
            bucketName,
            region,
            allowOnlyLatest,
          };
          const input = path.join(dirname, '..', 'lambda', 'origin-request.ts');
          const codes = await generateCode(input, {
            platform: 'node',
            target: getLambdaRuntimeTarget(runtime),
            format: esm ? 'esm' : 'cjs',
            sourcemap,
            treeshake,
            minify,
            define: {
              __CONFIG__: JSON.stringify(config),
            },
          });
          const assets = Object.fromEntries(
            codes.map(code => {
              return [code.fileName, new pulumi.asset.StringAsset(code.content)];
            })
          );
          return new pulumi.asset.AssetArchive(assets);
        });

    const lambdaOriginRequest = new aws.lambda.Function(
      'lambda_origin_request',
      {
        publish: true,
        architectures: lambdaArchitecture,
        environment: lambdaEnvironment,
        layers: lambdaLayers,
        tags: lambdaTags,
        name: lambdaName,
        description: lambdaDescription,
        role: lambdaRole.arn,
        runtime: lambdaRuntime,
        timeout: lambdaTimeout,
        memorySize: lambdaMemorySize,
        code,
        handler: lambdaHandler,
      },
      {
        provider: usEast1Provider,
        parent: this,
        dependsOn: [bucket, lambdaRolePolicy],
      }
    );

    const cloudfrontDistribution = new aws.cloudfront.Distribution(
      'cloudfront_distribution',
      {
        aliases: cloudfrontAliases,
        enabled: cloudfrontEnabled,
        httpVersion: cloudfrontHttpVersion,
        comment: cloudfrontComment,
        tags: cloudfrontTags,
        waitForDeployment: cloudfrontWaitForDeployment,
        origins: [
          {
            domainName: bucket.bucketRegionalDomainName,
            originId: bucket.id,
            s3OriginConfig: {
              originAccessIdentity: originAccessIdentity.cloudfrontAccessIdentityPath,
            },
          },
        ],
        restrictions: {
          geoRestriction: {
            restrictionType: 'none',
          },
        },
        defaultCacheBehavior: {
          allowedMethods: ['GET', 'HEAD', 'OPTIONS'],
          cachedMethods: ['GET', 'HEAD'],
          compress: true,
          defaultTtl: 0,
          maxTtl: 31536000,
          minTtl: 0,
          targetOriginId: bucket.id,
          viewerProtocolPolicy: 'allow-all',
          forwardedValues: {
            queryString: true,
            cookies: {
              forward: 'none',
            },
          },
          lambdaFunctionAssociations: [
            {
              eventType: 'origin-request',
              lambdaArn: lambdaOriginRequest.qualifiedArn,
            },
          ],
        },
        viewerCertificate: cloudfrontViewerCertificate,
      },
      {
        provider: usEast1Provider,
        parent: this,
      }
    );

    this.bucketId = bucket.id;
    this.bucketName = bucket.bucket;
    this.bucketDomainName = bucket.bucketDomainName;
    this.lambdaArn = lambdaOriginRequest.qualifiedArn;
    this.cloudfrontDistributionId = cloudfrontDistribution.id;
    this.cloudfrontDistributionArn = cloudfrontDistribution.arn;
    this.cloudfrontDistributionDomainName = cloudfrontDistribution.domainName;
    this.registerOutputs({
      bucketId: this.bucketId,
      bucketName: this.bucketName,
      bucketDomainName: this.bucketDomainName,
      lambdaArn: this.lambdaArn,
      cloudfrontDistributionId: this.cloudfrontDistributionId,
      cloudfrontDistributionArn: this.cloudfrontDistributionArn,
      cloudfrontDistributionDomainName: this.cloudfrontDistributionDomainName,
    });
  }
}

export const WvbRemoteProvider = WebviewBundleRemoteProvider;
