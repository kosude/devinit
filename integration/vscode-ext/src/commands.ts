/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

import * as vscode from "vscode";
import { RunnerState } from "./runnerState";
import { getAllFileTemplates } from "./getTemplates";

/**
 * Render a template into the current file
 */
export async function renderFileTemplate(runnerState: RunnerState) {
    // find available file templates and convert them to VSCode QuickPickItem objects
    const availableTemplates = getAllFileTemplates(runnerState)
        .map((x) => {
                return {
                    label: x.name,
                    description: x.source
                } as vscode.QuickPickItem;
            });

    const templateName = await vscode.window.showQuickPick(availableTemplates, {
        title: "Choose from available file templates",
        placeHolder: "Select a file template to render"
    });

    // return early if input was cancelled
    if (templateName === undefined) {
        return;
    }

    vscode.window.showInformationMessage(templateName.label);
}
