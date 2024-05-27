/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

import * as vscode from "vscode";
import { RunnerState } from "./runnerState";

/**
 * Static extension activation set-up function
 * @param context VS Code extension API context
 */
export async function activate(context: vscode.ExtensionContext) {
    // initialise runner state; update it if user configuration changes
    const runnerState = new RunnerState();
    vscode.workspace.onDidChangeConfiguration(_ => {
        runnerState.updateUserConfigProperties();
    });

    context.subscriptions.push(
        vscode.commands.registerCommand("devinit.render-template", () => {
            console.log(runnerState.buildRunner().run());


            // let runner = new Runner()
            //     .setConfigPath("/home/jack/Developer/Utilities/devinit/examples/devinitrc.yml")
            //     .setExecPath("/home/jack/Developer/Utilities/devinit/build/devinit")
            //     .setSubcommand(RunnerSubcommandVariant.List)
            // console.log(
            //     runner.run((stdout, stderr) => {
            //         // command returned succesfully
            //         vscode.window.showInformationMessage(stdout);
            //         vscode.window.showInformationMessage(stderr);
            //     }, (error) => {
            //         // command returned with errors
            //         vscode.window.showErrorMessage(`${error}`);
            //     })
            // );



            // runner = new Runner()
            //     .setConfigPath("/home/jack/Developer/Utilities/devinit/examples/devinitrc.yml")
            //     .setExecPath("/home/jack/Developer/Utilities/devinit/build/devinit")
            //     .setSubcommand(RunnerSubcommandVariant.File)
            //     .setOutputType(RunnerOutputType.ListVars)
            //     .setTemplateName("c_cpp_header")
            // console.log(
            //     runner.run((stdout, stderr) => {
            //         // command returned succesfully
            //         vscode.window.showInformationMessage(stdout);
            //         vscode.window.showInformationMessage(stderr);
            //     }, (error) => {
            //         // command returned with errors
            //         vscode.window.showErrorMessage(`${error}`);
            //     })
            // );
        }),
    );
}
