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

export function getDefaultVariableMaps(): Map<string, Map<string, string>> {
    return new Map(Object.entries(getConfig().get("devinit.automation.defaultTemplateVariables")!));
}

function getConfig() {
    return vscode.workspace.getConfiguration();
}
