import App from './App.svelte';
import { mount } from 'svelte';

const target = document.getElementById('app')!;

mount(App, { target });
