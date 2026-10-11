import { type BundleUpdate, updater } from '@wvb/bridge';
import { useEffect, useState } from 'react';

type Status =
  | 'checking'
  | 'available'
  | 'up-to-date'
  | 'downloading'
  | 'installing'
  | 'reloading'
  | 'error';

export function App({ version }: { version: string }) {
  const [update, setUpdate] = useState<BundleUpdate | null>(null);
  const [status, setStatus] = useState<Status>('checking');
  const [message, setMessage] = useState('Checking for updates…');

  useEffect(() => {
    let cancelled = false;
    updater
      .getUpdate()
      .then(result => {
        if (cancelled) return;
        const next = result?.bundles.find(bundle => bundle.name === 'testbed') ?? null;
        setUpdate(next);
        setStatus(next ? 'available' : 'up-to-date');
        setMessage(next ? `Update available: ${next.version}` : 'No updates available.');
      })
      .catch(error => {
        if (cancelled) return;
        setStatus('error');
        setMessage(`Update check failed: ${String(error)}`);
      });
    return () => {
      cancelled = true;
    };
  }, []);

  async function installUpdate() {
    if (update == null || status !== 'available') return;
    try {
      setStatus('downloading');
      setMessage(`Downloading ${update.version}…`);
      const downloads = await updater.download([update]);
      const downloaded = downloads.find(
        bundle => bundle.name === update.name && bundle.version === update.version
      );
      if (downloaded?.result.type !== 'downloaded') {
        throw new Error(`Download failed: ${JSON.stringify(downloaded?.result)}`);
      }
      setStatus('installing');
      setMessage(`Installing ${update.version}…`);
      const installs = await updater.install([{ name: update.name, version: update.version }]);
      const installed = installs.find(bundle => bundle.name === update.name);
      if (installed?.result.type !== 'installed' || installed.installVersion !== update.version) {
        throw new Error(`Install failed: ${JSON.stringify(installed)}`);
      }
      setStatus('reloading');
      setMessage(`Installed ${update.version}. Reloading…`);
      window.location.reload();
    } catch (error) {
      setStatus('error');
      setMessage(`Update failed: ${String(error)}`);
    }
  }

  return (
    <main>
      <h1>TestBed</h1>
      <h2 data-testid="version">{version}</h2>
      <p role="status" data-testid="update-status" data-state={status}>
        {message}
      </p>
      <button
        type="button"
        data-testid="update-button"
        disabled={status !== 'available'}
        onClick={installUpdate}
      >
        {update ? `Update to ${update.version}` : 'Update'}
      </button>
    </main>
  );
}
