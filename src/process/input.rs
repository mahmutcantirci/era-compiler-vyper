//!
//! Process for compiling a single compilation unit.
//!
//! The input data.
//!

use std::borrow::Cow;

use serde::Deserialize;
use serde::Serialize;

use crate::project::contract::Contract;
use crate::warning_type::WarningType;

///
/// The input data.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Input<'a> {
    /// The contract full path.
    pub full_path: Cow<'a, String>,
    /// The contract representation.
    pub contract: Cow<'a, Contract>,
    /// The source code hash.
    pub source_code_hash: Option<[u8; era_compiler_common::BYTE_LENGTH_FIELD]>,
    /// Enables the test bytecode encoding.
    pub enable_test_encoding: bool,
    /// The EVM target version.
    pub evm_version: Option<era_compiler_common::EVMVersion>,
    /// The optimizer settings.
    pub optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    /// The suppressed warnings.
    pub suppressed_warnings: Vec<WarningType>,
    /// The debug output config.
    pub debug_config: Option<era_compiler_llvm_context::DebugConfig>,
}

impl<'a> Input<'a> {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        full_path: Cow<'a, String>,
        contract: Cow<'a, Contract>,
        source_code_hash: Option<[u8; era_compiler_common::BYTE_LENGTH_FIELD]>,
        enable_test_encoding: bool,
        evm_version: Option<era_compiler_common::EVMVersion>,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        suppressed_warnings: Vec<WarningType>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> Self {
        Self {
            full_path,
            contract,
            source_code_hash,
            enable_test_encoding,
            evm_version,
            optimizer_settings,
            suppressed_warnings,
            debug_config,
        }
    }
}
