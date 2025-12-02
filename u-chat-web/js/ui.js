function addMessage(user, text) {
  const messages = document.getElementById("messages");

  const div = document.createElement("div");
  div.className = "message";

  const self = localStorage.getItem("username");
  if (user === self) div.classList.add("you");

  // Heart avatar for you, dot avatar for others
  const avatar = user === self ? "♥" : "●";

  div.innerText = avatar + " " + user + ": " + text;

  messages.appendChild(div);
  messages.scrollTop = messages.scrollHeight;
}

function sendMessage() {
  const box = document.getElementById("msgBox");
  const text = box.value;
  const user = localStorage.getItem("username");

  if (!text) return;

  ws.send(JSON.stringify({ user, text }));
  box.value = "";
}

function sendOnEnter(event) {
  if (event.key === "Enter") sendMessage();
}

function logout() {
  localStorage.removeItem("username");
  window.location = "index.html";
}

window.onload = () => {
  const name = localStorage.getItem("username");
  if (name && document.getElementById("usernameDisplay")) {
    document.getElementById("usernameDisplay").innerText = name;
  }

  if (document.getElementById("messages")) {
    connectWS();
  }
};
