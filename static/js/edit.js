import init, { parse_markdown } from '/static/libs/wasm-markdown-0.0.1/wasm_markdown.js';
init();
async function parse(x) {
 	await init();
	return parse_markdown(x);
}

ace.config.set('basePath', '/static/libs/ace-builds/src-min-noconflict');

var editor_title = ace.edit("editor-title", {
	theme: "ace/theme/tomorrow",
	maxLines: 200,
	fontSize: 16,
	wrap: true,
	autoScrollEditorIntoView: true
});
var editor_markdown = ace.edit("editor-markdown", {
	theme: "ace/theme/tomorrow",
	mode: "ace/mode/markdown",
	maxLines: 50,
	fontSize: 16,
	wrap: true,
	autoScrollEditorIntoView: true
});
var editor_from = ace.edit("editor-from", {
	theme: "ace/theme/tomorrow",
	mode: "ace/mode/json",
	maxLines: 1,
	wrap: true,
	autoScrollEditorIntoView: true
});
var editor_to = ace.edit("editor-to", {
	theme: "ace/theme/tomorrow",
	mode: "ace/mode/json",
	maxLines: 1,
	wrap: true,
	autoScrollEditorIntoView: true
});

var id=post_id;

$('#editor-submit').on('click', async function(e) {
	let title = editor_title.getValue();
	let markdown = editor_markdown.getValue();
	let to_list = editor_to.getValue();
	let from_list = editor_from.getValue();

	let post_await = request(`/api/post/${id}`, { title: title, markdown: markdown, });
	let to_await = request( `/api/post/${id}/to`, JSON.parse(to_list) );
	let from_await = request( `/api/post/${id}/from`, JSON.parse(from_list) );
	await post_await; await to_await; await from_await;

	mdui.snackbar({
		message: '更新完成',
		buttonText: '刷新',
		onButtonClick: function(){
			window.location.reload();
		},
	});
});

$('#preview-markdown').on('click', async function(e) {
	let markdown = await parse( editor_markdown.getValue() );
	mdui.dialog( { title: '预览', content: markdown, cssClass: 'mdui-typo' } );
	MathJax.startup.defaultPageReady();
	Prism.highlightAll();
});
