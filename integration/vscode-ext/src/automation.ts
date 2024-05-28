/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

import * as vscode from "vscode";
import * as userConfig  from "./userConfig";

/**
 * Class to handle automation of template expansion and rendering (e.g. autoamtically rendering associated
 * templates into new files)
 */
export class Automator {
    /**
     * Map of template names indexed by filename globs.
     */
    private templateAssociations!: Map<string, string>;

    /**
     * Array of FS watchers to listen to workspace file events
     */
    private watchers!: vscode.FileSystemWatcher[];

    /**
     * Initialise the automator (automation state) object
     */
    public constructor() {
        this.updateUserConfigProperties();
    }

    /**
     * Update the properties in the automator which are based on the user's VS Code configuration.
     * This should be done when config changes (via listening to `vscode.workspace.onDidChangeConfiguration()`)
     */
    public updateUserConfigProperties() {
        this.templateAssociations = userConfig.getTemplateAssociations();

        // dispose previous watchers
        if (this.watchers !== undefined) {
            this.watchers.forEach((w) => w.dispose());
        }

        // create watchers for each association
        this.watchers = [];
        Object.entries(this.templateAssociations).forEach(([pat, tpl]) =>  {
            let w = vscode.workspace.createFileSystemWatcher(`**/${pat}`);
            w.onDidCreate(
                (uri) => this.onFileCreated(uri, tpl)
            );

            this.watchers.push(w);
        });
    }

    /**
     * This function is called every time a file with an associated template is created within the workspace
     * @param uri File path
     * @param templateName Associated template name
     */
    private onFileCreated(uri: vscode.Uri, templateName: string) {
        // TODO: render the template into the new file
        console.log(`Created ${uri}: using template "${templateName}"`);
    }
}
