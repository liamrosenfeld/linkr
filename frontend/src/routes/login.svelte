<script lang="ts">
  import { currentUser } from "../stores";
  let username = "";
  let password = "";
  let result = "";
  let loggingIn = false;

  async function login() {
    loggingIn = true;
    const res = await fetch("/api/users/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        username,
        password,
      }),
    });
    if (res.ok) {
      let user_info = await res.json();
      currentUser.set(user_info);
      window.location.pathname = "/";
    } else {
      loggingIn = false;
      result = await res.text();
    }
  }
</script>

<div id="content">
  <h1>Login</h1>

  <form on:submit|preventDefault={login}>
    <label for="username">username</label>
    <input
      type="text"
      autocomplete="username"
      name="username"
      id="username"
      bind:value={username}
    /><br />
    <label for="password">password</label>
    <input
      type="password"
      autocomplete="current-password"
      name="password"
      id="password"
      bind:value={password}
    /><br />
    <input type="submit" disabled={loggingIn} value="Login" />
  </form>

  <p>{result}</p>
</div>
