/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

import { RunnerSubcommandVariant } from "./runner";
import { RunnerState } from "./runnerState";

/**
 * Definition of an object containing details about a retrieved file template.
 */
export interface FileTemplateDetail {
    name: string,
    source: string
}

/**
 * Invoke `devinit list` to retrieve a list of all file templates.
 * @return Array of all available file templates
 */
export async function getAllFileTemplates(runnerState: RunnerState): Promise<FileTemplateDetail[]> {
    let stdout, stderr;
    try {
        ({stdout, stderr} = await runnerState
            .buildRunner()
            .setSubcommand(RunnerSubcommandVariant.List)
            .run());
    } catch (e) {
        // most likely the config couldn't be found
        return Promise.reject(e);
    }

    // stdout is in JSON format (because the --parsable option is passed to devinit)
    const obj = JSON.parse(stdout);

    return obj["file"];
}
