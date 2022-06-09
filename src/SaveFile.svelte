<script lang="ts">
    import { invoke } from '@tauri-apps/api';
    import { listen } from '@tauri-apps/api/event';

    listen('file-chosen', (event) => {
        (document.getElementById('chosen-file') as HTMLInputElement).value = event.payload as string;
        onFileChanged()
    });

    function openFileChooser() {
        invoke('open_file_chooser');
    }

    function onFileChanged() {
        let newValue: string = (document.getElementById('chosen-file') as HTMLInputElement).value

        invoke('check_file_target', { path: newValue }).then((response: number) => {
            let status = document.getElementById('status');

            if (response == 1) {
                status.textContent = 'File size does not match expected 1985688 bytes size';
                status.classList.add('invalid-file')
            } else if (response == 2) {
                status.textContent = 'File name does not match expected "savedataArc.txt" name';
                status.classList.add('invalid-file')
            } else if (response == 0) {
                status.textContent = 'Save file is valid'
                status.classList.remove('invalid-file')
            }
        })
    }
</script>

<main>
    <div>
        <label>
            <button on:click="{openFileChooser}">
                Open Save Data
            </button>
            <input id='chosen-file' on:change="{onFileChanged}" type="text" value="Choose a file...">
        </label>
        <p id="status" >File not chosen...</p>
    </div>
</main>

<style>
    div {
        column-gap: 20px;
    }
    input {
        width: 400px;
        color: #666666;
    }
    button {
        border-radius: 0;
        cursor: pointer;
        border: 2px #999999;
        color: #666666;
    }

    /* Global prevents Svelte from removing the unused css */
    :global(.invalid-file) {
        color: red !important;
    }
</style>