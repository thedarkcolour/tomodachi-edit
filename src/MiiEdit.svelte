<script lang='ts'>
    import { invoke } from '@tauri-apps/api';
	import { foodsByFavoriteId } from './main';
	import { islandData, foodRegistry } from './stores';

	let miiTextFields = [
		{label: 'First Name:', fieldId: 'first_name', maxLength: 15},
		{label: 'Last Name:', fieldId: 'last_name', maxLength: 15},
		{label: 'Nickname:', fieldId: 'nickname', maxLength: 10},
		{label: 'First Name Pronunciation:', fieldId: 'first_name_pronunciation', maxLength: 30},
		{label: 'Last Name Pronunciation:', fieldId: 'last_name_pronunciation', maxLength: 30},
		{label: 'Nickname Pronunciation:', fieldId: 'nickname_pronunciation', maxLength: 20},
	];
	let miiFoodFields = [
		{label: 'Super All-Time Favorite: ', fieldId: 'super_all_time_favorite'},
		{label: 'All-Time Favorite: ', fieldId: 'all_time_favorite'},
		{label: 'Worst: ', fieldId: 'worst'},
		{label: 'Worst Ever: ', fieldId: 'worst_ever'},
	]

	export function onFoodsLoaded() {
		const selectors = document.getElementsByClassName('food-selector');			
			for (let i = 0; i < selectors.length; i++) {
				const selector = selectors[i];

				$foodRegistry.forEach((food) => {
					const option = document.createElement('option');
					option.value = food.name;
					option.innerHTML = food.name;
					selector.append(option);
				});
			}
	}

	function onMiiSelected() {
		let selectedMiiElement = document.getElementById('selected-mii') as HTMLSelectElement;

		if (selectedMiiElement.options.length > 0) {
			let selectedMiiIndex = selectedMiiElement.selectedIndex;
			let selectedMii = $islandData.miis[selectedMiiIndex];
			console.log(selectedMii);

			miiTextFields.forEach(field => {
				(document.getElementById(field.fieldId) as HTMLInputElement).value = selectedMii[field.fieldId];
			});
			miiFoodFields.forEach(field => {
				const element = (document.getElementById(field.fieldId) as HTMLSelectElement);
				element.disabled = false;
				// inventory does not inclue "Nothing" but selectors do
				element.selectedIndex = foodsByFavoriteId.get(selectedMii[field.fieldId]).inventory_id + 1;
			});
		}
	}

	function onFieldChange(fieldId: string) {
		return function() {
			let mii = $islandData.miis[(document.getElementById('selected-mii') as HTMLSelectElement).selectedIndex];
			let initialValue = mii[fieldId];

			if (initialValue.length > 0) {
				let value = (document.getElementById(fieldId) as HTMLInputElement).value;
				let span = (document.getElementById(fieldId + '_span'))

				if (initialValue == value) {
					span.classList.remove('field-changed');

					if (document.getElementsByClassName('field-changed').length < 1) {
						(document.getElementById('save-button') as HTMLButtonElement).disabled = true;
					}
				} else {
					span.classList.add('field-changed');
					(document.getElementById('save-button') as HTMLButtonElement).disabled = false;
				}
			} 
		}
	}

	function saveMiiChanges() {
		let changedFields = document.getElementsByTagName('field-changed');
		let changeList = [];

		for (let i = 0; i < changedFields.length; i++) {
			let label = changedFields[i];
			let field = document.getElementById(label.id.replace('_span', ''));

			changeList.push({
				// TODO
			});
		}

		invoke('save_mii_changes', {
			save_file_path: (document.getElementById('chosen-file') as HTMLInputElement).value,
			mii_index: (document.getElementById('selected-mii') as HTMLSelectElement).selectedIndex,
			changes: changeList,
		}).then(() => {});
	}
</script>

<main>
	<div id='mii-info' class='info-section'>
		<h1>Mii Editor</h1>
		<div class='info-options'>
			<label class='mii-info-option'><span>Mii:</span><select id='selected-mii' on:change='{onMiiSelected}'></select></label>
			{#each miiTextFields as {label, fieldId, maxLength}}
				<label><span id='{fieldId}_span'>{label}</span><input maxlength="{maxLength}" id='{fieldId}' type='text' on:change='{onFieldChange(fieldId)}'></label>
			{/each}
			<details>
				<summary>Favorite/Least Favorite Foods</summary>
				{#each miiFoodFields as {label, fieldId}}
					<label><span id='{fieldId}_span'>{label}</span><select disabled=true class='food-selector' id='{fieldId}'></select></label>
				{/each}
			</details>
		</div>
		<button id='save-button' disabled=true on:click="{saveMiiChanges}">Save Changes</button>
	</div>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0;
	}

	select {
		width: 130px;
		height: 1.4em;
		padding: 0;
	}

	.info-options {
		max-width: 400px;
		margin-left: auto;
		margin-right: auto;
		overflow: auto;
	}

	.info-section {
		background: #666666;
		margin-left: auto;
		margin-right: auto;
		border-radius: 20px;
		max-width: 600px;
	}
	.mii-info-option {
		align-content: center;
		min-width: 0px;
	}

	:global(.field-changed) {
		font-style: italic;
	}

	label {
		display: flex;
		float: right;
		color: white;
		justify-content: right;
		width: 600px;
	}

	h1 {
		color: white;
		font-size: 2em;
		font-weight: 100;
	}

	label > span {
		padding-right: 5px;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>