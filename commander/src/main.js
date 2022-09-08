const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
});

async function greet() {
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function addNode() {
  table = document.getElementById("nodelist")
  nodeinput = document.getElementById("nodeinput")
  await invoke("addNode", { token : "eyJwYXNzd29yZCI6InRlc3QiLCJob3N0IjoidGVzdCIsImlkIjoidGVzdCJ9"});
  console.log("test");
}

var nodes  = [];




function loadPage(pageName) {
    $("#main-contents").load(pageName);
}


window.addEventListener("hashchange", function() {
    console.log(location.hash);
    loadPage(location.hash.substring(1)+".html");
  });
window.greet = greet;
