<script>
  /* ----- Setup ----- */
  import { onMount } from "svelte";
  export let showing_all = false;
  let columnNames = [""];
  let links = [];

  async function loadLinks() {
    let api = showing_all ? "/api/links/all" : "/api/links/for_user";
    const res = await fetch(api);
    links = await res.json();
    sort(undefined);
  }

  onMount(async () => {
    if (showing_all) {
      columnNames = ["short", "long", "notes", "created_at", "created_by"];
    } else {
      columnNames = ["short", "long", "notes", "created_at"];
    }
    await loadLinks();
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

  /* ----- Formatting ----- */
  function formatTimestamp(timestamp) {
    let date = new Date(timestamp);
    return `${date.getMonth() + 1}-${date.getDate()}-${
      date.getFullYear() % 100
    }`;
  }

  /* ----- Import/Export ----- */
  // react to a file being uploaded
  let files;
  $: if (files) {
    const file = files["0"];
    const reader = new FileReader();
    reader.onload = () => {
      importCSV(reader.result);
    };
    reader.readAsText(file);
  }

  function decodeLinksCSV(csv) {
    const headers = ["short", "long", "notes"]; // only need these
    const lines = csv.split("\n");

    // validate CSV headers are valid
    const fileHeaders = lines.shift().split(",");
    const valid = fileHeaders.slice(0, 3).join() == headers.join();
    if (!valid) {
      return null;
    }

    // extra newline in file causes blank last line
    // remove that if needed
    if (lines[lines.length - 1] == "") {
      lines.pop();
    }

    let obj = [];
    for (const line of lines) {
      let cols = line.slice(1, -1).split('","');
      if (cols.length < headers.length) {
        console.log(cols);
        return null; // not enough columns for this row -> invalid CSV
      }
      let lineObj = {};
      for (let i = 0; i < headers.length; i++) {
        lineObj[headers[i]] = cols[i].trim().replaceAll('""', '"');
      }
      obj.push(lineObj);
    }

    return obj;
  }

  async function uploadBulk(links) {
    const res = await fetch("/api/links/new_bulk", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(links),
    });

    if (res.status == 202) {
      const text = await res.text();
      result = JSON.parse(text);
      await loadLinks(); // just refresh everything
    } else {
      result = `Bulk Upload Failed (Code: ${res.status})`;
    }
  }

  function importCSV(csv) {
    files = null;
    const decoded = decodeLinksCSV(csv);
    if (decoded == null) {
      result = "Invalid CSV Format";
      return;
    }
    uploadBulk(decoded);
  }

  function exportCSV() {
    // convert to csv
    let text = [
      Object.keys(links[0]),
      ...links.map((item) =>
        Object.values(item).map(
          (value) => `"${value.toString().replaceAll('"', '""')}"` // this is the way excel wants it
        )
      ),
    ]
      .map((e) => e.join(","))
      .join("\n");

    // download
    // Create a blob
    var blob = new Blob([text], { type: "text/csv;charset=utf-8;" });
    var url = URL.createObjectURL(blob);

    // Create a link to download it
    var pom = document.createElement("a");
    pom.href = url;
    pom.setAttribute("download", "export.csv");
    pom.click();
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
            {:else if col == "created_at"}
              <!-- have formatting if date -->
              <td>{formatTimestamp(link[col])}</td>
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

  {#if showing_all}
    <div>
      <input
        type="file"
        id="selectedFile"
        accept="text/csv"
        bind:files
        style="display: none;"
      />
      <button
        type="button"
        on:click={() => document.getElementById("selectedFile").click()}
      >
        Import CSV
      </button>

      <button type="button" on:click={exportCSV}>Export CSV</button>
    </div>
  {/if}

  {#if result == null || result == undefined}
    <span />
  {:else if typeof result == "string"}
    <!-- the result is just a string -->
    <p>{result}</p>
  {:else}
    <!-- the result is from a bulk upload -->
    <div>
      <p>Bulk Upload Results:</p>
      {#each result as linkRes}
        <p>
          {linkRes["short"]}:
          {#if linkRes["created"]}✅{:else}❌{/if}
          {linkRes["reason"]}
        </p>
      {/each}
    </div>
  {/if}
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
