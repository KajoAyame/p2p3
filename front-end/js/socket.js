var sock = new WebSocket("ws://127.0.0.1:4242/");
var first_remove = true;

sock.onopen = function(event){
  //TODO: connection is now open
};

sock.onmessage = function(event){
  var json = event.data,
  obj = eval("(" + json + ')');
  console.log(obj);
  switch (obj.variant) {
    case "InsertString":
      editor.setValue(obj.fields[1]); //TODO
      break;
    case "Output":
      console.log(obj.fields[0]);
      var output = document.getElementById('output');
      output.innerHTML = obj.fields[0];
      break;
    case "InsertChar":
      console.log(obj);
      var delta = asDelta(obj.fields[1], true, obj.fields[0]);
      editor.getSession().getDocument().applyDeltas([delta]);
      break;
    case "DeleteChar":
      console.log(obj);
      var delta = asDelta(obj.fields[1], true, obj.fields[0]);
      editor.getSession().getDocument().applyDeltas([delta]);
      break;
    case "DisableEditing":
      console.log("Disabling editing");
      editor.setReadOnly(true);
      editor.container.style.pointerEvents="none"
      editor.renderer.setStyle("disabled", true)
      editor.blur()
      break;
    default:

  }
}


// Convert a WOOT operation to an ACE delta object for WOOT index i:
function asDelta(ch, isVisible, i) {
  return {
    action: isVisible ? "insertText" : "removeText",
    range: {
      start: pos(i),
      end:   pos(i+1)
    },
    text: ch
  };
}

function compileOnClick() {
    sock.send(JSON.stringify({
      variant: "Compile",
      fields: [],
    }));
}

function commitOnClick() {
    console.log("Commit clicked");
    sock.send(JSON.stringify({
      variant: "Commit",
      fields: [],
    }));
}

function insert_char_at_position(index, character) {
  sock.send(JSON.stringify({
    variant: "InsertChar",
    fields: [index, character],
  }));
}

function delete_char_at_position(index) {
  sock.send(JSON.stringify({
    variant: "DeleteChar",
    fields: [index],
  }));
}

editor.getSession().on('change', function(e) {
  console.log(e);
    switch (e.action) {
      case "insert":
      var index = idx(e.start);
        if (enter_key_detected(e)) {
          insert_char_at_position(index, '\n');
        } else if (e.lines[0].length == 1) {
          insert_char_at_position(index, e.lines[0]);
        }
        break;
      case "remove":
        if (first_remove) {
          first_remove = false;
          return;
        }
        var index = idx(e.start);
        if (enter_key_detected(e)) {
          delete_char_at_position(index);
        } else if (e.lines[0].length == 1) {
          delete_char_at_position(index);
        }
        break;
    }
});

function enter_key_detected (e) {
  if (e.start.row != e.end.row && e.lines.length == 2) {
    if (e.lines[0] == "" && e.lines[1] == "") {
      return true;
    }
  }
  return false;
}

editor.getSession().selection.on('changeSelection', function(e) {
  console.log(e);
});

editor.getSession().selection.on('changeCursor', function(e) {
  console.log(e);
});

function getSelectedMode(mode) {
  editor.session.setMode("ace/mode/" + mode.value);
  sock.send(JSON.stringify({
    variant: "Mode",
    fields: [mode.value],
  }));
}
