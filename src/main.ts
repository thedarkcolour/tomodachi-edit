import App from './App.svelte';

export interface Island {
	name: string,
	name_pronunciation: string,
	money: number,
	problems_solved: number,
	weddings: number,
	children_born: number,
	travellers_received: number,
	streetpass_encounters: number,
	travellers_sent: number,
	event_fountain: number,
	apartment_stage: number,
	last_save_date: number,
	island_address: string,
	food_items: Array<number>,
	miis: Array<Mii>,
}

export interface Mii {
	sharing_enabled: boolean,
	copying_enabled: boolean,
	first_name: string,
	first_name_pronunciation: string,
	last_name: string,
	last_name_pronunciation: string,
	nickname: string,
	nickname_pronunciation: string,
	relationships: Array<Relationship>,
	pampered_rating: number,
	economy_rating: number,
	emotions: number,
	relation_to_you: number,
	all_time_favorite: number,
	super_all_time_favorite: number,
	worst: number,
	worst_ever: number,
}

export interface Relationship {
	value: number,
	relation: string,
}

export interface Food {
	name: string,
	favorite_id: number,
	inventory_id: number, // not in Rust, init in App.svelte
}

export class TwoWayMap<K, V> {
	map: Map<K, V>;
	reverse: Map<V, K>;

    constructor(map: Map<K, V>) {
        this.map = map;
        this.reverse = new Map<V, K>();

        map.forEach((value, key) => {
			this.reverse.set(value as V, key as K);
		});
    }
}

// Init in App.svelte
export const foodsByFavoriteId = new Map<number, Food>();

// init last, after stores have been initialized
const app = new App({
	target: document.body,
});
export default app;
