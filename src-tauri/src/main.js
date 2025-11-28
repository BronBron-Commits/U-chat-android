import { invoke } from "@tauri-apps/api/core";

async function login() {
  const user = document.getElementById("username").value.trim();
  const pass = document.getElementById("password").value.trim();
  const err = document.getElementById("error");

  err.innerText = "";

  if (!user || !pass) {
    err.innerText = "Enter username and password";
    return;
  }

  try {
    const token = await invoke("login_request", {
      username: user,
      password: pass
    });

    if (!token) {
      err.innerText = "Invalid login";
      return;
    }

    err.innerText = "Logged in!";
    console.log("JWT =", token);
  } catch (e) {
    console.error(e);
    err.innerText = "Login failed";
  }
}
