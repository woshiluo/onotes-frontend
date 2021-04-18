this.addEventListener('install', function(event) {
	event.waitUntil(
		caches.open('staticfiles-v2').then(function(cache) {
			return cache.addAll([
				'/static/libs/mdui-1.0.2/js/mdui.min.js',
				'/static/libs/mdui-1.0.2/css/mdui.min.css',
				'/static/libs/mdui-1.0.2/fonts/roboto/Roboto-Medium.woff2',
				'/static/libs/mdui-1.0.2/fonts/roboto/Roboto-Regular.woff2',
				'/static/libs/mdui-1.0.2/icons/material-icons/MaterialIcons-Regular.woff2',
				'/static/libs/wasm-markdown-0.0.1/wasm_markdown.js',
				'/static/libs/wasm-markdown-0.0.1/wasm_markdown_bg.wasm',
				'/static/libs/MathJax/es5/tex-svg.js',
				'/static/libs/ace-builds/ace.js',
				'/static/libs/ace-builds/worker-json.js',
				'/static/libs/ace-builds/theme-tomorrow.js',
				'/static/libs/ace-builds/mode-markdown.js',
				'/static/libs/ace-builds/mode-json.js',
				'/static/style.css',
				'/static/js/libs.js',
				'/static/js/cookies.js',
				'/static/js/edit.js',
				'/static/js/offline.js'
			]);
		})
	);
});

self.addEventListener('fetch', event => {
	event.respondWith( async function() {
		let reg_static = new RegExp("/*static*/");
		if( reg_static.test(event.request.url) ) {
			return caches.match(event.request).then( response => {
				return response || fetch(event.request).catch( () => caches.match('/static/js/offline.js') );
			})
		}
		return fetch(event.request).then( response => {
			return caches.open('cachefiles-v2').then( cache => {
				cache.put(event.request, response.clone());
				return response;
			});
		}).catch( () => {
			return caches.match(event.request) 
		});
	}());
});

self.addEventListener('activate', function(event) {
	var cacheWhitelist = ['cachefiles-v2', 'staticfiles-v2'];

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
