<script>
 import { Router, Route } from 'svelte-routing';
 import Main from './lib/Main.svelte';
 import Zcash from './lib/Zcash.svelte';
 import Ethereum from './lib/Ethereum.svelte';
</script>
<Router>
  <Route component={Main} />
  <Route path="/ethereum">
    <Ethereum />
  </Route>
  <Route path="/zcash">
    <Zcash />
  </Route>
</Router>
