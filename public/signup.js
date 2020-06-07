function sendLogin() {
    let username = document.getElementById("username").value;
    let password = document.getElementById("password").value;

    const data = { username: username, password: password };
    const dataEncoded = new URLSearchParams(data).toString();
    const options = {
        method: "POST",
        headers: {
            "Content-Type": "application/x-www-form-urlencoded",
        },
        body: dataEncoded,
    };

    fetch("/api/users/new/", options).then((response) => {
        if (response.redirected) {
            window.location.href = response.url;
            return;
        }
        switch (response.status) {
            case 409:
                document.getElementById("error-text").textContent =
                    "That username is taken";
                break;
            case 500:
                document.getElementById("error-text").textContent =
                    "There was an internal server error. (Code: 500)";
                break;
        }
    });
}

document.getElementById("submit").addEventListener("click", sendLogin);
