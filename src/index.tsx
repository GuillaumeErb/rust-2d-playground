import './style.css';
import { createRoot } from 'react-dom/client';
import { App } from './App'
import * as React from 'react';

document.body.innerHTML = '<div id="app"></div>';
const root = createRoot(document.getElementById('app')!);
root.render(<App />);