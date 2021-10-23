<script type="ts">
  import { currentUser, UserInfo } from "../stores";
  import { onMount } from "svelte";

  import Header from "$lib/Header.svelte";
  import Footer from "$lib/Footer.svelte";

  async function getUserFromServer(): Promise<UserInfo | null> {
    let res = await fetch("/api/users/current");
    if (res.ok) {
      let json = await res.json();
      return <UserInfo>json;
    } else {
      return null;
    }
  }

  $: onMount(async () => {
    currentUser.set(await getUserFromServer());
  });
</script>

<article>
  <Header />

  <main>
    <slot />
  </main>

  <Footer />
</article>

<style>
  /* Basic Styles */
  :global(*) {
    margin: 0;
    padding: 0;
    font-family: Arial, Helvetica, sans-serif;
  }

  :global(h1) {
    margin-bottom: 0.3em;
  }

  :global(h2) {
    margin-top: 0.8em;
    margin-bottom: 0.3em;
  }

  :global(h3) {
    margin-top: 1em;
  }

  /* Padding */
  :global(#content) {
    margin: 2em;
    flex: 1 0 auto;
  }

  /* Footer at Bottom */
  :global(html, body) {
    height: 100%;
    width: 100%;
  }

  :global(#svelte) {
    height: 100%;
  }

  article {
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: stretch;
  }

  main {
    flex-grow: 1;
    flex-shrink: 0;
  }
</style>
