import * as path from "path";
import {
  ExtensionContext,
  workspace,
} from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from "vscode-languageclient/node";

let client: LanguageClient | undefined;

export function activate(context: ExtensionContext) {
  const config = workspace.getConfiguration("itp");
  const enableLsp = config.get<boolean>("enableLsp", true);
  if (!enableLsp) {
    return;
  }

  const lspPath = config.get<string>("lspPath", "itp-lsp");

  const serverOptions: ServerOptions = {
    run: { command: lspPath, transport: TransportKind.stdio },
    debug: { command: lspPath, transport: TransportKind.stdio },
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "aql" }],
    synchronize: {
      fileEvents: workspace.createFileSystemWatcher("**/*.aql"),
    },
  };

  client = new LanguageClient(
    "itp-lsp",
    "ITP Language Server",
    serverOptions,
    clientOptions
  );

  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
