import * as wasm from "webwasm";


wasm.greet();


const myForm = document.getElementById("form");
myform.addEventListender("submit", (e) => {
  e.preventDefault();
  const name = document.getElementById("name").value;
  const desc = document.getSelection("#description").value;

  wasm.add_course(name, desc).then(json => {
    alert("添加成功");
    window.location.reload();
  });
});


