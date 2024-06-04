/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

import * as vscode from "vscode";
import * as userConfig  from "./userConfig";
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
    skipDefaults: boolean,
    assertEmpty: boolean
): Promise<{stdout: string, stderr: string}> {
    // get user-set default variables for this template
    const defaultVariablesUntyped = userConfig.getDefaultVariableMaps().get(templateName);
    const defaultVariablesMap = (!skipDefaults && defaultVariablesUntyped !== undefined)
        ? new Map(Object.entries(defaultVariablesUntyped!))
        : new Map();

    // query devinit for remaining needed variables in template `templateName`
    let remainingVariables: string[];
    try {
        remainingVariables = await listFileTemplateVars(runnerState, templateName, defaultVariablesMap);
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

    return renderFileTemplate(
        runnerState,
        templateName,
        outputPath,
        new Map([...definedVariables, ...defaultVariablesMap]),
        assertEmpty
    );
}

/**
 * Render the file template with name `templateName` into the file at path `outputPath`.
 * The file is created if it doesn't already exist.
 */
async function renderFileTemplate(
    runnerState: RunnerState,
    templateName: string,
    outputPath: string,
    variables: Map<string, string>,
    assertEmpty: boolean
): Promise<{stdout: string, stderr: string}> {
    return runnerState
        .buildRunner()
        .setSubcommand(RunnerSubcommandVariant.File)
        .setOutputType(RunnerOutputType.ToPath)
        .setOutputPath(outputPath)
        .setTemplateName(templateName)
        .setVariableMap(variables)
        .setAssertEmpty(assertEmpty)
        .run();
}

/**
 * Retrieve a list of the variables found in the template (file or project) with name `templateName`.
 */
async function listFileTemplateVars(
    runnerState: RunnerState,
    templateName: string,
    knownVariables: Map<string, string>
): Promise<string[]> {
    let stdout, stderr;
    try {
        ({stdout, stderr} = await runnerState
            .buildRunner()
            .setSubcommand(RunnerSubcommandVariant.File)
            .setOutputType(RunnerOutputType.ListVars)
            .setTemplateName(templateName)
            .setVariableMap(knownVariables)
            .run());
    } catch (e) {
        // most likely the config couldn't be found
        return Promise.reject(e);
    }

    // stdout is in JSON format (because the --parsable option is passed to devinit)
    const obj = JSON.parse(stdout);

    return obj;
}
