mod utils;
mod proto {
    // SAFETY: allow: prost
    #![allow(
        box_pointers,
        unreachable_pub,
        unused_qualifications,
        unused_results,
        clippy::default_trait_access,
        clippy::derive_partial_eq_without_eq,
        clippy::doc_markdown,
        clippy::future_not_send,
        clippy::large_enum_variant,
        clippy::missing_const_for_fn,
        clippy::missing_errors_doc,
        clippy::must_use_candidate,
        clippy::return_self_not_must_use,
        clippy::similar_names,
        clippy::too_many_lines,
        clippy::use_self,
        clippy::wildcard_imports
    )]

    tonic::include_proto!("clipcat");
}

use std::str::FromStr;

use time::OffsetDateTime;

pub use self::proto::{
    manager_client::ManagerClient,
    manager_server::{Manager, ManagerServer},
    watcher_client::WatcherClient,
    watcher_server::{Watcher, WatcherServer},
    BatchRemoveRequest, BatchRemoveResponse, ClipEntry, ClipEntryMetadata, ClipboardKind,
    GetCurrentClipRequest, GetCurrentClipResponse, GetRequest, GetResponse, InsertRequest,
    InsertResponse, LengthResponse, ListRequest, ListResponse, MarkRequest, MarkResponse,
    RemoveRequest, RemoveResponse, UpdateRequest, UpdateResponse, WatcherState, WatcherStateReply,
};

impl From<ClipboardKind> for clipcat::ClipboardKind {
    fn from(t: ClipboardKind) -> Self {
        match t {
            ClipboardKind::Clipboard => Self::Clipboard,
            ClipboardKind::Primary => Self::Primary,
            ClipboardKind::Secondary => Self::Secondary,
        }
    }
}

impl From<clipcat::ClipboardKind> for ClipboardKind {
    fn from(t: clipcat::ClipboardKind) -> Self {
        match t {
            clipcat::ClipboardKind::Clipboard => Self::Clipboard,
            clipcat::ClipboardKind::Primary => Self::Primary,
            clipcat::ClipboardKind::Secondary => Self::Secondary,
        }
    }
}

impl From<clipcat::ClipEntry> for ClipEntry {
    fn from(entry: clipcat::ClipEntry) -> Self {
        let mime = entry.mime().essence_str().to_owned();
        let data = entry.encoded().unwrap_or_default();
        let id = entry.id();
        let kind = entry.kind();
        let timestamp = utils::datetime_to_timestamp(&entry.timestamp());

        Self { id, data, kind: kind.into(), mime, timestamp: Some(timestamp) }
    }
}

impl From<ClipEntry> for clipcat::ClipEntry {
    fn from(ClipEntry { id: _, data, mime, kind, timestamp }: ClipEntry) -> Self {
        let timestamp = timestamp.and_then(|ts| utils::timestamp_to_datetime(&ts).ok());
        let kind = clipcat::ClipboardKind::from(kind);
        let mime = mime::Mime::from_str(&mime).unwrap_or(mime::APPLICATION_OCTET_STREAM);
        Self::new(&data, &mime, kind, timestamp).unwrap_or_default()
    }
}

impl From<clipcat::ClipEntryMetadata> for ClipEntryMetadata {
    fn from(metadata: clipcat::ClipEntryMetadata) -> Self {
        let clipcat::ClipEntryMetadata { id, kind: clipboard_kind, timestamp, mime, preview } =
            metadata;
        let mime = mime.essence_str().to_owned();
        let timestamp = utils::datetime_to_timestamp(&timestamp);
        Self { id, preview, kind: clipboard_kind.into(), mime, timestamp: Some(timestamp) }
    }
}

impl From<ClipEntryMetadata> for clipcat::ClipEntryMetadata {
    fn from(ClipEntryMetadata { id, mime, kind, timestamp, preview }: ClipEntryMetadata) -> Self {
        let timestamp = timestamp
            .and_then(|ts| utils::timestamp_to_datetime(&ts).ok())
            .unwrap_or_else(OffsetDateTime::now_utc);
        let clipboard_kind = clipcat::ClipboardKind::from(kind);
        let mime = mime::Mime::from_str(&mime).unwrap_or(mime::APPLICATION_OCTET_STREAM);
        Self { id, kind: clipboard_kind, timestamp, mime, preview }
    }
}

impl From<WatcherState> for clipcat::ClipboardWatcherState {
    fn from(state: WatcherState) -> Self {
        match state {
            WatcherState::Enabled => Self::Enabled,
            WatcherState::Disabled => Self::Disabled,
        }
    }
}

impl From<clipcat::ClipboardWatcherState> for WatcherState {
    fn from(val: clipcat::ClipboardWatcherState) -> Self {
        match val {
            clipcat::ClipboardWatcherState::Enabled => Self::Enabled,
            clipcat::ClipboardWatcherState::Disabled => Self::Disabled,
        }
    }
}
