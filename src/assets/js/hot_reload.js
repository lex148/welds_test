
(async function(){

  // If a use clicks an external link, The health check will die. don't fire a reload
	let isLeaving = false;
	window.addEventListener('beforeunload', () => { isLeaving = true; });

	fetch("/healthz", { headers: { 'content-type': 'text/event-stream' }})
		.then( watchForDisconnect )
		.catch( pollForReboot );

  function reload() {
		if( !isLeaving ) {
			Turbo.cache.clear();
			window.location = window.location + "";
		}
	}

	let pollCount = 0;
	function pollForReboot() {
		fetch("/healthz").then(reload).catch(() => {
			// If the server is taking a very long time, let the dev know
			pollCount = pollCount + 1;
			// if you can't use mold, you might want to bump this to wait a little longer
			if( pollCount > 40 ) {
				if( confirm('Server Is unavalable, Keep Trying?') ) {
					pollCount = 0;
				} else {
					return;
				}
			}
			setTimeout(pollForReboot, 100);
		})
	}

	function watchForDisconnect(request) {
		request.blob().then(pollForReboot).catch(pollForReboot);
	}
})()
