import { buildAllFixtures } from '@benchmark/tools';

export default async function setup(): Promise<void> {
  await buildAllFixtures();
}
