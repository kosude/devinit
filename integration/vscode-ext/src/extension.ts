/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

import * as vscode from "vscode";

/**
 * Static extension activation set-up function
 * @param context VS Code extension API context
 */
export async function activate(context: vscode.ExtensionContext) {
    context.subscriptions.push(
        vscode.commands.registerCommand(
            "devinit.render-template",
            () => {
                vscode.window.showInformationMessage("Rendering template (temp information message)")
            }
        ),
    )
}
