<script lang='ts'>
    import { invoke } from '@tauri-apps/api';
	import SaveFile from './SaveFile.svelte';	
	import { Food, foodsByFavoriteId } from './main';
	import MiiEdit from './MiiEdit.svelte';
	import { foodRegistry, islandData } from './stores';

	let miiEdit: MiiEdit;

	document.addEventListener('DOMContentLoaded', (event) => {
		invoke('get_food_registry').then((registry: Array<Food>) => {
			foodRegistry.set(registry);

			foodsByFavoriteId.clear();
			const foodInvSelector = document.getElementById('food-inventory-item');
			registry.forEach((food, i) => {
				// assign inventory_id and put foods into favorite map
				food.inventory_id = i - 1;
				foodsByFavoriteId.set(food.favorite_id, food);
				
				// add options to selector
				const option = document.createElement('option');
				option.value = food.name;
				option.innerHTML = food.name;
				foodInvSelector.append(option);
			});

			console.log(registry);

			miiEdit.onFoodsLoaded();
		});
	});

	export function onFoodSelected() {
		const index = (document.getElementById('food-inventory-item') as HTMLSelectElement).selectedIndex;
		//const food = $foodRegistry[index];
		const count = document.getElementById('food-inventory-amount') as HTMLInputElement;
		const discovered = document.getElementById('food-discovered') as HTMLInputElement;
		
		if (index == 0) {
			count.disabled = true;
		} else {
			count.disabled = false;
			const foodAmount = $islandData.food_items[index - 1];

			if (foodAmount == 253) {
				discovered.checked = false;
				count.value = '0';
			} else {
				discovered.checked = true;
				count.value = foodAmount.toString();
			}
		}
	}
</script>

<main>
	<SaveFile/>
	<div id='island-info' class='info-section'>
		<h1>Island Info</h1>
		<p>Island Name: <span id='island-name'></span></p>
		<p>Money: <span id='money'></span></p>
		<p>Problems Solved: <span id='problems-solved'></span></p>
		
		<p id='food-inventory-header'>Foods inventory editor</p>
		<div id='food-inventory-options'>
			<select id='food-inventory-item' on:change='{onFoodSelected}'></select>
			<input id='food-inventory-amount' type='number' min='0' max='99'>
			<span>Discovered</span>
			<input id='food-discovered' type='checkbox'>
		</div>
	</div>

	<MiiEdit bind:this={miiEdit}/>
</main>

<style>
	/*:root {
	//	background-color: #222222;
	//}*/
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0;
	}

	.info-section {
		background: #666666;
		margin-left: auto;
		margin-right: auto;
		border-radius: 20px;
		max-width: 600px;
	}

	:global(.field-changed) {
		font-style: italic;
	}

	#food-inventory-options {
		display: block;
	}

	h1 {
		color: white;
		font-size: 2em;
		font-weight: 100;
	}
	p, span {
		color: white;
		font-size: 1.2em;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>