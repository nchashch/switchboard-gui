<script>
 import { invoke } from '@tauri-apps/api/tauri';
 import { onMount } from 'svelte';
 import { afterUpdate } from 'svelte';
 import { Command } from '@tauri-apps/api/shell'
 import { appWindow } from "@tauri-apps/api/window";
 import { join } from '@tauri-apps/api/path';

 let console_child = null;

 let history = [];
 let command = 'eth.blockNumber';
 let index = -1;

 let history_element;

 async function enter(e) {
     if (e.code === 'ArrowUp') {
         command = history.filter(log => log.input !== '').at(index).input;
         index -= 1;
     }
     if (e.code === 'ArrowDown' && index < -1) {
         index += 1;
         command = history.filter(log => log.input !== '').at(index).input;
     }
     if (command.length > 0 && e.code === 'Enter') {
         index = -1;
         await console_child.write(command + '\n');
         history = [...history, {
             input: command,
             output: '',
         }];
         command = '';
     }
 }

 const scrollToBottom = async (node) => {
     node.scroll({ top: node.scrollHeight });
 };

 afterUpdate(() => {
     if(history_element) scrollToBottom(history_element);
 });

 let config = null;

 onMount(async () => {
     config = await invoke('get_config');
     await launch_geth_console();
     appWindow.onCloseRequested(async (event) => {
         await console_child.kill();
     });
 });

 async function launch_geth_console() {
     const args = [
         'attach',
         await join(config.datadir, 'data/ethereum/geth.ipc')
     ]
     const console = Command.sidecar('binaries/geth', args);
     console.stdout.on('data', line => {
         if (line !== '> ') {
             history = [...history, {
                 input: '',
                 output: line,
             }];
         }
     });

     console.stderr.on('data', line => {
         history = [...history, {
             input: '',
             output: line,
         }];
     });

     console_child = await console.spawn();
     await new Promise(r => setTimeout(r, 100));
 }
 let output = [];

</script>
<div class="console">
    <h1>Geth Console</h1>
    <div bind:this={history_element} class="console-history">
        {#each history as command}
            {#if command.input}
                <div class="console-input-log">
                    {command.input}
                </div>
            {/if}
            {#if command.output}
                <div class="console-output-log">
                    <pre>{command.output}</pre>
                </div>
            {/if}
        {/each}
    </div>
    <input bind:value={command} on:keydown={enter} class="console-input" type="text" spellcheck="false">
</div>
<style>
 .console {
     border: solid;
     margin: 2px;
     display: flex;
     flex-direction: column;
 }

 .console-history {
     flex-grow: 1;
     box-sizing: border-box;
     padding: 4px;
     margin: 2px;
     overflow-y: auto;
 }

 .console-input-log,
 .console-output-log {
     box-sizing: border-box;
 }

 pre {
     margin: 0;
     padding: 0;
 }

 .console-input-log:hover,
 .console-output-log:hover {
     background: #eee;
 }

 .console-input-log {
     font-weight: bold;
 }

 .console-input {
     flex-shrink: 0;
     box-sizing: border-box;
     padding: 12px;
     border: none;
     border-top: 1px solid;
     outline: none;
     background-color: #eee;
 }

 .console-input:hover,.console-input:focus {
     background-color: #ddd;
 }

 .console-history, .console-input {
     font-family: monospace;
 }

 input {
     width: 1fr;
     box-sizing: border-box;
 }
 .console {
     height: 400px;
 }
</style>
