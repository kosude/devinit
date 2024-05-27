/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

import * as vscode from "vscode";
import { RunnerState } from "./runnerState";
import * as commands from "./commands";

/**
 * Static extension activation set-up function
 * @param context VS Code extension API context
 */
export async function activate(context: vscode.ExtensionContext) {
    // initialise runner state; update it if user configuration changes
    const runnerState = new RunnerState();
    vscode.workspace.onDidChangeConfiguration(_ => {
        runnerState.updateUserConfigProperties();
    });

    context.subscriptions.push(
        vscode.commands.registerCommand("devinit.render-file-template", () => commands.renderFileTemplate(runnerState))
    );
}
