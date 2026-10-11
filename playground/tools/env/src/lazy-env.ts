import z from 'zod';

// Object schemas describe namespaces; validate individual values only when read.
export function lazyEnv<T extends z.ZodObject>(
  schema: T,
  raw: Record<string, unknown>,
  path: string[] = []
): z.output<T> {
  const result = {};

  for (const [key, field] of Object.entries(schema.shape as Record<string, z.ZodType>)) {
    Object.defineProperty(result, key, {
      enumerable: true,
      get() {
        const value = raw[key];
        const fieldPath = [...path, key];

        if (field instanceof z.ZodObject) {
          return lazyEnv(field, (value ?? {}) as Record<string, unknown>, fieldPath);
        }

        const parsed = field.safeParse(value);
        if (!parsed.success) {
          throw new z.ZodError(
            parsed.error.issues.map(issue => ({
              ...issue,
              path: [...fieldPath, ...issue.path],
            }))
          );
        }
        return parsed.data;
      },
    });
  }

  return result as z.output<T>;
}
