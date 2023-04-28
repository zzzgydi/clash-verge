use std::{
    fs::remove_file,
    io::{ErrorKind, Read, Result, Write},
    os::unix::net::{UnixListener, UnixStream},
    sync::Mutex,
};

use objc2::{
    class, declare_class, msg_send, msg_send_id,
    rc::{Id, Owned, Shared},
    runtime::{NSObject, Object},
    sel, ClassType,
};
use once_cell::sync::OnceCell;

use crate::ID;

type THandler = OnceCell<Mutex<Box<dyn FnMut(String) + Send + 'static>>>;

// If the Mutex turns out to be a problem, or FnMut turns out to be useless, we can remove the Mutex and turn FnMut into Fn
static HANDLER: THandler = OnceCell::new();

pub fn register<F, Fut>(_scheme: &str, handler: F) -> Result<()>
where
F: FnMut(String) -> Fut + Send + 'static,
Fut: Future<Output = ()> + Send + 'static,
{
    listen(handler)?;

    Ok(())
}

pub fn unregister(_scheme: &str) -> Result<()> {
    Ok(())
}

// kInternetEventClass
const EVENT_CLASS: u32 = 0x4755524c;
// kAEGetURL
const EVENT_GET_URL: u32 = 0x4755524c;

// Adapted from https://github.com/mrmekon/fruitbasket/blob/aad14e400d710d1d46317c0d8c55ff742bfeaadd/src/osx.rs#L848
fn parse_url_event(event: *mut Object) -> Option<String> {
    if event as u64 == 0u64 {
        return None;
    }
    unsafe {
        let class: u32 = msg_send![event, eventClass];
        let id: u32 = msg_send![event, eventID];
        if class != EVENT_CLASS || id != EVENT_GET_URL {
            return None;
        }

        let subevent: *mut Object = msg_send![event, paramDescriptorForKeyword: 0x2d2d2d2d_u32];
        let nsstring: *mut Object = msg_send![subevent, stringValue];
        let cstr: *const i8 = msg_send![nsstring, UTF8String];
        if !cstr.is_null() {
            Some(std::ffi::CStr::from_ptr(cstr).to_string_lossy().to_string())
        } else {
            None
        }
    }
}

declare_class!(
    struct Handler;

    unsafe impl ClassType for Handler {
        type Super = NSObject;
        const NAME: &'static str = "TauriPluginDeepLinkHandler";
    }

    unsafe impl Handler {
        #[method(handleEvent:withReplyEvent:)]
        fn handle_event(&self, event: *mut Object, _replace: *const Object) {
            let s = parse_url_event(event).unwrap_or_default();
            let mut cb = HANDLER.get().unwrap().lock().unwrap();
            cb(s);
        }
    }
);

impl Handler {
    pub fn new() -> Id<Self, Owned> {
        let cls = Self::class();
        unsafe { msg_send_id![msg_send_id![cls, alloc], init] }
    }
}

#[cfg(debug_assertions)]
fn secondary_handler(s: String) {
    let addr = format!(
        "/tmp/{}-deep-link.sock",
        ID.get()
            .expect("URL event received before prepare() was called")
    );
    if let Ok(mut stream) = UnixStream::connect(addr) {
        if let Err(io_err) = stream.write_all(s.as_bytes()) {
            log::error!(
                "Error sending message to primary instance: {}",
                io_err.to_string()
            );
        };
    }
    std::process::exit(0);
}

pub fn listen<F,Fut>(handler: F) -> Result<()>
where
F: FnMut(String) -> Fut + Send + 'static ,
Fut: Future<Output = ()> + Send  + 'static,{
    #[cfg(debug_assertions)]
    let addr = format!(
        "/tmp/{}-deep-link.sock",
        ID.get().expect("listen() called before prepare()")
    );

    #[cfg(debug_assertions)]
    if HANDLER
        .set(match UnixStream::connect(&addr) {
            Ok(_) => Mutex::new(Box::new(secondary_handler)),
            Err(err) => {
                log::error!("Error creating socket listener: {}", err.to_string());
                if err.kind() == ErrorKind::ConnectionRefused {
                    let _ = remove_file(&addr);
                }
                Mutex::new(Box::new(handler))
            }
        })
        .is_err()
    {
        return Err(std::io::Error::new(
            ErrorKind::AlreadyExists,
            "Handler was already set",
        ));
    }

    #[cfg(not(debug_assertions))]
    if HANDLER.set(Mutex::new(Box::new(handler))).is_err() {
        return Err(std::io::Error::new(
            ErrorKind::AlreadyExists,
            "Handler was already set",
        ));
    }

    unsafe {
        let event_manager: Id<Object, Shared> =
            msg_send_id![class!(NSAppleEventManager), sharedAppleEventManager];

        let handler = Handler::new();
        let handler_boxed = Box::into_raw(Box::new(handler));

        let _: () = msg_send![&event_manager,
            setEventHandler: &**handler_boxed
            andSelector: sel!(handleEvent:withReplyEvent:)
            forEventClass:EVENT_CLASS
            andEventID:EVENT_GET_URL];
    }

    #[cfg(debug_assertions)]
    std::thread::spawn(move || {
        let listener = UnixListener::bind(addr).expect("Can't create listener");

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buffer = String::new();
                    if let Err(io_err) = stream.read_to_string(&mut buffer) {
                        log::error!("Error reading incoming connection: {}", io_err.to_string());
                    };

                    let mut cb = HANDLER.get().unwrap().lock().unwrap();
                    cb(buffer);
                }
                Err(err) => {
                    log::error!("Incoming connection failed: {}", err);
                    continue;
                }
            }
        }
    });

    Ok(())
}

pub fn prepare(identifier: &str) {
    ID.set(identifier.to_string())
        .expect("prepare() called more than once with different identifiers.");
}
