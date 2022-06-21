import { writable } from 'svelte/store';
import type { Food, Island } from './main';

export const islandData = writable<Island>();
export const foodRegistry = writable<Array<Food>>();