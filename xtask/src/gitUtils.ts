import { type Commit, Repository, RepositoryOpenFlags } from '@napi-rs/simple-git';

export function openRepo(dir: string) {
  return Repository.openExt(dir, RepositoryOpenFlags.NoSearch, []);
}

export interface Tag {
  oid: string;
  ref: string;
  name: string;
}

export function listTags(repo: Repository) {
  const tags: Tag[] = [];
  repo.tagForeach((oid, refBuf) => {
    const ref = refBuf.toString('utf8');
    const name = ref.replace('refs/tags/', '');
    tags.push({ oid, ref, name });
    return true;
  });
  return tags;
}

export function listCommits(repo: Repository, from: string, to?: string) {
  const commits: Commit[] = [];
  const walk = repo.revWalk();
  walk.push(from);
  for (const oid of walk) {
    if (oid === to) {
      break;
    }
    const commit = repo.findCommit(oid);
    if (commit != null) {
      commits.push(commit);
    }
  }
  return commits;
}
