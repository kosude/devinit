/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

import * as vscode from "vscode";

/**
 * Get the path to the `devinit` executable
 */
export function getExecutablePath(): string {
    return getConfig().get("devinit.environment.executablePath")!;
}

/**
 * Get the path to the `devinitrc.yml` file
 */
export function getConfigPath(): string {
    return getConfig().get("devinit.environment.configurationFile")!;
}

/**
 * Get the map of glob patterns to template names
 */
export function getTemplateAssociations(): Map<string, string> {
    return getConfig().get("devinit.automation.templateAssociations")!;
}

function getConfig() {
    return vscode.workspace.getConfiguration();
}
