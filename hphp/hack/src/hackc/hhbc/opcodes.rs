// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

use crate::hhbc_ast::NUM_ACT_REC_CELLS;
use crate::AdataId;
use crate::BareThisOp;
use crate::ClassName;
use crate::ClassNum;
use crate::CollectionType;
use crate::ConstName;
use crate::ContCheckOp;
use crate::Dummy;
use crate::FCallArgs;
use crate::FatalOp;
use crate::FloatBits;
use crate::FunctionName;
use crate::IncDecOp;
use crate::InitPropOp;
use crate::IsLogAsDynamicCallOp;
use crate::IsTypeOp;
use crate::IterArgs;
use crate::IterId;
use crate::Label;
use crate::Local;
use crate::LocalRange;
use crate::MOpMode;
use crate::MemberKey;
use crate::MethodName;
use crate::NumParams;
use crate::OODeclExistsOp;
use crate::ObjMethodOp;
use crate::PropName;
use crate::QueryMOp;
use crate::ReadonlyOp;
use crate::RepoAuthType;
use crate::SetOpOp;
use crate::SetRangeOp;
use crate::SilenceOp;
use crate::SpecialClsRef;
use crate::StackIndex;
use crate::SwitchKind;
use crate::Targets;
use crate::TypeStructResolveOp;
use emit_opcodes_macro::Targets;
use ffi::Slice;
use ffi::Str;

#[emit_opcodes_macro::emit_opcodes]
#[derive(Clone, Debug, Targets, Hash, Eq, PartialEq)]
#[repr(C)]
pub enum Opcode<'arena> {
    // This is filled in by the emit_opcodes macro.  It can be printed using the
    // "//hphp/hack/src/hackc/hhbc:dump-opcodes" binary.
}

// The macro also generates:
// impl Opcode<'arena> {
//   pub fn variant_name(&self) -> &'static str;
//   pub fn num_inputs(&self) -> usize;
// }
