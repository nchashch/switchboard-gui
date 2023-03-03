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
     const withdrawalIds = await invoke(sidechain[0], {
         method: 'listmywithdrawals',
         params: [],
     });
     withdrawals = await Promise.all(withdrawalIds.map(async id => {
         const withdrawal = await invoke(sidechain[0], {
             method: 'getwithdrawal',
             params: [id.id],
         }).catch(() => {});
         return withdrawal;
     }));
     refundable = withdrawals
         .filter(w => w)
         .filter(w => w.status === 'Unspent')
         .map(w => w.amount)
         .reduce((a, b) => a+b, 0)/100000000;
 }

 async function refundAll() {
     await invoke(sidechain[0], {
         method: 'refundallwithdrawals',
         params: [],
     });
 }

 onMount(async () => {
     setInterval(updateWithdrawals, 1000);
 });

 async function withdraw(amount) {
     const main_address = await invoke('mainchain', {
         method: 'getnewaddress',
         params: ['', 'legacy'],
     });
     const refund_address = await invoke(sidechain[0], {
         method: 'getnewaddress',
         params: ['', 'legacy'],
     });
     const side_fee = 0.001;
     const main_fee = 0.001;
     console.log(main_address, refund_address, amount, side_fee, main_fee);
     await invoke(sidechain[0], {
         method: 'createwithdrawal',
         params: [main_address, refund_address, amount, side_fee, main_fee],
     });
 }

 async function deposit(amount) {
     const address = await invoke(sidechain[0], {
         method: 'getdepositaddress',
         params: [],
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
