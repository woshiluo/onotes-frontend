 $(function () {
	 $('#online-status').html('<i class="mdui-icon material-icons">cloud_off</i>');
	 var inst = new mdui.Tooltip('#online-status', {
		   content: '当前离线，页面为缓存'
	 });
 });
