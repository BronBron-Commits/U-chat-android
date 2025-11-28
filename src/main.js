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

    console.log("TOKEN:", token);

    err.style.color = "green";
    err.innerText = "Login successful";

    // TODO: navigate to chat UI
  } catch (e) {
    err.innerText = "Invalid credentials";
    console.error(e);
  }
}
