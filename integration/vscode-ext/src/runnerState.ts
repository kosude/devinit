/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

import { Runner } from "./runner";
import * as userConfig  from "./userConfig";

/**
 * A state object from which multiple command runners can be created to use shared configuration.
 */
export class RunnerState {
    /**
     * Path to `devinit` - may just be the term 'devinit' itself, if we are using PATH.
     */
    private execPath!: string;

    /**
     * Path to the `devinitrc.yml` config file to read, or undefined to use system defaults
     */
    private configPath: string | undefined;

    /**
     * Initialise the runner state object
     */
    public constructor() {
        this.updateUserConfigProperties();
    }

    /**
     * Update the properties in the runner state which are based on the user's VS Code configuration.
     * This should be done when config changes (via listening to `vscode.workspace.onDidChangeConfiguration()`)
     */
    public updateUserConfigProperties() {
        this.execPath = userConfig.getExecutablePath();

        let configPath = userConfig.getConfigPath();
        this.configPath = (configPath.length > 0) ? configPath : undefined;
    }

    /**
     * Build a Runner object from the shared configuration in this state object.
     * @returns New command runner instance
     */
    public buildRunner(): Runner {
        let runner = new Runner()
            .setExecPath(this.execPath);

        if (this.configPath !== undefined) {
            runner.setConfigPath(this.configPath);
        }

        return runner;
    }
}
