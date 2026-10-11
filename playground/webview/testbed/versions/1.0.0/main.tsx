import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { App } from '../../src/App.tsx';

const root = createRoot(document.getElementById('root')!);
root.render(
  <StrictMode>
    <App version="1.0.0" />
  </StrictMode>
);
