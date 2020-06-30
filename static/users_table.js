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

/**
 * @param {number} id
 */
function updateButtonClicked(id) {
    let button = document.getElementById(`${id}-update`);
    let nameElement = document.getElementById(`${id}-name`);

    if (button.textContent === "Edit") {
        let name = nameElement.textContent;
        let input = document.createElement("input");
        input.value = name;
        nameElement.replaceWith(input);
        input.id = `${id}-name`;
        button.textContent = "Save";

    } else if (button.textContent === "Save") {
        let name = nameElement.value;
        sendUpdate(id, name).then(success => {
            if (success) {
                let text = document.createElement("span");
                text.textContent = name;
                text.id = `${id}-name`; // has to be after replace
                nameElement.replaceWith(text);
                button.textContent = "Edit";
            }
        });
    } else {
        console.error("button is not named correctly");
    }
}

/**
 * @param {number} id
 * @param {string} newName
 */
async function sendUpdate(id, newName) {
    const data = { user_id: id, new_name: newName };
    const dataEncoded = new URLSearchParams(data).toString();
    const options = {
        method: "POST",
        headers: {
            "Content-Type": "application/x-www-form-urlencoded",
        },
        body: dataEncoded,
    };

    let response = await fetch("/api/users/update/username", options);
    switch (response.status) {
        case 200:
            document.getElementById("manage-output").textContent =
                "Username Updated";
            return true;
        case 404:
            document.getElementById("manage-output").textContent =
                "That user does not exist. Please refresh your page";
            return false;
        case 409:
            document.getElementById("manage-output").textContent =
                "That username is taken. (Code: 409)";
            break;
        case 500:
            document.getElementById("manage-output").textContent =
                "There was an internal server error. (Code: 500)";
            return false;
        default:
            document.getElementById("manage-output").textContent =
                `There was an error. (Code: ${response.status})`;
            return false;
    }
}
