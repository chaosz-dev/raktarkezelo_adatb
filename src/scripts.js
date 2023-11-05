const { invoke } = window.__TAURI__.tauri;

// Utility functions

function getCookie(c_name) {
  var c_value = " " + document.cookie;
  var c_start = c_value.indexOf(" " + c_name + "=");
  if (c_start == -1) {
    c_value = null;
  }
  else {
    c_start = c_value.indexOf("=", c_start) + 1;
    var c_end = c_value.indexOf(";", c_start);
    if (c_end == -1) {
      c_end = c_value.length;
    }
    c_value = unescape(c_value.substring(c_start, c_end));
  }
  return c_value;
}

function register_driver() {
  if (register() == true) {
    alert('Register susscesful!');
    window.location = 'login.html';
  } else {
    alert('Register failed!');
  }
}

// Login and register functions

window.addEventListener('load', loaded);
function loaded() {
  function showLoginRegister() {
    var username = getCookie("username");

    if (username == null) {
      var x = document.getElementById("RegisterLogin");
      var y = document.getElementById("LoggedIn");
      x.style.visibility = "visible";
      y.style.visibility = "hidden";
    } else {
      var x = document.getElementById("RegisterLogin");
      var y = document.getElementById("LoggedIn");
      x.style.visibility = "hidden";
      y.style.visibility = "visible";
    }
  }
  showLoginRegister();
}

function register() {
  var username = document.getElementById("username").value;
  var email = document.getElementById("email").value;
  var password = document.getElementById("psw").value;
  var password_repeat = document.getElementById("psw-repeat").value;

  if (!email.includes("@")) {
    alert('Email must contain a \'@\' symbol!')
    return false
  }
  if (!(password === password_repeat)) {
    alert('Passwords must match!')
    document.getElementById("psw").value = "";
    document.getElementById("psw-repeat").value = "";
    return false
  }

  console.log("Starting register process...");

  invoke('register_user', {
    username: String(username),
    email: String(email),
    password: String(password),
  }).then((ret) => {
    if (ret) {
      console.log("Function returned."); return true
    }
  }).catch((e) => console.error(e));

  document.cookie = "username=" + username + ";";

  return true;
}

function login() {
  var username = document.getElementById("username").value;
  var password = document.getElementById("psw").value;

  invoke('login', {
    username: String(username),
    password: String(password),
  }).then((ret) => {
    if (ret) {
      console.log(ret);
      console.log("Function returned.");
      document.cookie = "username=" + username + ";"
      return true;
    }
  }).catch((e) => console.log(e));

  window.location.replace = "./index.html";

  return true;
}

function logout() {
  console.log("Logout called.")

  document.cookie = 'username=' + getCookie("username") + ';max-age=-1' + ';path=/';
}

// Content functions

function get_products() {
  console.log("get_products called...");

  invoke('products').then((ret) => {
    if (ret) {
      console.log("Function returned.");
      console.log(ret);
      console.log(ret.length);

      var table = document.createElement('table');
      var tr = document.createElement('tr');
      var array = ["#", "Name", "Unit Price", "Description"];

      for (var i = 0; i < array.length; i++) {
        var th = document.createElement('th');
        var text = document.createTextNode(array[i]);
        th.appendChild(text);
        tr.appendChild(th);
      }

      table.appendChild(tr);
      for (var i = 0; i < ret.length; i++) {
        var tr = document.createElement('tr');

        var td1 = document.createElement('td');
        var td2 = document.createElement('td');
        var td3 = document.createElement('td');
        var td4 = document.createElement('td');

        var row_num = document.createTextNode(i);
        var text1 = document.createTextNode(ret[i].name);
        var text2 = document.createTextNode(ret[i].unit_price);
        var text3 = document.createTextNode(ret[i].description);

        td1.appendChild(row_num);
        td2.appendChild(text1);
        td3.appendChild(text2);
        td4.appendChild(text3);


        tr.appendChild(td1);
        tr.appendChild(td2);
        tr.appendChild(td3);
        tr.appendChild(td4);

        table.appendChild(tr);
      }
      table.classList.add("product_table")
      document.body.appendChild(table);
      console.log("Table appended.")
    }
  }).catch((e) => console.error(e));
}

function search_product() {
  var product_name = document.getElementById("product_search").value;

  if (product_name == null) {
    alert('Product name cannot be empty!');
  }

  invoke('search_product', {
    product_name: String(product_name),
  }).then((ret) => { if (!ret) { console.log("Function returned."); return ret } }).catch((e) => console.error(e));
}