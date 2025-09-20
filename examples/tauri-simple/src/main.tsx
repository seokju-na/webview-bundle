import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App.js';

console.log('Hello World!');

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
