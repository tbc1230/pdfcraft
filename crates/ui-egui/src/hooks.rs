/// Files delivered asynchronously: (name, bytes).
pub type Inbox = std::sync::Arc<std::sync::Mutex<Vec<(String, Vec<u8>)>>>;

/// An author name the embedding page set (web), waiting to be taken by the app on its next frame.
pub type AuthorInbox = std::sync::Arc<std::sync::Mutex<Option<String>>>;

/// An interface language the embedding page set (web), a canonical `language` preference value waiting to be taken by the app on its next frame.
pub type LanguageInbox = std::sync::Arc<std::sync::Mutex<Option<String>>>;

/// A theme choice the embedding page set (web), waiting to be taken by the app on its next frame.
pub type ThemeInbox = std::sync::Arc<std::sync::Mutex<Option<crate::theme::ThemePreference>>>;

/// Files that failed to arrive asynchronously (a browser `?file=` URL that couldn't be fetched):
/// `(name, error)`, reported to the user on the next frame (#173).
pub type FailedInbox = std::sync::Arc<std::sync::Mutex<Vec<(String, String)>>>;

/// A request the operating system sends the running app, outside its window: on macOS, Finder
/// double-clicks, Open With and drops on the Dock icon arrive as Apple events, not arguments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OsEvent {
    /// Open these files.
    Open(Vec<String>),
    /// Quit (the Dock's Quit, logging out), asking about unsaved changes first.
    Quit,
}

/// Returns the [`OsEvent`]s that arrived since it was last called (set by the desktop app).
pub type OsEventsFn = Box<dyn FnMut() -> Vec<OsEvent>>;

/// Hands a saved PDF to the page embedding PdfCraft (web): `(name, bytes, save_as)`. `save_as`
/// is `true` for Save As. Set by the web app when a host page (such as a Nextcloud app) asked
/// for saves; without it the browser downloads the file.
pub type HostSaveFn = Box<dyn Fn(&str, &[u8], bool) -> Result<(), String>>;

/// Told every frame whether any tab has unsaved work, so a host page can warn before leaving.
pub type HostDirtyFn = Box<dyn FnMut(bool)>;
