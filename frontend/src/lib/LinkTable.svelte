<script>
  /* ----- Setup ----- */
  import { onMount } from "svelte";
  export let show_author = false;
  let columnNames = [""];
  let links = [];

  onMount(async () => {
    if (show_author) {
      columnNames = ["short", "long", "notes", "created_at", "created_by"];
    } else {
      columnNames = ["short", "long", "notes", "created_at"];
    }
    let api = "";
    if (show_author) {
      api = "/api/links/all";
    } else {
      api = "/api/links/for_user";
    }
    const res = await fetch(api);
    links = await res.json();
    sort(undefined);
  });

  /* ----- Sorting ----- */
  let sortBy = { col: "created_at", ascending: false };

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
    // links = is needed to update it
    links = links.sort(sortFunc);
  }

  export const addRow = (link) => {
    // this instead of push makes the table update
    links = [...links, link];
    sort(undefined);
  };

  /* ----- Deleting ----- */
  let result = "";

  async function deleteLink(short) {
    if (!confirm(`Delete the link "${short}"?`)) {
      return;
    }

    const res = await fetch("/api/links/delete", {
      method: "DELETE",
      headers: {
        "Content-Type": "text/plain",
      },
      body: short,
    });

    switch (res.status) {
      case 200:
        result = "Link deleted!";
        links.splice(
          links.findIndex((link) => link.short == short),
          1
        );
        links = links;
        break;
      case 404:
        result = "That link does not exist. (Code: 404)";
        break;
      case 500:
        result = "There was an internal server error. (Code: 500)";
        break;
      default:
        result = `An error has occurred. (Code: ${res.status}).`;
        break;
    }
  }

  /* ----- Editing ----- */
  let editingLongOf = "";
  let editedLong = "";

  function editLongClicked(short) {
    editingLongOf = short;
    editedLong = links.find((link) => link.short == short).long;
  }

  async function sendLongEdit() {
    const res = await fetch("/api/links/update", {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        short: editingLongOf,
        long: editedLong,
      }),
    });

    switch (res.status) {
      case 200:
        result = "Link updated!";
        let index = links.findIndex((link) => link.short == editingLongOf);
        links[index].long = editedLong;
        editingLongOf = "";
        break;
      case 404:
        result = "That link does not exist. (Code: 404)";
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

<div>
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
        <th>Action</th>
      </tr>
    </thead>
    <tbody>
      {#each links as link}
        <tr>
          {#each columnNames as col}
            {#if col == "long"}
              <!-- have editing if it's the long cell -->
              <td>
                {#if editingLongOf == link.short}
                  <input bind:value={editedLong} />
                  <button on:click={sendLongEdit}>Save</button>
                {:else}
                  {link.long}
                  <button on:click={() => editLongClicked(link.short)}>
                    Edit
                  </button>
                {/if}
              </td>
            {:else}
              <td>{link[col]}</td>
            {/if}
          {/each}
          <td>
            <button type="button" on:click={() => deleteLink(link.short)}>
              Delete
            </button>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>

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

  button {
    font-size: 0.8em;
    padding: 0.2em;
  }
</style>
