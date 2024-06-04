/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

import * as vscode from "vscode";
import { RunnerState } from "./runnerState";
import { getAllFileTemplatesCli } from "./templateGet";
import { renderFileTemplatePrompted } from "./templateRender";

/**
 * Render a template into the current file
 */
export async function renderFileTemplate(runnerState: RunnerState, skipDefaults?: boolean) {
    // get active document path (to render to)
    const activePath = getCurrentFilePath();
    if (activePath === undefined) {
        vscode.window.showErrorMessage("No text editor is currently active");
        return;
    }

    // query devinit for available templates
    let availableTemplates;
    try {
        availableTemplates = await getAllFileTemplatesCli(runnerState);
    } catch (e) {
        vscode.window.showErrorMessage(String(e));
        return;
    }

    // quick pick - map each available template to a QuickPickItem object
    const templateName = await vscode.window.showQuickPick(
        availableTemplates.map(
            (x) => {
                return {
                    label: x.name,
                    description: x.source
                } as vscode.QuickPickItem;
            }
        ), {
            title: "Choose from available file templates",
            placeHolder: "Select a file template to render"
        }
    );
    if (templateName === undefined) {
        return;
    }

    // attempt to render
    try {
        await renderFileTemplatePrompted(
            runnerState,
            templateName.label,
            activePath,
            skipDefaults ?? false,
            false
        );
    } catch (e) {
        if (e !== "Input cancelled") {
            // TODO: implement --parsable version of errors in the devinit CLI, and then parse it and print it here for better readability.
            vscode.window.showErrorMessage(`Error when rendering template \"${templateName.label}\": ${e}`);
        }
    }
}

/**
 * Get the absolute path to the currently active document, or undefined if no text document is open.
 * @returns Absolute file path as a string, or undefined if not applicable
 */
function getCurrentFilePath(): string | undefined {
    return vscode.window.activeTextEditor?.document.uri.fsPath;
}
