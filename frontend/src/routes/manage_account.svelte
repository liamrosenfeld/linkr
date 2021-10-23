<script lang="ts">
  import { currentUser, UserInfo } from "../stores";
  let user: UserInfo;

  let username = "";
  let currentPassword = "";
  let newPassword = "";
  let destructPassword = "";

  let updatingUsername = false;
  let updatingPassword = false;
  let destructing = false;

  let result = "";

  currentUser.subscribe((value) => {
    user = value;
    if (value != null) {
      username = value.username;
    }
  });

  async function updateUsername() {
    updatingUsername = true;
    const res = await fetch(`/api/users/update/username`, {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        user_id: user.id,
        new_name: username,
      }),
    });
    if (res.ok) {
      user.username = username;
      currentUser.set(user);
      result = "Username updated!";
    } else {
      result = await res.text();
    }

    updatingUsername = false;
  }

  async function updatePassword() {
    updatingPassword = true;

    const res = await fetch(`/api/users/update/password`, {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        current_pw: currentPassword,
        new_pw: newPassword,
      }),
    });
    if (res.ok) {
      currentUser.set(null);
      // should set some sort of global snackbar to say that it worked
      window.location.pathname = "/login";
    } else {
      result = await res.text();
    }

    updatingPassword = false;
  }

  async function disableAccount() {
    if (!confirm("Are you sure you want to disable your account?")) {
      return;
    }
    await destructAccount("disable");
  }

  async function deleteAccount() {
    if (
      !confirm(
        "Are you sure you want to permanently delete your account and all links?"
      )
    ) {
      return;
    }
    await destructAccount("delete");
  }

  async function destructAccount(action: string) {
    destructing = true;
    const body = {
      id: user.id,
      password: destructPassword,
    };
    const res = await fetch(`/api/users/${action}`, {
      method: action == "delete" ? "DELETE" : "PATCH",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(body),
    });
    if (res.ok) {
      currentUser.set(null);
      window.location.pathname = "/login";
    } else {
      result = await res.text();
      destructing = false;
    }
  }
</script>

<div id="content">
  <h1>Manage Account</h1>
  {#if user != null}
    <h2>Username</h2>

    <div>
      <input
        type="text"
        id="username"
        disabled={updatingUsername}
        bind:value={username}
      />
      <button
        type="button"
        disabled={updatingUsername}
        on:click={updateUsername}
      >
        Save
      </button>
    </div>

    <h2>Password</h2>

    <div>
      <label for="current_pw">Current Password:</label>
      <input
        autocomplete="current-password"
        type="password"
        disabled={updatingPassword}
        bind:value={currentPassword}
      /><br />
      <label for="new_pw">New Password:</label>
      <input
        autocomplete="new-password"
        type="password"
        disabled={updatingPassword}
        bind:value={newPassword}
      /><br />
      <button
        type="button"
        disabled={updatingPassword}
        on:click={updatePassword}
      >
        Change
      </button>
    </div>

    <h2>Danger Zone</h2>

    {#if user.orig}
      <p>You cannot delete the original user</p>
    {:else}
      <div>
        <label for="password">Current Password:</label>
        <input
          autocomplete="current-password"
          type="password"
          disabled={destructing}
          bind:value={destructPassword}
        /><br />
        <button type="button" disabled={destructing} on:click={disableAccount}>
          Disable Account
        </button>
      </div>

      <div class="note">
        <p>Disabling preserves links and is able to be undone by an admin.</p>
      </div>

      <button type="button" disabled={destructing} on:click={deleteAccount}>
        Delete Account
      </button>

      <div class="note">
        <p>
          Deleting your account will also delete <strong>all your links</strong
          >. Do not delete your account if links you created are still in use.
        </p>
      </div>
    {/if}

    <p>{result}</p>
  {/if}
</div>

<style>
  .note {
    max-width: 20em;
    padding: 0.5em;
    background-color: #f5f5f5;
    margin: 1em;
  }
</style>
