/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

import { RunnerOutputType, RunnerSubcommandVariant } from "./runner";
import { RunnerState } from "./runnerState";

/**
 * Render the file template with name `templateName` into the file at path `outputPath`.
 * The file is created if it doesn't already exist.
 */
export async function renderFileTemplateCli(
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
export async function renderTemplateListVariablesCli(
    runnerState: RunnerState,
    templateName: string
): Promise<string[]> {
    let stdout, stderr;
    try {
        ({stdout, stderr} = await runnerState
            .buildRunner()
            .setSubcommand(RunnerSubcommandVariant.File)
            .setOutputType(RunnerOutputType.ListVars)
            .setTemplateName(templateName)
            .run());
    } catch (e) {
        // most likely the config couldn't be found
        return Promise.reject(e);
    }

    // stdout is in JSON format (because the --parsable option is passed to devinit)
    const obj = JSON.parse(stdout);

    return obj;
}
