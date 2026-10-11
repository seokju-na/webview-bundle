import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { App } from '../../src/App.tsx';

const root = createRoot(document.getElementById('root')!);
root.render(
  <StrictMode>
    <App version="2.0.0" />
  </StrictMode>
);
