const promptEl = document.getElementById("prompt");

promptEl.addEventListener("blur", (event) => {
  event.target.focus();
});

promptEl.addEventListener("change", (event) => {
  console.log(event.target.value);
  event.target.value = "";
});
