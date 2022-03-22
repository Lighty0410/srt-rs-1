use std::ffi::CStr;
use std::pin::Pin;

use libc::{c_char as char, c_int as int, c_void as void};
use srt::sockaddr;

use crate::error::handle_result;
use crate::{srt, SrtAsyncListener};

use super::Result;

pub trait ListenerCallback {
    fn callback(&self, stream_id: Option<&str>) -> ListenerCallbackAction;
}

impl<T: Fn(Option<&str>) -> ListenerCallbackAction> ListenerCallback for T {
    fn callback(&self, stream_id: Option<&str>) -> ListenerCallbackAction {
        (*self)(stream_id)
    }
}

pub enum ListenerCallbackAction {
    Deny,
    Allow { passphrase: Option<String> },
}

extern "C" fn listener_callback(
    opaq: *mut void,
    ns: srt::SRTSOCKET,
    _hs_version: int,
    _peer: *const sockaddr,
    stream_id: *const char,
) -> int {
    unsafe {
        let f = opaq as *mut Box<dyn ListenerCallback>;
        let stream_id = if stream_id.is_null() {
            None
        } else {
            CStr::from_ptr(stream_id).to_str().ok()
        };
        match (*f).callback(stream_id) {
            ListenerCallbackAction::Deny => -1,
            ListenerCallbackAction::Allow { passphrase } => {
                if let Some(v) = &passphrase {
                    return srt::srt_setsockopt(
                        ns,
                        0,
                        srt::SRT_SOCKOPT::SRTO_PASSPHRASE,
                        v.as_ptr() as *const _,
                        v.len() as _,
                    );
                }
                0
            }
        }
    }
}

impl<'c> SrtAsyncListener<'c> {
    pub fn with_callback<F: ListenerCallback + 'c>(self, f: F) -> Result<SrtAsyncListener<'c>> {
        let mut cb: Box<Box<dyn ListenerCallback>> = Box::new(Box::new(f));
        let ptr = &mut *cb as *mut Box<dyn ListenerCallback>;
        let pb = unsafe { Pin::new_unchecked(cb) };
        let res = unsafe {
            srt::srt_listen_callback(self.socket.id, Some(listener_callback), ptr as *mut _)
        };
        if res != 0 {
            return handle_result(self, res);
        }

        Ok(SrtAsyncListener {
            socket: self.socket,
            _callback: Some(pb),
        })
    }
}
