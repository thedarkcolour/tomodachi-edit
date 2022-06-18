<script lang='ts'>
	import SaveFile from './SaveFile.svelte';
	import MiiEdit from './MiiEdit.svelte';
</script>

<main>
	<SaveFile/>
	<div id='island-info' class='info-section'>
		<h1>Island Info</h1>
		<p>Island Name: <span id='island-name'></span></p>
		<p>Money: <span id='money'></span></p>
		<p>Problems Solved: <span id='problems-solved'></span></p>
	</div>

	<MiiEdit/>
	<!-- <div id='mii-info' class='info-section'>
		<h1>Mii Editor</h1>
		<div class='info-options'>
			<label class='mii-info-option'><span>Mii:</span><select id='selected-mii' on:change='{onMiiSelected}'></select></label>
			{#each miiTextFields as {label, fieldId, maxLength}}
				<label><span id='{fieldId}_span'>{label}</span><input maxlength="{maxLength}" id='{fieldId}' type='text' on:change='{onFieldChange(fieldId)}'></label>
			{/each}
			{#each miiFoodFields as {label, fieldId}}
				<label><span id='{fieldId}_span'>{label}</span><select disabled=true class='food-selector' id='{fieldId}'></select></label>
			{/each}
		</div>
		<button id='save-button' disabled=true on:click="{saveMiiChanges}">Save Changes</button>
	</div> -->
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

	h1 {
		color: white;
		font-size: 2em;
		font-weight: 100;
	}
	p {
		color: white;
		font-size: 1.2em;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>