<script>
  import LinkTable from "$lib/LinkTable.svelte";

  let short = "";
  let long = "https://";
  let notes = "";
  let result = "";

  let table;

  async function createNew() {
    const res = await fetch("/api/links/new", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        short,
        long,
        notes,
      }),
    });

    if (res.status == 201) {
      // insert into table without another request
      table.addRow({
        short: short,
        long: long,
        notes: notes,
        created_at: new Date().toLocaleDateString(undefined, {
          year: "2-digit",
          month: "2-digit",
          day: "2-digit",
        }),
        created_by: 0,
      });

      // clear
      short = "";
      long = "https://";
      notes = "";

      // set result
      result = "Link created!";
    } else {
      result = await res.text();
    }
  }
</script>

<div id="content">
  <div id="new">
    <h2>Create New Links</h2>
    <form on:submit|preventDefault={createNew}>
      <label for="short">this-domain.com/</label>
      <input name="short" bind:value={short} /><br />

      <label for="short">original url:</label>
      <input name="short" bind:value={long} /><br />

      <label for="short">notes:</label>
      <input name="short" bind:value={notes} /><br />

      <input type="submit" value="Create New" />
    </form>
  </div>

  <div id="result">
    <p>{result}</p>
  </div>

  <div id="manage">
    <h2>Your Links</h2>
    <LinkTable bind:this={table} show_author={false} />
  </div>
</div>

<style>
  input {
    margin-bottom: 10px;
  }
</style>
