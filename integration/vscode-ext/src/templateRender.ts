/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

import * as vscode from "vscode";
import { RunnerOutputType, RunnerSubcommandVariant } from "./runner";
import { RunnerState } from "./runnerState";

/**
 * Render the file template with name `templateName` into the file at path `outputPath`, prompting the
 * user for the values of any variables not already contained in the `knownVariables` map.
 * The file is created if it doesn't already exist.
 */
export async function renderFileTemplatePrompted(
    runnerState: RunnerState,
    templateName: string,
    outputPath: string,
    knownVariables?: Map<string, string> | undefined
): Promise<{stdout: string, stderr: string}> {
    // query devinit for remaining needed variables in template `templateName`
    let remainingVariables: string[];
    try {
        remainingVariables = await listFileTemplateVars(runnerState, templateName, knownVariables);
    } catch (e) {
        return Promise.reject(e);
    }

    // query the user to specify each variable as necessary
    let definedVariables = new Map<string, string>();
    for (const ident of remainingVariables) {
        const value = await vscode.window.showInputBox({
            title: `Define template variable \"${ident}\"`,
            placeHolder: `What is \"${ident}\" equal to?`
        });
        // early return if any variables are skipped (i.e. input cancelled)
        if (value === undefined) {
            return Promise.reject("Input cancelled");
        }
        definedVariables.set(ident, value);
    }

    console.log(definedVariables);

    return renderFileTemplate(
        runnerState,
        templateName,
        outputPath,
        new Map([...definedVariables, ...(knownVariables ?? new Map())]));
}

/**
 * Render the file template with name `templateName` into the file at path `outputPath`.
 * The file is created if it doesn't already exist.
 */
async function renderFileTemplate(
    runnerState: RunnerState,
    templateName: string,
    outputPath: string,
    variables: Map<string, string>
): Promise<{stdout: string, stderr: string}> {
    return runnerState
        .buildRunner()
        .setSubcommand(RunnerSubcommandVariant.File)
        .setOutputType(RunnerOutputType.ToPath)
        .setOutputPath(outputPath)
        .setTemplateName(templateName)
        .setVariableMap(variables)
        .run();
}

/**
 * Retrieve a list of the variables found in the template (file or project) with name `templateName`.
 */
async function listFileTemplateVars(
    runnerState: RunnerState,
    templateName: string,
    knownVariables?: Map<string, string> | undefined
): Promise<string[]> {
    let stdout, stderr;
    try {
        ({stdout, stderr} = await runnerState
            .buildRunner()
            .setSubcommand(RunnerSubcommandVariant.File)
            .setOutputType(RunnerOutputType.ListVars)
            .setTemplateName(templateName)
            .setVariableMap(knownVariables ?? new Map())
            .run());
    } catch (e) {
        // most likely the config couldn't be found
        return Promise.reject(e);
    }

    // stdout is in JSON format (because the --parsable option is passed to devinit)
    const obj = JSON.parse(stdout);

    return obj;
}
