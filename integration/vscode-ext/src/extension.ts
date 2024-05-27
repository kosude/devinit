/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

import * as vscode from "vscode";
import { Runner, RunnerOutputType, RunnerSubcommandVariant } from "./runner";

/**
 * Static extension activation set-up function
 * @param context VS Code extension API context
 */
export async function activate(context: vscode.ExtensionContext) {
    context.subscriptions.push(
        vscode.commands.registerCommand("devinit.render-template", () => {
            let runner = new Runner()
                .setConfigPath("/home/jack/Developer/Utilities/devinit/examples/devinitrc.yml")
                .setExecPath("/home/jack/Developer/Utilities/devinit/build/devinit")
                .setSubcommand(RunnerSubcommandVariant.List)
            console.log(
                runner.run((stdout, stderr) => {
                    // command returned succesfully
                    vscode.window.showInformationMessage(stdout);
                    vscode.window.showInformationMessage(stderr);
                }, (error) => {
                    // command returned with errors
                    vscode.window.showErrorMessage(`${error}`);
                })
            );



            runner = new Runner()
                .setConfigPath("/home/jack/Developer/Utilities/devinit/examples/devinitrc.yml")
                .setExecPath("/home/jack/Developer/Utilities/devinit/build/devinit")
                .setSubcommand(RunnerSubcommandVariant.File)
                .setOutputType(RunnerOutputType.ListVars)
                .setTemplateName("c_cpp_header")
            console.log(
                runner.run((stdout, stderr) => {
                    // command returned succesfully
                    vscode.window.showInformationMessage(stdout);
                    vscode.window.showInformationMessage(stderr);
                }, (error) => {
                    // command returned with errors
                    vscode.window.showErrorMessage(`${error}`);
                })
            );
        }),
    );
}
