/* ------------------------- functions run directly ------------------------- */

/**
 * @param {string} short
 */
function removeByShort(short) {
    if (!confirm(`Delete the link "${short}"?`)) {
        return;
    }

    const options = {
        method: "POST",
        headers: {
            "Content-Type": "application/x-www-form-urlencoded",
        },
        body: "short=" + short,
    };
    fetch("/api/links/delete/", options).then((response) => {
        switch (response.status) {
            case 200:
                document.getElementById("manage-output").textContent =
                    "Shortcut deleted!";
                document.getElementById(`${short}-row`).remove();
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
 * @param {string} short
 */
function updateButtonClicked(short) {
    let button = document.getElementById(`${short}-update`);
    let longElement = document.getElementById(`${short}-long`);

    if (button.textContent === "Edit") {
        let long = longElement.textContent;
        let input = document.createElement("input");
        input.value = long;
        longElement.replaceWith(input);
        input.id = `${short}-long`;
        button.textContent = "Save";

    } else if (button.textContent === "Save") {
        let long = longElement.value;
        sendUpdate(short, long).then(success => {
            if (success) {
                let text = document.createElement("span");
                text.textContent = long;
                text.id = `${short}-long`; // has to be after replace
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
 * @param {string} short
 * @param {string} newLong
 */
async function sendUpdate(short, newLong) {
    const data = { short: short, long: newLong };
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
