<script>
 import { Router, Route } from 'svelte-routing';
 import Main from './lib/Main.svelte';
 import Ethereum from './lib/Ethereum.svelte';
</script>
<Router>
  <Route component={Main} />
  <Route path="/ethereum">
    <Ethereum />
  </Route>
</Router>
