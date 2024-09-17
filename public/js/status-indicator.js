document.addEventListener('htmx:wsConnecting', function() {
  updateWsStatus('connecting');
});

document.addEventListener('htmx:wsOpen', function() {
  updateWsStatus('open');
});

document.addEventListener('htmx:wsClose', function() {
  updateWsStatus('closed');
});

function updateWsStatus(status) {
  var circle = document.getElementById('ws-status-circle');
  if (!circle) return;

  var ping = document.getElementById('ws-status-ping');
  if (!ping) return;

  // Remove existing status classes
  circle.classList.remove('bg-green-500', 'bg-orange-500', 'bg-red-500');
  ping.classList.remove('bg-green-500', 'bg-orange-500', 'bg-red-500');

  // Update the class based on the status
  if (status === 'connecting') {
    circle.classList.add('bg-orange-500');
    ping.classList.add('bg-orange-500');
  } else if (status === 'open') {
    circle.classList.add('bg-green-500');
    ping.classList.add('bg-green-500');
  } else if (status === 'closed') {
    circle.classList.add('bg-red-500');
    ping.classList.add('bg-red-500');
  }
}

