<script>
  /* ----- Setup ----- */
  import { onMount } from "svelte";
  let columnNames = [""];
  let users = [];

  onMount(async () => {
    columnNames = ["id", "username"];
    const res = await fetch("/api/users/all");
    users = await res.json();
    sort(undefined);
  });

  /* ----- Sorting ----- */
  let sortBy = { col: "id", ascending: true };

  function sort(column) {
    if (column != undefined) {
      if (sortBy.col == column) {
        // if the same column, switch direction
        sortBy.ascending = !sortBy.ascending;
      } else {
        // change to new column
        sortBy.col = column;
        sortBy.ascending = true;
      }
    } else {
      column = sortBy.col;
    }

    // Change sort function dependant on ascending or decending
    let sortModifier = sortBy.ascending ? 1 : -1;
    let sortFunc = (a, b) =>
      a[column] < b[column]
        ? -1 * sortModifier
        : a[column] > b[column]
        ? 1 * sortModifier
        : 0;

    // sort
    users = users.sort(sortFunc);
  }

  /* ----- Actions ----- */
  let result = "";

  async function deleteUser(id, name) {
    if (!confirm(`Delete user "${name}" and all their links?`)) {
      return;
    }

    const res = await fetch("/api/users/delete", {
      method: "DELETE",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        id,
      }),
    });

    switch (res.status) {
      case 200:
        result = "User deleted!";
        users.splice(
          users.findIndex((user) => user.id == id),
          1
        );
        users = users;
        break;
      case 404:
        result = "That user does not exist. (Code: 404)";
        break;
      case 500:
        result = "There was an internal server error. (Code: 500)";
        break;
      default:
        result = `An error has occurred. (Code: ${res.status}).`;
        break;
    }
  }

  async function enableUser(id, name) {
    if (!confirm(`Enable user "${name}"?`)) {
      return;
    }

    const res = await fetch("/api/users/enable", {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        id,
      }),
    });

    switch (res.status) {
      case 200:
        result = "User enable!";
        let index = users.findIndex((user) => user.id == id);
        users[index].disabled = false;
        break;
      case 404:
        result = "That user does not exist. (Code: 404)";
        break;
      case 500:
        result = "There was an internal server error. (Code: 500)";
        break;
      default:
        result = `An error has occurred. (Code: ${res.status}).`;
        break;
    }
  }

  async function disableUser(id, name) {
    if (!confirm(`Disable user "${name}"?`)) {
      return;
    }

    const res = await fetch("/api/users/disable", {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        id,
      }),
    });

    switch (res.status) {
      case 200:
        result = "User disabled!";
        let index = users.findIndex((user) => user.id == id);
        users[index].disabled = true;
        break;
      case 404:
        result = "That user does not exist. (Code: 404)";
        break;
      case 500:
        result = "There was an internal server error. (Code: 500)";
        break;
      default:
        result = `An error has occurred. (Code: ${res.status}).`;
        break;
    }
  }

  /* ----- Permissions ----- */
  async function updatePermissions(id, property) {
    // make patch to send
    let index = users.findIndex((user) => user.id == id);
    let patch = {
      user_id: id,
      manage_links: users[index].manage_links,
      manage_users: users[index].manage_users,
    };

    // send request
    const res = await fetch("/api/users/update/permissions", {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(patch),
    });

    // update checkbox if success, undo if not
    if (!res.ok) {
      users[index][property] = !users[index][property];
      users = users;
      result = await res.text();
    }
  }

  /* ----- Editing Username ----- */
  let editingUsernameOf = "";
  let editedUsername = "";

  function editNameClicked(id) {
    editingUsernameOf = id;
    editedUsername = users.find((user) => user.id == id).username;
  }

  async function sendNameEdit() {
    let patch = {
      user_id: editingUsernameOf,
      new_name: editedUsername,
    };
    const res = await fetch("/api/users/update/username", {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(patch),
    });

    switch (res.status) {
      case 200:
        result = "Username updated!";
        let index = users.findIndex((user) => user.id == editingUsernameOf);
        users[index].username = editedUsername;
        users = users;
        editingUsernameOf = "";
        break;
      case 404:
        result = "That user does not exist. (Code: 404)";
        break;
      case 500:
        result = "There was an internal server error. (Code: 500)";
        break;
      default:
        result = `An error has occurred. (Code: ${res.status}).`;
        break;
    }
  }
</script>

<div id="content">
  <h2>Manage Users</h2>
  <a href="/new_user">Create New User</a>
  <table>
    <thead>
      <tr>
        {#each columnNames as col}
          <th>
            <button type="button" on:click={() => sort(col)}>
              {#if sortBy.col == col}
                {#if sortBy.ascending}
                  ▼
                {:else}
                  ▲
                {/if}
              {:else}
                ▾
              {/if}
            </button>
            <p>{col}</p>
          </th>
        {/each}
        <th>Permissions</th>
        <th>Action</th>
      </tr>
    </thead>
    <tbody>
      {#each users as user}
        <tr class={user.disabled ? "disabled" : ""}>
          <td>{user.id}</td>
          <td>
            {#if editingUsernameOf == user.id}
              <input bind:value={editedUsername} />
              <button on:click={sendNameEdit}>Save</button>
            {:else}
              {user.username}
              <button on:click={() => editNameClicked(user.id)}> Edit </button>
            {/if}
          </td>
          <td>
            <input
              type="checkbox"
              name="manage_links"
              bind:checked={user.manage_links}
              on:change={() => updatePermissions(user.id, "manage_links")}
            />
            <label for="manage_links">Manage All Links</label><br />
            <input
              type="checkbox"
              for="manage_users"
              bind:checked={user.manage_users}
              on:change={() => updatePermissions(user.id, "manage_users")}
            />
            <label for="manage_users">Manage All Users</label><br />
          </td>
          <td>
            {#if user.orig}
              <p>Original</p>
            {:else}
              {#if user.disabled}
                <button on:click={() => enableUser(user.id, user.username)}>
                  Enable
                </button>
              {:else}
                <button on:click={() => disableUser(user.id, user.username)}>
                  Disable
                </button>
              {/if}
              <button on:click={() => deleteUser(user.id, user.username)}>
                Delete
              </button>
            {/if}
          </td>
        </tr>
      {/each}
    </tbody>
  </table>

  <div class="note">
    <p>Note:</p>
    <p>
      Deleting a user will permanently delete their account
      <strong>and all their links</strong>. Disabling accounts preserves their
      links and is able to be undone.
    </p>
  </div>

  <p>{result}</p>
</div>

<style>
  table {
    border-collapse: collapse;
    margin-top: 1em;
    margin-bottom: 1em;
  }

  th,
  td {
    border: 1px solid;
    padding: 15px;
    text-align: left;
  }

  th {
    font-weight: bold;
  }

  tbody tr:hover {
    background-color: #f5f5f5;
  }

  .disabled td {
    color: gray;
  }

  button {
    font-size: 0.8em;
    padding: 0.2em;
  }

  .note {
    max-width: 20em;
    padding: 0.5em;
    background-color: #f5f5f5;
    margin: 1em;
  }
</style>
