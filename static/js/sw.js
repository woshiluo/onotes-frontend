this.addEventListener('install', function(event) {
	event.waitUntil(
		caches.open('staticfiles-v3').then(function(cache) {
			return cache.addAll([
				'/static/libs/mdui/dist/js/mdui.min.js',
				'/static/libs/mdui/dist/css/mdui.min.css',
				'/static/libs/mdui/dist/fonts/roboto/Roboto-Medium.woff2',
				'/static/libs/mdui/dist/fonts/roboto/Roboto-Regular.woff2',
				'/static/libs/mdui/dist/icons/material-icons/MaterialIcons-Regular.woff2',

				'/static/libs/wasm-markdown-0.0.1/wasm_markdown.js',
				'/static/libs/wasm-markdown-0.0.1/wasm_markdown_bg.wasm',

				'/static/libs/MathJax/es5/tex-svg.js',

				'/static/libs/ace-builds/src-min-noconflict/ace.js',
				'/static/libs/ace-builds/src-min-noconflict/worker-json.js',
				'/static/libs/ace-builds/src-min-noconflict/theme-tomorrow.js',
				'/static/libs/ace-builds/src-min-noconflict/mode-markdown.js',
				'/static/libs/ace-builds/src-min-noconflict/mode-json.js',

				'/static/libs/prism/prism.js',
				'/static/libs/prism/components/prism-core.min.js',
				'/static/libs/prism/plugins/line-numbers/prism-line-numbers.min.js',
				'/static/libs/prism/plugins/autoloader/prism-autoloader.min.js',
				'/static/libs/prism/themes/prism.css',
				'/static/libs/prism/plugins/line-numbers/prism-line-numbers.css',

				'/static/style.css',
				'/static/js/libs.js',
				'/static/js/config.js',
				'/static/js/cookies.js',
				'/static/js/edit.js',
				'/static/js/offline.js'
			]);
		})
	);
});

function put_into_cache( cache_name, request, response ) {
	return caches.open(cache_name).then( cache => {
		cache.put(request, response.clone());
		return response;
	});
}

self.addEventListener('fetch', event => {
	event.respondWith( async function() {
		let reg_static = /static/;

		if( reg_static.test(event.request.url) ) {
			let reg_online = /static\/js\/online\.js/;
			if( reg_online.test( event.request.url ) ) {
				return fetch(event.request).catch( () => caches.match( "/static/js/offline.js" ) );
			}
			return caches.match(event.request).then( response => {
				return response || fetch(event.request)
					.then( 
						response => put_into_cache('staticfiles-v3', event.request, response) 
					).catch( 
						() => caches.match(event.request)
					);
			});
		}

		return fetch(event.request).then( 
			response => put_into_cache('cachefiles-v3', event.request, response) 
		).catch( () => caches.match(event.request) );
	}());
});

self.addEventListener('activate', function(event) {
	var cacheWhitelist = ['cachefiles-v3', 'staticfiles-v3'];

	event.waitUntil(
		caches.keys().then(function(keyList) {
			return Promise.all(keyList.map(function(key) {
				if (cacheWhitelist.indexOf(key) === -1) {
					return caches.delete(key);
				}
			}));
		})
	);
});
