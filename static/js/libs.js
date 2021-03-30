if ('serviceWorker' in navigator) {
  navigator.serviceWorker.register('/sw.js').then(function(reg) {
    // registration worked
    console.log('Registration succeeded. Scope is ' + reg.scope);
  }).catch(function(error) {
    // registration failed
    console.log('Registration failed with ' + error);
  });
}

var $ = mdui.$;

function alert_err(err) {
	mdui.alert(`更新错误！${JSON.stringify(err)}`);
}

function run_result(data) {
	console.log(typeof(data));
	if( typeof(data.Ok) === typeof(undefined) ) {
		alert_err(data.Err);
	}
}

async function request( url, data, method='POST' ) {
	let result = await $.ajax({
		method: method,
		url: url,
		contentType: 'application/json',
		dataType: 'json',
		data: JSON.stringify({
			auth: { Token: [ Number(docCookies.getItem('uid')), docCookies.getItem('token') ] },
			data: data,
		}),
		success: function (data) {
			run_result(data);
		},
		error: function(err) {
			alert_err(err);
		}
	});

	return result.Ok;
}
