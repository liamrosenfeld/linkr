/**
 * @param {number} id
 */
function removeByID(id) {
    const options = {
        method: "POST",
        headers: {
            "Content-Type": "application/x-www-form-urlencoded",
        },
        body: "id=" + id,
    };
    fetch("/api/users/delete/", options).then((response) => {
        switch (response.status) {
            case 200:
                document.getElementById("manage-output").textContent =
                    "User deleted!";
                document.getElementById(`${id}-row`).remove();
                break;
            case 404:
                document.getElementById("manage-output").textContent =
                    "That user does not exist. (Code: 404)";
                break;
            case 405:
                document.getElementById("manage-output").textContent =
                    "You cannot delete yourself here.";
                break;
            case 500:
                document.getElementById("manage-output").textContent =
                    "There was an internal server error. (Code: 500)";
                break;
            default:
                document.getElementById("manage-output").textContent =
                    `An error has occurred. (Code: ${response.status}).`;
                break;
        }
    });
}

/**
 * @param {number} id
 */
function updatePermissions(id) {
    // Get the checkbox
    const links = document.getElementById(`${id}-permission-links`).checked;
    const users = document.getElementById(`${id}-permission-users`).checked;

    const data = { user_id: id, manage_links: links, manage_users: users };
    const dataEncoded = new URLSearchParams(data).toString();

    const options = {
        method: "POST",
        headers: {
            "Content-Type": "application/x-www-form-urlencoded",
        },
        body: dataEncoded,
    };
    fetch("/api/users/update/permissions", options).then((response) => {
        switch (response.status) {
            case 200:
                document.getElementById("manage-output").textContent =
                    "Permissions updated!!";
                break;
            case 401:
                document.getElementById("manage-output").textContent =
                    "You are not logged in. (Code: 401)";
                break;
            case 403:
                document.getElementById("manage-output").textContent =
                    "Your user is not allowed to other manage other users. (Code: 403)";
                break;
            case 404:
                document.getElementById("manage-output").textContent =
                    "That user does not exist. (Code: 404)";
                break;
            case 405:
                document.getElementById("manage-output").textContent =
                    "You cannot edit the primary user's permissions. (Code: 405)";
                break;
            case 500:
                document.getElementById("manage-output").textContent =
                    "There was an internal server error. (Code: 500)";
                break;
            default:
                document.getElementById("manage-output").textContent =
                    `An error has occurred. (Code: ${response.status}).`;
                break;
        }
    });
}
