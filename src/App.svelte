<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import SaveFile from './SaveFile.svelte';

	//const webview = new WebviewWindow('window');
	//webview.show();
</script>

<main>
	<SaveFile/>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>