import {executeCommand, isDestinationExist, isFileEmpty, createTmpDirectory, pathToVyBinOutputFile, pathToVyAsmOutputFile, createFiles} from "../src/helper";
import { paths } from '../src/entities';
import * as os from 'os';

if (os.platform() !== 'win32') { //bugs on windows
    describe("Overwrite output dir", () => {
        const zkvyperCommand = 'zkvyper';

        //id1983
        describe("Default run with --overwrite output dir", () => {
            const tmpDirZkVyper = createTmpDirectory();

            it("Output dir is created", () => {
                expect(isDestinationExist(tmpDirZkVyper.name)).toBe(true);
            });

            //adding empty files to tmp dir
            createFiles(tmpDirZkVyper.name, [`${paths.contractVyFilename}${paths.binExtension}`, `${paths.contractVyFilename}${paths.asmExtension}`])

            //trying to run a command to get a warning and verify an exit code
            const pre_args = [`"${paths.pathToBasicVyContract}"`, `-o`, `"${tmpDirZkVyper.name}"`]; // issue on windows
            const pre_result = executeCommand(zkvyperCommand, pre_args);

            it("Refusing to overwrite in the output", () => {
                expect(pre_result.output).toMatch(/(Refusing to overwrite)/i);
            });

            //trying to add a flag and verify that command passed with 0 exit code
            const args = [`"${paths.pathToBasicVyContract}"`, `-o`, `"${tmpDirZkVyper.name}"`, `--overwrite`]; // issue on windows
            const result = executeCommand(zkvyperCommand, args);

            it("Exit code = 0", () => {
                expect(result.exitCode).toBe(0);
            });

            //verify that files are not empty
            it("The output files are not empty", () => {
                // Remove if () {} after the bugfix on win
                if (os.platform() === 'win32') {
                    const args_cmd = [`"${paths.pathToVyBinOutputFile}"`];
                    console.log(`The output file: ${pathToVyBinOutputFile(tmpDirZkVyper.name)} contains: \n`
                        + executeCommand('type', [pathToVyBinOutputFile(tmpDirZkVyper.name)]).output);
                    console.log(`The output file should contain: \n`
                        + executeCommand(zkvyperCommand, args_cmd).output);
                } else {
                    expect(isFileEmpty(pathToVyBinOutputFile(tmpDirZkVyper.name))).toBe(false);
                    expect(isFileEmpty(pathToVyAsmOutputFile(tmpDirZkVyper.name))).toBe(false);
                }
            });

            it("No 'Error'/'Warning'/'Fail' in the output", () => {
                expect(result.output).not.toMatch(/([Ee]rror|[Ww]arning|[Ff]ail)/i);
                tmpDirZkVyper.removeCallback()
            });

        });

    });
}
