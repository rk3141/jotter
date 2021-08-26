const promptEl = document.getElementById("prompt");

promptEl.addEventListener("blur", (event) => {
  event.target.focus();
});

promptEl.addEventListener("change", (event) => {
  const command = event.target.value;

  const commands = command.split(" ");

  run(...commands).then((result) => hydrate_output(commands, result));

  event.target.value = "";
});

function hydrate_output(commands, result) {
  const outputElement = document.getElementById("output");

  const colorClasses = outputElement.classList.values();
  outputElement.classList.remove(...colorClasses);
  outputElement.innerText = "";
  if (result !== null) {
    let { error, syntax_error } = result;
    if (error) {
      outputElement.classList.add("text-red");
      outputElement.innerText = `Error unknown command '${commands[0]}'`;
      return;
    } else if (syntax_error) {
      outputElement.classList.add("text-red");
      outputElement.innerText = `Wrong syntax for command '${commands[0]}', maybe arguments missing?`;
      return;
    }
  }

  switch (commands[0]) {
    case "all":
      if (result) {
        const text = result.join("\n");

        outputElement.classList.add("text-green-soft");
        outputElement.innerText = text;
      } else {
        outputElement.classList.add("text-yellow");
        outputElement.innerText = "You have got 0 notes";
      }
      break;

    case "get":
      if (result) {
        const { data } = result;

        outputElement.classList.add("text-green-soft");
        outputElement.innerText = `${commands[1]}: "${data}"`;
      } else {
        outputElement.classList.add("text-red");
        outputElement.innerText = `${commands[1]} doesnt exist :/`;
      }

      break;

    case "set":
      if (result) {
        outputElement.classList.add("text-green-soft");
        outputElement.innerText = `set ${commands[1]} to '${commands[2]}'`;
      } else {
        outputElement.classList.add("text-red");
        outputElement.innerText = `'${commands[1]}' is already assigned a value, try 'remove ${commands[1]}' and then try setting again`;
      }

      break;

    case "remove":
      if (result) {
        outputElement.classList.add("text-green-soft");
        outputElement.innerText = `removed ${commands[1]}`;
      } else {
        outputElement.classList.add("text-red");
        outputElement.innerText = `${commands[1]} doesnt have a value assigned to it, please consider setting a value to it before removing it :)`;
      }

      break;

    default:
      console.log(result);
      break;
  }
}

async function run(...command) {
  return await fetch("/api/run", {
    method: "POST",
    body: JSON.stringify({
      command,
    }),
    headers: new Headers({
      "Content-Type": "application/json",
    }),
  })
    .then((v) => v.json())

    .catch((error) => {
      return { error };
    });
}
