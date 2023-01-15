(function () {
  const vscode = acquireVsCodeApi();

  const resolveSyntaxCB = document.lastChild.querySelector(
    'input[name="_snippets_resolve_syntax"]'
  );
  resolveSyntaxCB.checked = resolveSyntaxCB.value === "true" ? "checked" : "";

  document.querySelector("form").addEventListener("submit", (e) => {
    e.preventDefault();
    const form = document.querySelector('form[name="edit-snippet-form"]');
    const snippetLabel = form.elements["snippet-label"].value;
    const snippetContent = form.elements["snippet-content"].value;
    const snippetDescription = form.elements["snippet-description"].value;
    const snippetRequires = form.elements["snippet-requires"].value;
    const resolveSyntax = form.elements["snippet-resolveSyntax"].checked;

    vscode.postMessage({
      data: {
        label: snippetLabel,
        content: snippetContent,
        description: snippetDescription,
        requires: snippetRequires,
        resolveSyntax: resolveSyntax,
      },
      command: "edit-snippet",
    });
  });

  function detectSnippetSyntax(element) {
    const regex = "\\${?\\w+(:?\\S*)}?";
    let re = new RegExp(regex);
    var res = element.value.search(re);
    div.style.display = res < 0 ? "none" : "inline";
  }

  const div = document.lastChild.querySelector('div[name="_snippets_syntax"]');
  document
    .querySelector('textarea[name="snippet-content"]')
    .addEventListener("keyup", (e) => {
      detectSnippetSyntax(e.target);
    });

  detectSnippetSyntax(
    document.querySelector('textarea[name="snippet-content"]')
  );
})();
