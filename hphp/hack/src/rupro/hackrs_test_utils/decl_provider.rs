// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

use crate::registrar::DependencyGraph;
use crate::serde_store::StoreOpts;
use crate::SerializingStore;
use datastore::NonEvictingStore;
use hackrs::decl_parser::DeclParser;
use hackrs::folded_decl_provider::FoldedDeclProvider;
use hackrs::folded_decl_provider::LazyFoldedDeclProvider;
use hackrs::shallow_decl_provider::EagerShallowDeclProvider;
use hackrs::shallow_decl_provider::LazyShallowDeclProvider;
use hackrs::shallow_decl_provider::ShallowDeclProvider;
use hackrs::shallow_decl_provider::ShallowDeclStore;
use naming_provider::SqliteNamingTable;
use std::path::PathBuf;
use std::sync::Arc;
use ty::reason::Reason;

pub fn make_folded_decl_provider<R: Reason>(
    store_opts: StoreOpts,
    naming_table: Option<&PathBuf>,
    shallow_decl_store: ShallowDeclStore<R>,
    decl_parser: DeclParser<R>,
) -> impl FoldedDeclProvider<R> {
    let shallow_decl_provider: Arc<dyn ShallowDeclProvider<R>> =
        if let Some(naming_table_path) = naming_table {
            Arc::new(LazyShallowDeclProvider::new(
                Arc::new(shallow_decl_store),
                Arc::new(SqliteNamingTable::new(naming_table_path).unwrap()),
                decl_parser,
            ))
        } else {
            Arc::new(EagerShallowDeclProvider::new(Arc::new(shallow_decl_store)))
        };

    LazyFoldedDeclProvider::new(
        Arc::new(oxidized::global_options::GlobalOptions::default()),
        match store_opts {
            StoreOpts::Serialized(compression_type) => {
                Arc::new(SerializingStore::with_compression(compression_type))
            }
            StoreOpts::Unserialized => Arc::new(NonEvictingStore::new()),
        },
        shallow_decl_provider,
        Arc::new(DependencyGraph::new()),
    )
}
