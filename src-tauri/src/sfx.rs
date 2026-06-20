use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::io::{BufReader, Cursor};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// Global storage for the audio output stream. Must live for the process
/// lifetime — if dropped, the audio device closes and playback stops.
/// OutputStream is not Send/Sync (raw pointers), but we only ever access it
/// from the main thread during init, then never touch it again.
struct StreamHolder(Option<OutputStream>);
unsafe impl Send for StreamHolder {}
unsafe impl Sync for StreamHolder {}

static STREAM_KEEPALIVE: Mutex<StreamHolder> = Mutex::new(StreamHolder(None));

/// Thread-safe audio player.
pub struct SfxPlayer {
    handle: OutputStreamHandle,
    enabled: AtomicBool,
}

pub type SharedSfx = Arc<SfxPlayer>;

impl SfxPlayer {
    pub fn new() -> Option<Self> {
        match OutputStream::try_default() {
            Ok((stream, handle)) => {
                // Store the stream in a global static so it stays alive.
                if let Ok(mut guard) = STREAM_KEEPALIVE.lock() {
                    guard.0 = Some(stream);
                }
                println!("Fox:audio initialized successfully");
                Some(SfxPlayer {
                    handle,
                    enabled: AtomicBool::new(true),
                })
            }
            Err(e) => {
                eprintln!("Fox:audio init failed: {e}");
                None
            }
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    fn play_bytes(&self, data: &[u8]) {
        if !self.is_enabled() {
            println!("Fox:sfx skipped (disabled)");
            return;
        }
        println!("Fox:sfx playing {} bytes", data.len());
        let cursor = Cursor::new(data.to_vec());
        let buf_reader = BufReader::new(cursor);
        match Decoder::new(buf_reader) {
            Ok(source) => {
                match Sink::try_new(&self.handle) {
                    Ok(sink) => {
                        sink.append(source.amplify(3.0));
                        sink.detach(); // Play in background, don't block
                        println!("Fox:sfx appended to sink");
                    }
                    Err(e) => {
                        eprintln!("Fox:sink creation error: {e}");
                        // Fallback: try play_raw
                        let cursor2 = Cursor::new(data.to_vec());
                        let buf2 = BufReader::new(cursor2);
                        if let Ok(src2) = Decoder::new(buf2) {
                            let _ = self.handle.play_raw(src2.convert_samples());
                            println!("Fox:sfx fallback play_raw");
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Fox:sfx decode error: {e}");
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Embedded sound effect files (OGG Vorbis)
// ---------------------------------------------------------------------------

pub struct SfxBank {
    pub permission_request: &'static [u8],
    pub task_complete: &'static [u8],
    pub error: &'static [u8],
    pub session_start: &'static [u8],
    pub subagent_complete: &'static [u8],
}

impl SfxBank {
    pub fn load() -> Self {
        SfxBank {
            permission_request: include_bytes!("../sound/permission_request.ogg"),
            task_complete: include_bytes!("../sound/task_complete.ogg"),
            error: include_bytes!("../sound/error.ogg"),
            session_start: include_bytes!("../sound/session_start.ogg"),
            subagent_complete: include_bytes!("../sound/subagent_complete.ogg"),
        }
    }
}

pub type SharedSfxBank = Arc<SfxBank>;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Initialize the audio subsystem. Returns (player, bank).
pub fn init() -> (SharedSfx, SharedSfxBank) {
    let player = SfxPlayer::new();
    let bank = SfxBank::load();
    let sfx: SharedSfx = match player {
        Some(p) => Arc::new(p),
        None => {
            // Create a dummy player with disabled state
            Arc::new(SfxPlayer {
                handle: OutputStream::try_default()
                    .map(|(_, h)| h)
                    .expect("audio fallback failed"),
                enabled: AtomicBool::new(false),
            })
        }
    };
    (sfx, Arc::new(bank))
}

/// Play a sound effect by name. Thread-safe, no-op if audio is disabled.
pub fn play(sfx: &SharedSfx, bank: &SharedSfxBank, name: &str) {
    let data = match name {
        "permission_request" => bank.permission_request,
        "task_complete" => bank.task_complete,
        "error" => bank.error,
        "session_start" => bank.session_start,
        "subagent_complete" => bank.subagent_complete,
        _ => return,
    };
    sfx.play_bytes(data);
}

/// Toggle sound on/off. Returns new state.
pub fn toggle(sfx: &SharedSfx) -> bool {
    let new = !sfx.is_enabled();
    sfx.set_enabled(new);
    new
}

/// Set sound enabled state directly.
pub fn set_enabled(sfx: &SharedSfx, enabled: bool) {
    sfx.set_enabled(enabled);
}

/// Check if sound is currently enabled.
pub fn is_enabled(sfx: &SharedSfx) -> bool {
    sfx.is_enabled()
}
