<script type="ts">
  import { currentUser, UserInfo } from "../stores";

  let user: UserInfo;
  currentUser.subscribe((value) => (user = value));

  async function logout() {
    let res = await fetch("/api/users/logout");
    if (res.ok) {
      currentUser.set(null);
      window.location.pathname = "/login";
    }
  }
</script>

<header>
  {#if user != null}
    <div id="left">
      <a href="/">Dashboard</a>
      {#if user.manage_links}
        <a href="/manage_links">Manage Links</a>
      {/if}
      {#if user.manage_users}
        <a href="/manage_users">Manage Users</a>
      {/if}
    </div>

    <div id="right" class="dropdown">
      {user.username}
      <div class="dropdown-content">
        <a href="/manage_account">Manage Account</a>
        <a href="" on:click={logout}> Logout </a>
      </div>
    </div>
  {/if}
</header>

<style>
  header {
    background-color: gray;
    padding: 1.25em;
    color: white;
    flex-shrink: 0;
  }

  header #left a {
    color: white;
    margin-left: 1em;
  }

  header #left {
    float: left;
  }

  header #right {
    float: right;
  }

  /* The container <div> - needed to position the dropdown content */
  .dropdown {
    position: relative;
    display: inline-block;
  }

  /* Dropdown Content (Hidden by Default) */
  .dropdown-content {
    display: none;
    position: absolute;
    background-color: #f9f9f9;
    min-width: 10em;
    box-shadow: 0px 8px 16px 0px rgba(0, 0, 0, 0.2);
    z-index: 1;
    right: 0;
  }

  /* Links inside the dropdown */
  .dropdown-content a {
    color: black;
    padding: 12px 16px;
    text-decoration: none;
    display: block;
  }

  /* Change color of dropdown links on hover */
  .dropdown-content a:hover {
    background-color: #f1f1f1;
  }

  /* Show the dropdown menu on hover */
  .dropdown:hover .dropdown-content {
    display: block;
  }
</style>
