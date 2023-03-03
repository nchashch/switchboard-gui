<script>
 export let address;
 export let amount;
 export let transfer;
</script>

<form class="container">
    <input type="text" bind:value={address} placeholder="address">
    <input type="number" bind:value={amount} placeholder="amount">
    <button class="transfer" on:click={transfer}>
        Transfer
    </button>
</form>

<style>
 .container {
     display: grid;
     grid-template-rows: 1fr 1fr 1fr;
 }
</style>
