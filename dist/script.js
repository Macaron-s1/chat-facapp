const { invoke } = window.__TAURI__.tauri

function nameFunction(){
    var nameInput = document.getElementById("name").value;

    invoke('greet', { name: nameInput })
        .then((response) => console.log(response))
}


