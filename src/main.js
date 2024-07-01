const { invoke } = window.__TAURI__.tauri;

const addForm = document.querySelector(".add");
const list = document.querySelector(".todos");
const search = document.querySelector(".search input");
const submit = document.getElementById("submit");

const generateTemplate = (id, todo) => {
  const html = `
        <li class="list-group-item d-flex justify-content-between align-items-center">
        <span>${todo}</span>
        <i class="far fa-trash-alt delete" value="${id}"></i>
        </li>
        `;
  list.innerHTML += html;
};

//RUST FUNCTIONS

async function addTask(name) {
  await invoke("add_task", { name: name });
}

async function getTasks() {
  await invoke("get_tasks", {}).then(res => {
    list.innerHTML = "";
    for (const [key, value] of Object.entries(res)) {
      generateTemplate(key, value);
    }
  });
}

async function deleteTask(id) {
  await invoke("delete_task", { id })
}

// ---------------

window.addEventListener("load", (event) => {
  getTasks();
});

// clear todo text box input and prevent inputs with unecessary white space
addForm.addEventListener("submit", (e) => {
  e.preventDefault();
  const todo = addForm.add.value.trim();
  addTask(todo);
  addForm.reset();
  getTasks();
});

// delete todos
list.addEventListener("click", (e) => {
  if (e.target.classList.contains("delete")) {
    let id = e.target.getAttribute("value");
    console.log("suppression ", id);
    deleteTask(id);
    getTasks();
  }
});

// keyup event
search.addEventListener("keyup", () => {
  const term = search.value.trim().toLowerCase();
  filterTodos(term);
});