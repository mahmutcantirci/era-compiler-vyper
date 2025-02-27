//!
//! Vyper compiler arguments.
//!

use std::path::Path;
use std::path::PathBuf;

use path_slash::PathExt;
use structopt::StructOpt;

///
/// Pythonic Smart Contract Language for the EraVM.
///
/// Example: `zkvyper ERC20.vy`
///
#[derive(Debug, StructOpt)]
#[structopt(
    name = "The EraVM Vyper compiler",
    global_settings = &[structopt::clap::AppSettings::ArgRequiredElseHelp],
)]
pub struct Arguments {
    /// Print the version and exit.
    #[structopt(long = "version")]
    pub version: bool,

    /// Specify the input file paths.
    /// Multiple Vyper files can be passed in the default Vyper mode.
    /// LLVM IR mode currently supports only a single file.
    #[structopt(parse(from_os_str))]
    pub input_files: Vec<PathBuf>,

    /// Create one file per component and contract/file at the specified directory, if given.
    #[structopt(short = "o", long = "output-dir")]
    pub output_directory: Option<PathBuf>,

    /// Overwrite existing files (used together with -o).
    #[structopt(long = "overwrite")]
    pub overwrite: bool,

    /// Set the optimization parameter -O[0 | 1 | 2 | 3 | s | z].
    /// Use `3` for best performance and `z` for minimal size.
    #[structopt(short = "O", long = "optimization")]
    pub optimization: Option<char>,

    /// Try to recompile with -Oz if the bytecode is too large.
    #[structopt(long = "fallback-Oz")]
    pub fallback_to_optimizing_for_size: bool,

    /// Disable the system request memoization.
    #[structopt(long = "disable-system-request-memoization")]
    pub disable_system_request_memoization: bool,

    /// Set the jump table density threshold.
    #[structopt(long = "jump-table-density-threshold")]
    pub jump_table_density_threshold: Option<u32>,

    /// Disable the `vyper` LLL IR optimizer.
    #[structopt(long = "disable-vyper-optimizer")]
    pub disable_vyper_optimizer: bool,

    /// Specify the path to the `vyper` executable. By default, the one in `${PATH}` is used.
    /// In LLVM IR mode `vyper` is unused.
    #[structopt(long = "vyper")]
    pub vyper: Option<String>,

    /// The EVM version to generate IR for.
    /// See https://github.com/matter-labs/era-compiler-common/blob/main/src/evm_version.rs for reference.
    #[structopt(long = "evm-version")]
    pub evm_version: Option<String>,

    /// An extra output format string.
    /// See `vyper --help` for available options including combined JSON mode.
    #[structopt(short = "f")]
    pub format: Option<String>,

    /// Switch to LLVM IR mode.
    /// Only one input LLVM IR file is allowed.
    /// Cannot be used with combined JSON mode.
    /// Use this mode at your own risk, as LLVM IR input validation is not implemented.
    #[structopt(long = "llvm-ir")]
    pub llvm_ir: bool,

    /// Switch to EraVM assembly mode.
    /// Only one input EraVM assembly file is allowed.
    /// Cannot be used with combined JSON modes.
    /// Use this mode at your own risk, as EraVM assembly input validation is not implemented.
    #[structopt(long = "zkasm")]
    pub zkasm: bool,

    /// Set metadata hash mode: `keccak256` | `none`.
    /// `keccak256` is enabled by default.
    #[structopt(long = "metadata-hash")]
    pub metadata_hash: Option<String>,

    /// Dump all IR (LLL, LLVM IR, assembly) to files in the specified directory.
    /// Only for testing and debugging.
    #[structopt(long = "debug-output-dir")]
    pub debug_output_directory: Option<PathBuf>,

    /// Suppress specified warnings.
    /// Available arguments: `ecrecover`, `extcodesize`, `txorigin`.
    #[structopt(long = "suppress-warnings")]
    pub suppress_warnings: Option<Vec<String>>,

    /// Set the `verify-each` option in LLVM.
    /// Only for testing and debugging.
    #[structopt(long = "llvm-verify-each")]
    pub llvm_verify_each: bool,

    /// Set the `debug-logging` option in LLVM.
    /// Only for testing and debugging.
    #[structopt(long = "llvm-debug-logging")]
    pub llvm_debug_logging: bool,

    /// Run this process recursively and provide JSON input to compile a single contract.
    /// Only for usage from within the compiler.
    #[structopt(long = "recursive-process")]
    pub recursive_process: bool,
}

impl Default for Arguments {
    fn default() -> Self {
        Self::new()
    }
}

impl Arguments {
    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self::from_args()
    }

    ///
    /// Validates the arguments.
    ///
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.version && std::env::args().count() > 2 {
            anyhow::bail!("No other options are allowed while getting the compiler version.");
        }

        if self.recursive_process && std::env::args().count() > 2 {
            anyhow::bail!("No other options are allowed in recursive mode.");
        }

        let modes_count = [self.llvm_ir, self.zkasm, self.format.is_some()]
            .iter()
            .filter(|&&x| x)
            .count();
        if modes_count > 1 {
            anyhow::bail!(
                "Only one modes is allowed at the same time: Vyper, LLVM IR, EraVM assembly."
            );
        }

        if self.llvm_ir || self.zkasm {
            if self.vyper.is_some() {
                anyhow::bail!("`vyper` is not used in LLVM IR and EraVM assembly modes.");
            }

            if self.evm_version.is_some() {
                anyhow::bail!("EVM version is not used in LLVM IR and EraVM assembly modes.");
            }
        }

        if self.zkasm {
            if self.optimization.is_some() {
                anyhow::bail!("LLVM optimizations are not supported in EraVM assembly mode.");
            }

            if self.fallback_to_optimizing_for_size {
                anyhow::bail!("Falling back to -Oz is not supported in EraVM assembly mode.");
            }
            if self.disable_system_request_memoization {
                anyhow::bail!(
                    "Disabling the system request memoization is not supported in EraVM assembly mode."
                );
            }
            if self.jump_table_density_threshold.is_some() {
                anyhow::bail!(
                    "Setting the jump table density threshold is not supported in EraVM assembly mode."
                );
            }
        }

        Ok(())
    }

    ///
    /// Normalizes input paths by converting it to POSIX format.
    ///
    pub fn normalize_input_paths(&mut self) -> anyhow::Result<()> {
        for input_path in self.input_files.iter_mut() {
            *input_path = Self::path_to_posix(input_path.as_path())?;
        }
        Ok(())
    }

    ///
    /// Normalizes an input path by converting it to POSIX format.
    ///
    fn path_to_posix(path: &Path) -> anyhow::Result<PathBuf> {
        let path = path
            .to_slash()
            .ok_or_else(|| anyhow::anyhow!("Input path {:?} POSIX conversion error", path))?
            .to_string();
        let path = PathBuf::from(path.as_str());
        Ok(path)
    }
}
