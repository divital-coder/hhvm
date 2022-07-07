(*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the "hack" directory of this source tree.
 *
 *)

exception Refactor_sd_exn of string

type mode =
  | FlagTargets
  | DumpConstraints
  | SimplifyConstraints
  | SolveConstraints [@deriving eq]

type options = { mode: mode }

type refactor_sd_result =
  | Exists_Upcast
  | No_Upcast
