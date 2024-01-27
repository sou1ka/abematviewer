const { appWindow } = window.__TAURI__.window;

window.addEventListener(function() {
  document.addEventListener('keydown', async function(e) {
    console.log(e);
    if(e.key == 'F5' || (e.ctrlKey && e.key == 'r') || e.key == 'F7') {
      e.preventDefault();
      e.stopPropagation();

    } else if(e.altKey && e.key == 'Enter') {
      appWindow.toggleMaximize();

    } else if(e.key == 'F11') {
      if(await appWindow.isFullscreen()) {
        appWindow.setDecorations(true);
        appWindow.setTitle(true);
        appWindow.setFullscreen(false);
      } else {
        appWindow.setDecorations(false);
        appWindow.setTitle(false);
        appWindow.setFullscreen(true);
      }
  
    } else if(e.key == 'Escape') {
      appWindow.setFullscreen(false);
      appWindow.setDecorations(true);
      appWindow.setTitle(true);
    }
  });

  document.addEventListener('contextmenu', function(e) {
    e.preventDefault();
    e.stopPropagation();
  }, false);

  document.addEventListener("selectstart", function(e) {
    e.preventDefault();
    e.stopPropagation();
  }, false);
})