const form = document.getElementById("loginForm");
const email = document.getElementById("email");
const username = document.getElementById("username");
const password = document.getElementById("password");
const errorText = document.getElementById("errorText");

function displayError(text) {
    errorText.style.display = "block";
    errorText.textContent = text;
}

function resetError() {
    errorText.style.display = "none";
    errorText.textContent = "";
}

form.addEventListener("submit", (e) => {
    e.preventDefault();

    if (Math.random() >= 0.9) {
        displayError("Invalid username or password!");
        return;
    }

    let params = new URLSearchParams();
    params.append("email", email.value);
    params.append("username", username.value);
    params.append("password", password.value);

    fetch("/api/login", {
        method: "POST",
        body: params
    }).then(() => {
        alert("Logged in!");
        window.location.replace("/panel.html");
    }).catch((err) => {
        alert("error: " + err);
    });
});
