/* ------------------------------ run at start ------------------------------ */

document.getElementById("shorten").addEventListener("click", sendCreate);
getTable();

/* ------------------------- functions run directly ------------------------- */

function removeByID(id) {
    const options = {
        method: "POST",
        headers: {
            "Content-Type": "application/x-www-form-urlencoded",
        },
        body: "id=" + id,
    };
    fetch("/api/delete/", options).then((response) => {
        switch (response.status) {
            case 200:
                document.getElementById("manage-output").textContent =
                    "Shortcut deleted!";
                getTable()
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
                    "There was an internal server error.";
                break;
        }
    });
}

function sendCreate() {
    let short = document.getElementById("short").value;
    let long = document.getElementById("long").value;

    const data = { short: short, long: long };
    const dataEncoded = new URLSearchParams(data).toString();
    const options = {
        method: "POST",
        headers: {
            "Content-Type": "application/x-www-form-urlencoded",
        },
        body: dataEncoded,
    };

    fetch("/api/shorten/", options).then((response) => {
        switch (response.status) {
            case 200:
                document.getElementById("new-output").textContent =
                    "Shortcut created!";
                getTable()
                break;
            case 409:
                document.getElementById("new-output").textContent =
                    "That short is already in use. (Code: 409)";
                break;
            case 500:
                document.getElementById("new-output").textContent =
                    "There was an internal server error. (Code: 500)";
                break;
            default:
                document.getElementById("new-output").textContent =
                    "There was an internal server error.";
                break;
        }
    });
}

function getTable() {
    document.getElementById("manage-table").innerHTML = ""

    const options = {
        method: "GET",
        headers: {
            Accept: "application/json",
        },
    };
    
    fetch("/api/all/", options).then((response) => {
        if ((response.status = 200)) {
            response.json().then((data) => {
                if (data.length == 0) {
                    document
                        .getElementById("manage-table")
                        .appendChild(document.createTextNode("No shortcuts added"));
                } else {
                    document
                        .getElementById("manage-table")
                        .appendChild(buildTable(data.sort((a, b) => a.id - b.id)));
                }
            }).catch((error) => {
                document
                    .getElementById("manage-table")
                    .appendChild(document.createTextNode(`Invalid data received. (${error})`));
            });
        } else {
            document
                .getElementById("manage-table")
                .appendChild(document.createTextNode("There was an internal server error. (Code: 500)"));
        }
    });
    
}


/* ------------------------- functions for functions ------------------------ */

function buildTable(links) {
    let table = document.createElement("table");

    // header
    const colNames = ["Short", "Long", "Actions"];
    let headerRow = document.createElement("tr");
    for (const colName of colNames) {
        let header = document.createElement("th");
        header.textContent = colName;
        headerRow.appendChild(header);
    }
    table.appendChild(headerRow)

    // Body rows
    const propNames = ["short", "long"];
    for (const link of links) {
        let row = document.createElement("tr");

        // add text from server
        for (const prop of propNames) {
            let entry = document.createElement("td");
            entry.textContent = link[prop];
            row.appendChild(entry);
        }

        // actions
        let deleteButton = document.createElement("button");
        deleteButton.textContent = "delete";
        deleteButton.addEventListener("click", () => { removeByID(link.id) });
        row.appendChild(deleteButton);

        table.appendChild(row);
    }


    return table
}
