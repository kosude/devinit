/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

import * as vscode from "vscode";
import * as userConfig  from "./userConfig";
import { renderFileTemplatePrompted } from "./templateRender";
import { RunnerState } from "./runnerState";

/**
 * Class to handle automation of template expansion and rendering (e.g. autoamtically rendering associated
 * templates into new files)
 */
export class Automator {
    private readonly runnerState: RunnerState;

    /**
     * Array of FS watchers to listen to workspace file events
     */
    private watchers!: vscode.FileSystemWatcher[];

    /**
     * Initialise the automator (automation state) object
     */
    public constructor(runnerState: RunnerState) {
        this.runnerState = runnerState;

        this.updateUserConfigProperties();
    }

    /**
     * Update the properties in the automator which are based on the user's VS Code configuration.
     * This should be done when config changes (via listening to `vscode.workspace.onDidChangeConfiguration()`)
     */
    public updateUserConfigProperties() {
        // dispose previous watchers
        if (this.watchers !== undefined) {
            this.watchers.forEach((w) => w.dispose());
        }

        // create watchers for each association
        this.watchers = [];
        Object.entries(userConfig.getTemplateAssociations()).forEach(([pat, tpl]) =>  {
            let w = vscode.workspace.createFileSystemWatcher(`**/${pat}`);
            w.onDidCreate(
                async (uri) => {
                    await this.onFileCreated(uri, tpl);
                }
            );

            this.watchers.push(w);
        });
    }

    /**
     * This function is called every time a file with an associated template is created within the workspace
     * @param uri File path
     * @param templateName Associated template name
     */
    private async onFileCreated(uri: vscode.Uri, templateName: string) {
        try {
            await renderFileTemplatePrompted(
                this.runnerState,
                templateName,
                uri.fsPath,
                false,
                true
            );
        } catch (e) {
            if (e !== "Input cancelled") {
                // TODO: implement --parsable version of errors in the devinit CLI, and then parse it and print it here for better readability.
                vscode.window.showErrorMessage(`Error when rendering template \"${templateName}\": ${e}`);
            }
        }
    }
}
