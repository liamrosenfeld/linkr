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
