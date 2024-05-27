/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

import * as vscode from "vscode";

export function getExecutablePath(): string {
    return getConfig().get<string>("devinit.environment.executablePath")!;
}

export function getConfigPath(): string {
    return getConfig().get<string>("devinit.environment.configurationFile")!;
}

function getConfig() {
    return vscode.workspace.getConfiguration();
}
