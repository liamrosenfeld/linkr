<script>
  let newUser = {
    username: "",
    password: "",
    manage_links: true,
    manage_users: true,
  };

  let result = "";
  let creating = false;

  async function createNewUser() {
    creating = true;
    const res = await fetch("/api/users/new", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(newUser),
    });
    creating = false;
    if (res.ok) {
      // TODO: set a global success snackbar
      window.location.pathname = "/";
    } else {
      result = await res.text();
    }
  }
</script>

<div id="content">
  <h1>Setup Linkr</h1>
  <form on:submit|preventDefault={createNewUser}>
    <label for="username">username</label>
    <input
      type="text"
      autocomplete="username"
      name="username"
      bind:value={newUser.username}
    /><br />
    <label for="password">password</label>
    <input
      type="password"
      autocomplete="new-password"
      name="password"
      bind:value={newUser.password}
    /><br />

    <input type="submit" disabled={creating} value="Create Account" />
  </form>
  <p>{result}</p>
</div>
