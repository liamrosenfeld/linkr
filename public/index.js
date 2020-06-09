/* ------------------------- functions run directly ------------------------- */

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
    fetch("/api/links/delete/", options).then((response) => {
        switch (response.status) {
            case 200:
                document.getElementById("manage-output").textContent =
                    "Shortcut deleted!";
                document.getElementById(`${id}-row`).remove();
                break;
            case 404:
                document.getElementById("manage-output").textContent =
                    "That link does not exist. (Code: 404)";
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
    let longElement = document.getElementById(`${id}-long`);

    if (button.textContent === "Edit") {
        let long = longElement.textContent;
        let input = document.createElement("input");
        input.value = long;
        longElement.replaceWith(input);
        input.id = `${id}-long`;
        button.textContent = "Save";

    } else if (button.textContent === "Save") {
        let long = longElement.value;
        sendUpdate(id, long).then(success => {
            if (success) {
                let text = document.createElement("span");
                text.textContent = long;
                text.id = `${id}-long`; // has to be after replace
                longElement.replaceWith(text);
                button.textContent = "Edit";
            }
        });
    } else {
        console.error("button is not named correctly");
    }
}


/* ------------------------- functions for functions ------------------------ */

/**
 * @param {number} id
 * @param {string} newLong
 */
async function sendUpdate(id, newLong) {
    const data = { id: id, long: newLong };
    const dataEncoded = new URLSearchParams(data).toString();
    const options = {
        method: "POST",
        headers: {
            "Content-Type": "application/x-www-form-urlencoded",
        },
        body: dataEncoded,
    };

    let response = await fetch("/api/links/update/", options);
    switch (response.status) {
        case 200:
            document.getElementById("manage-output").textContent =
                "Shortcut updated!";
            return true;
        case 404:
            document.getElementById("manage-output").textContent =
                "That link does not exist. Please refresh your page. (Code: 404)";
            return false;
        case 500:
            document.getElementById("manage-output").textContent =
                "There was an internal server error. (Code: 500)";
            return false;
        default:
            document.getElementById("manage-output").textContent =
                "There was an internal server error.";
            return false;
    }
}
