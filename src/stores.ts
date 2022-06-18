import { writable } from 'svelte/store';
import type { Island } from './main';

export const islandData = writable<Island>();
export const foodRegistry = writable<Array<string>>();