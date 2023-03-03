<script>
 import DepositsWithdrawals from './DepositsWithdrawals.svelte';
 import { onMount } from 'svelte';
 import { invoke } from '@tauri-apps/api/tauri';
 import { writeText } from '@tauri-apps/api/clipboard';
 import { Command } from '@tauri-apps/api/shell';
 import { join } from '@tauri-apps/api/path';
 import { WebviewWindow } from '@tauri-apps/api/window';

 async function generate(amount) {
     if (testchain_running) {
         await invoke('testchain', {
             method: 'refreshbmm',
             params: [0.0001],
         });
     }
     if (bitassets_running) {

         await invoke('bitassets', {
             method: 'refreshbmm',
             params: [0.0001],
         });
     }
     await invoke('mainchain', { method: 'generate', params: [1] });
 }


 async function launch_mainchain() {
     const args = [
         `-regtest=${config.config.switchboard.regtest ? 1 : 0}`,
         `-datadir=${await join(config.datadir, '/data/main')}`,
         `-rpcport=${config.config.main.port}`,
         `-rpcuser=${config.config.switchboard.rpcuser}`,
         `-rpcpassword=${config.config.switchboard.rpcpassword}`,
         '-server=1'
     ];
     const mainchain = Command.sidecar('binaries/drivechain-qt', args);
     console.log(mainchain);
     await mainchain.spawn();
     let block_count = null;
     for (let i = 0; i < 100; ++i) {
         await invoke('mainchain', {
             method: 'getblockcount',
             params: [],
         }).then(count => block_count = count)
           .catch(() => {});
         console.log(block_count);
         if (block_count !== null) {
             break;
         }
         await new Promise(r => setTimeout(r, 1000));
     }
     if (block_count < 200) {
         await invoke('mainchain', {
             method: 'createsidechainproposal',
             params: [0, 'testchain'],
         });
         await invoke('mainchain', {
             method: 'createsidechainproposal',
             params: [4, 'testchain'],
         });
         await invoke('mainchain', {
             method: 'createsidechainproposal',
             params: [5, 'zcash'],
         });
         await invoke('mainchain', {
             method: 'createsidechainproposal',
             params: [6, 'ethereum'],
         });
         await invoke('mainchain', {
             method: 'generate',
             params: [200],
         });
     }
 }

 async function launch_testchain() {
     const args = [
         `-regtest=${config.config.switchboard.regtest ? 1 : 0}`,
         `-datadir=${await join(config.datadir, '/data/testchain')}`,
         `-rpcport=${config.config.testchain.port}`,
         `-rpcuser=${config.config.switchboard.rpcuser}`,
         `-rpcpassword=${config.config.switchboard.rpcpassword}`,
         '-server=1'
     ];
     const testchain = Command.sidecar('binaries/testchain-qt', args);
     await testchain.spawn();
 }

 async function launch_bitassets() {
     const args = [
         `-regtest=${config.config.switchboard.regtest ? 1 : 0}`,
         `-datadir=${await join(config.datadir, 'data/bitassets')}`,
         `-rpcport=${config.config.bitassets.port}`,
         `-rpcuser=${config.config.switchboard.rpcuser}`,
         `-rpcpassword=${config.config.switchboard.rpcpassword}`,
         '-server=1'
     ];
     const bitassets = Command.sidecar('binaries/bitassets-qt', args);
     await bitassets.spawn();
 }

 async function launch_ethereum() {
     const args = [
         `--datadir=${await join(config.datadir, 'data/ethereum')}`,
         `--http.port=${config.config.ethereum.port}`,
         `--main.port=${config.config.main.port}`,
         `--verbosity=${config.config.ethereum.verbose ? 3 : 0}`,
         '--http',
         '--http.api=eth,web3,personal,net',
         '--maxpeers=0',
         '--dev',
     ];
     const ethereum = Command.sidecar('binaries/geth', args);
     ethereum_child = await ethereum.spawn();
     await new Promise(r => setTimeout(r, 200));
     ethereum_webview = new WebviewWindow('ethereumWindow', {
         url: '/ethereum',
     });
     ethereum_webview.onCloseRequested(async (event) => {
         await ethereum_child.kill();
     });
 }


 let ethereum_child = null;
 let ethereum_webview = null;

 let mainchain_running = false;
 let testchain_running = false;
 let bitassets_running = false;
 let ethereum_running = false;

 async function update() {
     invoke('mainchain', {
         method: 'getblockcount',
         params: [],
     }).then(() => { mainchain_running = true; })
       .catch(() => { mainchain_running = false; });
     invoke('testchain', {
         method: 'getblockcount',
         params: [],
     }).then(() => { testchain_running = true; })
       .catch(() => { testchain_running = false; });
     invoke('bitassets', {
         method: 'getblockcount',
         params: [],
     }).then(() => { bitassets_running = true; })
       .catch(() => { bitassets_running = false; });
     invoke('web3', {
         method: 'eth_blockNumber',
         params: [],
     }).then(() => { ethereum_running = true; })
       .catch(() => { ethereum_running = false; });
 }

 let config = null;

 onMount(async () => {
     config = await invoke('get_config');
     setInterval(update, 1000);
 });

 async function copyToClipboard() {
     await writeText(geth_console);
 }
</script>
<div>
    <ul>
        <li>
            {#if !mainchain_running}
                <button on:click={launch_mainchain}>Launch Mainchain Qt</button>
            {:else}
                Mainchain node running...
            {/if}
        </li>
        {#if mainchain_running}
        <li>
            {#if !testchain_running}
                <button on:click={launch_testchain}>Launch Testchain Qt</button>
            {:else}
                Testchain node running...
            {/if}
        </li>
        <li>
            {#if !bitassets_running}
                <button on:click={launch_bitassets}>Launch Bitassets Qt</button>
            {:else}
                BitAssets node running...
            {/if}
        </li>
        <li>
            {#if !ethereum_running}
                <button on:click={launch_ethereum}>Launch Ethereum</button>
            {:else}
                Ethereum node running...
            {/if}
        </li>
        {/if}
    </ul>
</div>
{#if mainchain_running}
<div class="mining">
    <h1> Mining </h1>
    <button on:click={() => generate(1000)}>Mine block</button>
</div>
{/if}
{#if testchain_running}
<DepositsWithdrawals sidechain={['testchain', 0]} />
{/if}
{#if bitassets_running}
<DepositsWithdrawals sidechain={['bitassets', 4]} />
{/if}

<style>
 .mining {
     width: 1fr;
     border: solid;
     margin: 2px;
 }
 a {
     border: solid;
     display: block;
     margin: 2px;
     padding: 10px;
     text-align: center;
     background-color: #eee;
     color: #000;
     text-decoration: none;
 }
</style>
