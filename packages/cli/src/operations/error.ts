export class OperationError extends Error {
  readonly name = 'OperationError';
}

export function isOperationError(e: unknown): e is OperationError {
  return (
    e instanceof OperationError ||
    (e != null && typeof e === 'object' && (e as OperationError)?.name === 'OperationError')
  );
}
