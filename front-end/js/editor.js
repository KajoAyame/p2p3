var editor = ace.edit("editor");
editor.setTheme("ace/theme/merbivore");
editor.getSession().getDocument().setNewLineMode("unix");
editor.getSession().setMode("ace/mode/markdown");

// Convert ACE "position" (row/column) to WOOT "index":
function idx(position) {
  return editor.getSession().getDocument().positionToIndex(position);
}

// Covert a WOOT "index" into an ACE "position"
function pos(index) {
  return editor.getSession().getDocument().indexToPosition(index);
}
