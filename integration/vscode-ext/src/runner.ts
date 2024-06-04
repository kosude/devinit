/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

import * as child_process from "node:child_process";
import * as util from "node:util";

/**
 * Variants of devinit subcommands
 */
export enum RunnerSubcommandVariant {
    File = "file",
    Project = "project",
    List = "list",
}

/**
 * Types of output supported by devinit
 */
export enum RunnerOutputType {
    ToPath,
    DryRun,
    ListVars,
}

/**
 * Class to abstract devinit command execution.
 */
export class Runner {
    /**
     * Run the command with configured options.
     * `then` is run after a succesful execution completes, otherwise `err` is run instead.
     * @returns The executed command as a verbatim string, for diagnostics
     */
    public run(): Promise<{ stdout: string, stderr: string }> {
        const exec = util.promisify(child_process.exec);
        return exec(`"${this.execPath}" ${this.buildArgs().join(" ")}`);
    }

    /**
     * Build a list of arguments to pass to `node: child_process.execFile()`.
     * @returns Arguments built from class values
     */
    private buildArgs(): string[] {
        let args = [];

        if (this.configPath !== undefined) {
            args.push(`--config="${this.configPath}"`);
        }

        args.push("--parsable");

        args.push(this.subcmdVariant);

        if (this.subcmdVariant === RunnerSubcommandVariant.List) {
            return args;
        }

        switch (this.outputType) {
            case RunnerOutputType.ToPath:
                args.push(`--path="${this.outputPath ?? ""}"`);
                break;
            case RunnerOutputType.DryRun:
                args.push("--dry-run");
                break;
            case RunnerOutputType.ListVars:
                args.push("--list-vars");
                break;
        }

        this.variables.forEach((val, key) => {
            args.push(`-D"${key}"="${val}"`)
        })

        if (this.assertEmpty) {
            args.push("--assert-empty")
        }

        args.push(`"${this.templateName ?? ""}"`);

        return args;
    }

    /**
     * Path to the `devinit` executable
     */
    private execPath: string = "";
    public setExecPath(path: string): Runner {
        this.execPath = path;
        return this;
    }

    /**
     * Path to the `devinitrc.yml` file to use - if undefined, use system defaults
     */
    private configPath: string | undefined;
    public setConfigPath(path: string): Runner {
        this.configPath = path;
        return this;
    }


    /**
     * Devinit subcommand to run
     */
    private subcmdVariant: RunnerSubcommandVariant = RunnerSubcommandVariant.File;
    public setSubcommand(subcommand: RunnerSubcommandVariant): Runner {
        this.subcmdVariant = subcommand;
        return this;
    }

    /**
     * The type of output to produce. Only applicable if:
     *  - `subcmdVariant` is **not** equal to `RunnerSubcommandVariant.List`
     */
    private outputType: RunnerOutputType = RunnerOutputType.ToPath;
    public setOutputType(type: RunnerOutputType): Runner {
        this.outputType = type;
        return this;
    }

    /**
     * The path to send output to. Only applicable if:
     *  - `subcmdVariant` is **not** equal to `RunnerSubcommandVariant.List`
     *  - `outputType` is equal to `RunnerOutputType.ToPath`
     */
    private outputPath: string | undefined;
    public setOutputPath(path: string): Runner {
        this.outputPath = path;
        return this;
    }

    /**
     * A map of variables, keyed by their identifier, to be rendered into the template. Only applicable if:
     *  - `subcmdVariant` is **not** equal to `RunnerSubcommandVariant.List`
     */
    private variables: Map<string, string> = new Map<string, string>();
    public setVariable(id: string, val: string): Runner {
        this.variables.set(id, val);
        return this;
    }
    public setVariableMap(map: Map<string, string>): Runner {
        this.variables = map;
        return this;
    }

    /**
     * The name of a template to render. Only applicable if:
     *  - `subcmdVariant` is **not** equal to `RunnerSubcommandVariant.List`
     */
    private templateName: string | undefined;
    public setTemplateName(name: string): Runner {
        this.templateName = name;
        return this;
    }

    /**
     * Whether or not to only template a file if it is empty, or if it doesn't exist.
     */
    private assertEmpty: boolean = false;
    public setAssertEmpty(val: boolean): Runner {
        this.assertEmpty = val;
        return this;
    }
}
