<script>
 import { invoke } from '@tauri-apps/api/tauri';
 import { onMount } from 'svelte';
 import Amount from './Amount.svelte';
 import Transfer from './Transfer.svelte';

 export let sidechain;
 let withdraw_amount;
 let deposit_amount;
 let withdrawals = [];
 let refundable = 0;

 async function updateWithdrawals() {
     refundable = await invoke(sidechain[0], {
         method: 'getrefund',
         params: [],
     });
 }

 async function refundAll() {
     const address = await invoke('zcash', {
         method: 'getnewaddress',
         params: [],
     });
     const main_fee = 0.0001;
     await invoke(sidechain[0], {
         method: 'refund',
         params: [refundable, main_fee, true],
     });
 }

 onMount(async () => {
     setInterval(updateWithdrawals, 1000);
 });

 async function withdraw(amount) {
     const main_fee = 0.0001;
     await invoke(sidechain[0], {
         method: 'withdraw',
         params: [amount, main_fee],
     });
 }

 async function deposit(amount) {
     const address = await invoke(sidechain[0], {
         method: 'getnewaddress',
         params: [true],
     });

     const fee = 0.001;
     await invoke('mainchain', {
         method: 'createsidechaindeposit',
         params: [ sidechain[1], address, amount, fee ],
     });
 }
</script>

<div>
    <h1>{sidechain[0]}</h1>
    <div class="container">
        <div class="item">
            <input type="number" bind:value={deposit_amount} placeholder="Deposit amount">
            <button on:click={() => deposit(deposit_amount)}>Deposit</button>
        </div>
        <div class="item">
            <button on:click={refundAll}>Refund all</button>
            Refundable: <Amount value={refundable} />
        </div>
        <div class="item">
            <input type="number" bind:value={withdraw_amount} placeholder="Withdrawal amount">
            <button on:click={() => withdraw(withdraw_amount)}>Withdraw</button>
        </div>
    </div>
</div>

<style>
 .container {
     display: grid;
     grid-template-columns: 1fr 1fr 1fr;
 }

 .item {
     border: solid;
     margin: 2px;
     padding: 10px;
 }
</style>
