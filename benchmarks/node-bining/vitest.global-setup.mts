import { buildAllFixtures } from 'benchmark-tools';

export default async function setup() {
  await buildAllFixtures();
}
