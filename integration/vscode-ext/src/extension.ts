/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

import * as vscode from "vscode";
import { RunnerState } from "./runnerState";
import * as commands from "./commands";
import { Automator } from "./automation";

/**
 * Static extension activation set-up function
 * @param context VS Code extension API context
 */
export async function activate(context: vscode.ExtensionContext) {
    // runner state for devinit CLI execution
    const runnerState = new RunnerState();

    // automation state for file system watching
    const automator = new Automator();

    // update config-based state when user configuration changes
    vscode.workspace.onDidChangeConfiguration(_ => {
        runnerState.updateUserConfigProperties();
        automator.updateUserConfigProperties();
    });

    context.subscriptions.push(
        vscode.commands.registerCommand("devinit.render-file-template", () => commands.renderFileTemplate(runnerState))
    );
}
