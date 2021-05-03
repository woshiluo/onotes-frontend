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
	console.log(data);
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


 $( () => {
	 $('main a').each( (idx,value) => {
		 let href = $(value).html();
		 for( address of note_list ) {
			 // Test notes
			 let reg_note = new RegExp( address + "/post/(\\d+)$");
			 if( reg_note.test(href) ) {
				 $(value).html( 'Notes #' + href.match(reg_note)[1] );
				 return ;
			 }
		 }
		 for( address of git_list ) {
			 // Test repo
			 let reg_repo = new RegExp( address + "/([\\w-]+)/([\\w-]+)$");
			 if( reg_repo.test(href) ) {
				 let match_result = href.match(reg_repo);
				 $(value).html( `${match_result[1]}/${match_result[2]}` );
				 return ;
			 }
			 // Test Issue
			 let reg_issue = new RegExp( address + "/([\\w-]+)/([\\w-]+)/issues/(\\d+)$");
			 if( reg_issue.test(href) ) {
				 let match_result = href.match(reg_issue);
				 $(value).html( `${match_result[1]}/${match_result[2]} #${match_result[3]}` );
				 return ;
			 }
			 // Test PR
			 let reg_pull = new RegExp( address + "/([\\w-]+)/([\\w-]+)/pull/(\\d+)$");
			 if( reg_pull.test(href) ) {
				 let match_result = href.match(reg_pull);
				 $(value).html( `${match_result[1]}/${match_result[2]} PR#${match_result[3]}` );
				 return ;
			 }
		 }
	 });
 })
