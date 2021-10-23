<script lang="ts">
  let defaultUser = {
    username: "",
    password: "",
    manage_links: false,
    manage_users: false,
  };
  let newUser = {
    username: "",
    password: "",
    manage_links: false,
    manage_users: false,
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
      result = `"${newUser.username}" created!`;
      newUser = defaultUser;
    } else {
      result = await res.text();
    }
  }
</script>

<div id="content">
  <h2>Create New User</h2>
  <p>Create an account for someone else on your team</p>
  <a href="/manage_users">Back</a>

  <div>
    <h3>Credentials</h3>
    <form on:submit|preventDefault={createNewUser}>
      <label for="username">username</label>
      <input
        type="text"
        autocomplete="username"
        name="username"
        id="username"
        bind:value={newUser.username}
      /><br />
      <label for="password">password</label>
      <input
        type="password"
        autocomplete="new-password"
        name="password"
        id="password"
        bind:value={newUser.password}
      />
      <p><em>They should update their password on login</em></p>

      <h3>Permissions</h3>
      <input
        type="checkbox"
        id="manage_links"
        name="manage_links"
        bind:checked={newUser.manage_links}
      />
      <label for="manage_links">Manage All Links</label><br />

      <input
        type="checkbox"
        id="manage_users"
        name="manage_users"
        bind:checked={newUser.manage_users}
      />
      <label for="manage_users">Manage All Users</label><br />

      <input type="submit" disabled={creating} value="Create Account" />
    </form>
  </div>

  <p>{result}</p>
</div>
