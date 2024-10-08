import { type Commit, type GitObject, Repository, RepositoryOpenFlags, Signature } from '@napi-rs/simple-git';

export function openRepo(dir: string) {
  return Repository.openExt(dir, RepositoryOpenFlags.NoSearch, []);
}

export interface Tag {
  oid: string;
  name: string;
  shorthand: string;
}

export function listTags(repo: Repository) {
  const tags: Tag[] = [];
  repo.tagForeach((oid, nameBuf) => {
    const name = nameBuf.toString('utf8');
    const shorthand = name.replace('refs/tags/', '');
    tags.push({ oid, name, shorthand });
    return true;
  });
  return tags;
}

export function addTag(
  repo: Repository,
  name: string,
  target: GitObject,
  signature: Signature,
  message: string,
  force = false
) {
  const shorthand = tagShorthand(name);
  const oid = repo.tag(shorthand, target, signature, message, force);
  const tag: Tag = {
    oid,
    name: tagName(shorthand),
    shorthand,
  };
  return tag;
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

export function tagName(val: string) {
  return `refs/tags/${tagShorthand(val)}`;
}

export function tagShorthand(val: string) {
  return val.startsWith('refs/tags/') ? val.replace('refs/tags/', '') : val;
}

export function signature(name: string, email: string) {
  return Signature.now(name, email);
}

export function defaultSignature() {
  return signature('Seokju Na', 'seokju.me@gmail.com');
}

export function commitToHead(repo: Repository, signature: Signature, message: string) {
  const headRef = repo.head();
  const oid = repo.commit(null, signature, signature, message, headRef.peelToTree());
  return repo.findCommit(oid)!;
}
