export type LambdaRuntime = 'nodejs18.x' | 'nodejs20.x' | 'nodejs22.x';

export function getLambdaRuntimeTarget(runtime: LambdaRuntime): string {
  switch (runtime) {
    case 'nodejs18.x':
      return 'node18';
    case 'nodejs20.x':
      return 'node20';
    case 'nodejs22.x':
      return 'node22';
  }
}
