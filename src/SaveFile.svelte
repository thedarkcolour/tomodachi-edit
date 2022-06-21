<script lang='ts'>
    import { invoke } from '@tauri-apps/api';
    import { listen } from '@tauri-apps/api/event';
    import type { Food, Island } from './main';
    import { islandData, foodRegistry } from './stores';

    invoke('get_food_registry').then((payload: Array<Food>) => {
        foodRegistry.set(payload);
    });

    listen('file-chosen', (event) => {
        (document.getElementById('chosen-file') as HTMLInputElement).value = event.payload as string;
        onFileChanged();
    });
    

    function openFileChooser() {
        invoke('open_file_chooser');
    }

    function onFileChanged() {
        let newValue: string = (document.getElementById('chosen-file') as HTMLInputElement).value;

        invoke('check_file_target', { path: newValue }).then((response: number) => {
            let status = document.getElementById('status');

            if (response == 1) {
                status.textContent = 'File size does not match expected 1985688 bytes size';
                status.classList.add('invalid-file');
            } else if (response == 2) {
                status.textContent = 'File name does not match expected "savedataArc.txt" name';
                status.classList.add('invalid-file');
            } else if (response == 3) {
                status.textContent = 'Could not read file at specified path (does the file exist?)';
                status.classList.add('invalid-file');
            } else if (response == 0) {
                status.textContent = 'Save file is valid';
                status.classList.remove('invalid-file');

                invoke('load_island', { path: newValue }).then((island: Island) => {
                    islandData.set(island);
                    console.log(island);
                    // Thank you: https://stackoverflow.com/a/34897151/10860617
                    (document.getElementById('island-name') as HTMLSpanElement).textContent = island.name.replace(/\0[\s\S]*$/g,'');
                    (document.getElementById('money') as HTMLSpanElement).textContent = formatMoney(island.money);
                    (document.getElementById('problems-solved') as HTMLSpanElement).textContent = island.problems_solved.toString();
                    
                    const miiSelector = (document.getElementById('selected-mii') as HTMLSelectElement);     
                    miiSelector.textContent = '';
                    island.miis.forEach((mii, i) => {
                        if (mii.nickname != '' && !mii.nickname.startsWith('\0')) {
                            const option = document.createElement('option');
                            const optionLabel = `${i + 1}: ${mii.nickname}`;
                            // todo check the difference between these two
                            option.value = mii.nickname;
                            option.innerHTML = optionLabel;
                            miiSelector.appendChild(option);
                        }
                    });
                    // manually fire the event because adding options to an empty select element doesn't fire the event
                    // even though the selected element has changed
                    document.getElementById('food-inventory-item').dispatchEvent(new Event("change"));
                    miiSelector.dispatchEvent(new Event("change"));
                });
            }
        });
    }

    function formatMoney(money: number): string {
        let temp = `$${Math.floor(money / 100)}.${Math.floor((money % 100) / 10)}${money % 10}`;
        console.log(temp + ` (parsed from ${money})`);
        return temp;
    }
</script>

<main>
    <div>
        <label>
            <button on:click='{openFileChooser}'>
                Open Save Data
            </button>
            <input id='chosen-file' on:change='{onFileChanged}' type='text' value='Choose a file...'>
        </label>
        <p id='status'>File not chosen...</p>
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