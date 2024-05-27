/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

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
export function getAllFileTemplates(runnerState: RunnerState): FileTemplateDetail[] {
    return [
        // {
        //     name: "base",
        //     source: "/home/jack/Developer/Utilities/devinit/examples/templates/file/base"
        // },
        // {
        //     name: "python",
        //     source: "/home/jack/Developer/Utilities/devinit/examples/templates/file/python"
        // },
    ];
}
