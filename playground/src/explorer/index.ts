import { editor, languages } from "monaco-editor";
import { compileJSX } from "./compileJSX";

const baseEditorOptions: editor.IStandaloneEditorConstructionOptions = {
  theme: "vs-dark",
  tabSize: 2,
  fontSize: 14,
  wordWrap: "on",
  scrollBeyondLastLine: false,
  renderWhitespace: "selection",
  contextmenu: false,
  minimap: {
    enabled: false,
  },
};

function initTypescriptCompilerOptions() {
  const { typescriptDefaults, JsxEmit, ScriptTarget } = languages.typescript;

  typescriptDefaults.setCompilerOptions({
    allowJs: true,
    allowNonTsExtensions: true,
    jsx: JsxEmit.Preserve,
    target: ScriptTarget.Latest,
  });
}

export function init() {
  const sourceContainer = document.getElementById("source-editor")!;
  const outputContainer = document.getElementById("output-editor")!;

  initTypescriptCompilerOptions();

  const sourceEditor = editor.create(sourceContainer, {
    language: "typescript",
    value: "const App = () => <div>Hello World</div>",
    ...baseEditorOptions,
  });

  const outputEditor = editor.create(outputContainer, {
    language: "javascript",
    value: "",
    readOnly: true,
    ...baseEditorOptions,
  });

  sourceEditor.onDidChangeModelContent(() => {
    const src = sourceEditor.getValue();

    // TODO
    // const output = await compileJSX(src);
    const output = src;

    outputEditor.setValue(output);
  });
}
