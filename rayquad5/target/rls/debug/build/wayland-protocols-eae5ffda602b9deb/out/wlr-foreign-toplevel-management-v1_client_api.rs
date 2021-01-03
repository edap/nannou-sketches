use std::os::raw::{c_char, c_void};
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const sys::common::wl_interface; 1] =
    [NULLPTR as *const sys::common::wl_interface];
#[doc = "list and control opened apps\n\nThe purpose of this protocol is to enable the creation of taskbars\nand docks by providing them with a list of opened applications and\nletting them request certain actions on them, like maximizing, etc.\n\nAfter a client binds the zwlr_foreign_toplevel_manager_v1, each opened\ntoplevel window will be sent via the toplevel event"]
pub mod zwlr_foreign_toplevel_manager_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        types_null, AnonymousObject, Argument, ArgumentType, HandledBy, Interface, Message,
        MessageDesc, MessageGroup, NewProxy, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    pub enum Request {
        #[doc = "stop sending events\n\nIndicates the client no longer wishes to receive events for new toplevels.\nHowever the compositor may emit further toplevel_created events, until\nthe finished event is emitted.\n\nThe client must not send any more requests after this one."]
        Stop,
        #[doc(hidden)]
        __nonexhaustive,
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "stop",
            since: 1,
            signature: &[],
        }];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::__nonexhaustive => unreachable!(),
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::__nonexhaustive => unreachable!(),
                Request::Stop => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::__nonexhaustive => unreachable!(),
                Request::Stop => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::__nonexhaustive => unreachable!(),
                Request::Stop => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: vec![],
                },
            }
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Request, ()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            match self {
                Request::__nonexhaustive => unreachable!(),
                Request::Stop => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                }
            }
        }
    }
    pub enum Event {
        #[doc = "a toplevel has been created\n\nThis event is emitted whenever a new toplevel window is created. It\nis emitted for all toplevels, regardless of the app that has created\nthem.\n\nAll initial details of the toplevel(title, app_id, states, etc.) will\nbe sent immediately after this event via the corresponding events in\nzwlr_foreign_toplevel_handle_v1."]
        Toplevel {
            toplevel: NewProxy<super::zwlr_foreign_toplevel_handle_v1::ZwlrForeignToplevelHandleV1>,
        },
        #[doc = "the compositor has finished with the toplevel manager\n\nThis event indicates that the compositor is done sending events to the\nzwlr_foreign_toplevel_manager_v1. The server will destroy the object\nimmediately after sending this request, so it will become invalid and\nthe client should free any resources associated with it."]
        Finished,
        #[doc(hidden)]
        __nonexhaustive,
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "toplevel",
                since: 1,
                signature: &[super::ArgumentType::NewId],
            },
            super::MessageDesc {
                name: "finished",
                since: 1,
                signature: &[],
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Event::__nonexhaustive => unreachable!(),
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Event::__nonexhaustive => unreachable!(),
                Event::Toplevel { .. } => 0,
                Event::Finished => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::__nonexhaustive => unreachable!(),
                Event::Toplevel { .. } => 1,
                Event::Finished => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(Object::from_interface::<
                    super::zwlr_foreign_toplevel_handle_v1::ZwlrForeignToplevelHandleV1,
                >(version, meta.child())),
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                0 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Toplevel {
                        toplevel: {
                            if let Some(Argument::NewId(val)) = args.next() {
                                map.get_new(val).ok_or(())?
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                1 => Ok(Event::Finished),
                _ => Err(()),
            }
        }
        fn into_raw(self, sender_id: u32) -> Message {
            panic!("Event::into_raw can not be used Client-side.")
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Event, ()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Toplevel {
                        toplevel: NewProxy::<
                            super::zwlr_foreign_toplevel_handle_v1::ZwlrForeignToplevelHandleV1,
                        >::from_c_ptr(_args[0].o as *mut _),
                    })
                }
                1 => Ok(Event::Finished),
                _ => return Err(()),
            }
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }
    #[derive(Clone, Eq, PartialEq)]
    pub struct ZwlrForeignToplevelManagerV1(Proxy<ZwlrForeignToplevelManagerV1>);
    impl AsRef<Proxy<ZwlrForeignToplevelManagerV1>> for ZwlrForeignToplevelManagerV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZwlrForeignToplevelManagerV1>> for ZwlrForeignToplevelManagerV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZwlrForeignToplevelManagerV1(value)
        }
    }
    impl From<ZwlrForeignToplevelManagerV1> for Proxy<ZwlrForeignToplevelManagerV1> {
        #[inline]
        fn from(value: ZwlrForeignToplevelManagerV1) -> Self {
            value.0
        }
    }
    impl Interface for ZwlrForeignToplevelManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwlr_foreign_toplevel_manager_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwlr_foreign_toplevel_manager_v1_interface }
        }
    }
    impl ZwlrForeignToplevelManagerV1 {
        #[doc = "stop sending events\n\nIndicates the client no longer wishes to receive events for new toplevels.\nHowever the compositor may emit further toplevel_created events, until\nthe finished event is emitted.\n\nThe client must not send any more requests after this one."]
        pub fn stop(&self) -> () {
            let msg = Request::Stop;
            self.0.send(msg);
        }
    }
    #[doc = r" An interface for handling events."]
    pub trait EventHandler {
        #[doc = "a toplevel has been created\n\nThis event is emitted whenever a new toplevel window is created. It\nis emitted for all toplevels, regardless of the app that has created\nthem.\n\nAll initial details of the toplevel(title, app_id, states, etc.) will\nbe sent immediately after this event via the corresponding events in\nzwlr_foreign_toplevel_handle_v1."]
        fn toplevel(
            &mut self,
            object: ZwlrForeignToplevelManagerV1,
            toplevel: NewProxy<super::zwlr_foreign_toplevel_handle_v1::ZwlrForeignToplevelHandleV1>,
        ) {
        }
        #[doc = "the compositor has finished with the toplevel manager\n\nThis event indicates that the compositor is done sending events to the\nzwlr_foreign_toplevel_manager_v1. The server will destroy the object\nimmediately after sending this request, so it will become invalid and\nthe client should free any resources associated with it."]
        fn finished(&mut self, object: ZwlrForeignToplevelManagerV1) {}
    }
    impl<T: EventHandler> HandledBy<T> for ZwlrForeignToplevelManagerV1 {
        #[inline]
        fn handle(__handler: &mut T, event: Event, __object: Self) {
            match event {
                Event::Toplevel { toplevel } => __handler.toplevel(__object, toplevel),
                Event::Finished {} => __handler.finished(__object),
                Event::__nonexhaustive => unreachable!(),
            }
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_STOP_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_TOPLEVEL_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_FINISHED_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwlr_foreign_toplevel_manager_v1_requests: [wl_message; 1] = [wl_message {
        name: b"stop\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    static mut zwlr_foreign_toplevel_manager_v1_events_toplevel_types: [*const wl_interface; 1] =
        [unsafe {
            &super::zwlr_foreign_toplevel_handle_v1::zwlr_foreign_toplevel_handle_v1_interface
                as *const wl_interface
        }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwlr_foreign_toplevel_manager_v1_events: [wl_message; 2] = [
        wl_message {
            name: b"toplevel\0" as *const u8 as *const c_char,
            signature: b"n\0" as *const u8 as *const c_char,
            types: unsafe { &zwlr_foreign_toplevel_manager_v1_events_toplevel_types as *const _ },
        },
        wl_message {
            name: b"finished\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwlr_foreign_toplevel_manager_v1_interface: wl_interface = wl_interface {
        name: b"zwlr_foreign_toplevel_manager_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 1,
        requests: unsafe { &zwlr_foreign_toplevel_manager_v1_requests as *const _ },
        event_count: 2,
        events: unsafe { &zwlr_foreign_toplevel_manager_v1_events as *const _ },
    };
}
#[doc = "an opened toplevel\n\nA zwlr_foreign_toplevel_handle_v1 object represents an opened toplevel\nwindow. Each app may have multiple opened toplevels.\n\nEach toplevel has a list of outputs it is visible on, conveyed to the\nclient with the output_enter and output_leave events."]
pub mod zwlr_foreign_toplevel_handle_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        types_null, AnonymousObject, Argument, ArgumentType, HandledBy, Interface, Message,
        MessageDesc, MessageGroup, NewProxy, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[doc = "types of states on the toplevel\n\nThe different states that a toplevel can have. These have the same meaning\nas the states with the same names defined in xdg-toplevel"]
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum State {
        #[doc = "the toplevel is maximized"]
        Maximized = 0,
        #[doc = "the toplevel is minimized"]
        Minimized = 1,
        #[doc = "the toplevel is active"]
        Activated = 2,
        #[doc(hidden)]
        __nonexhaustive,
    }
    impl State {
        pub fn from_raw(n: u32) -> Option<State> {
            match n {
                0 => Some(State::Maximized),
                1 => Some(State::Minimized),
                2 => Some(State::Activated),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum Error {
        #[doc = "the provided rectangle is invalid"]
        InvalidRectangle = 0,
        #[doc(hidden)]
        __nonexhaustive,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::InvalidRectangle),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    pub enum Request {
        #[doc = "requests that the toplevel be maximized\n\nRequests that the toplevel be maximized. If the maximized state actually\nchanges, this will be indicated by the state event."]
        SetMaximized,
        #[doc = "requests that the toplevel be unmaximized\n\nRequests that the toplevel be unmaximized. If the maximized state actually\nchanges, this will be indicated by the state event."]
        UnsetMaximized,
        #[doc = "requests that the toplevel be minimized\n\nRequests that the toplevel be minimized. If the minimized state actually\nchanges, this will be indicated by the state event."]
        SetMinimized,
        #[doc = "requests that the toplevel be unminimized\n\nRequests that the toplevel be unminimized. If the minimized state actually\nchanges, this will be indicated by the state event."]
        UnsetMinimized,
        #[doc = "activate the toplevel\n\nRequest that this toplevel be activated on the given seat.\nThere is no guarantee the toplevel will be actually activated."]
        Activate { seat: super::wl_seat::WlSeat },
        #[doc = "request that the toplevel be closed\n\nSend a request to the toplevel to close itself. The compositor would\ntypically use a shell-specific method to carry out this request, for\nexample by sending the xdg_toplevel.close event. However, this gives\nno guarantees the toplevel will actually be destroyed. If and when\nthis happens, the zwlr_foreign_toplevel_handle_v1.closed event will\nbe emitted."]
        Close,
        #[doc = "the rectangle which represents the toplevel\n\nThe rectangle of the surface specified in this request corresponds to\nthe place where the app using this protocol represents the given toplevel.\nIt can be used by the compositor as a hint for some operations, e.g\nminimizing. The client is however not required to set this, in which\ncase the compositor is free to decide some default value.\n\nIf the client specifies more than one rectangle, only the last one is\nconsidered.\n\nThe dimensions are given in surface-local coordinates.\nSetting width=height=0 removes the already-set rectangle."]
        SetRectangle {
            surface: super::wl_surface::WlSurface,
            x: i32,
            y: i32,
            width: i32,
            height: i32,
        },
        #[doc = "destroy the zwlr_foreign_toplevel_handle_v1 object\n\nDestroys the zwlr_foreign_toplevel_handle_v1 object.\n\nThis request should be called either when the client does not want to\nuse the toplevel anymore or after the closed event to finalize the\ndestruction of the object.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc(hidden)]
        __nonexhaustive,
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "set_maximized",
                since: 1,
                signature: &[],
            },
            super::MessageDesc {
                name: "unset_maximized",
                since: 1,
                signature: &[],
            },
            super::MessageDesc {
                name: "set_minimized",
                since: 1,
                signature: &[],
            },
            super::MessageDesc {
                name: "unset_minimized",
                since: 1,
                signature: &[],
            },
            super::MessageDesc {
                name: "activate",
                since: 1,
                signature: &[super::ArgumentType::Object],
            },
            super::MessageDesc {
                name: "close",
                since: 1,
                signature: &[],
            },
            super::MessageDesc {
                name: "set_rectangle",
                since: 1,
                signature: &[
                    super::ArgumentType::Object,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                ],
            },
            super::MessageDesc {
                name: "destroy",
                since: 1,
                signature: &[],
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::__nonexhaustive => unreachable!(),
                Request::Destroy => true,
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::__nonexhaustive => unreachable!(),
                Request::SetMaximized => 0,
                Request::UnsetMaximized => 1,
                Request::SetMinimized => 2,
                Request::UnsetMinimized => 3,
                Request::Activate { .. } => 4,
                Request::Close => 5,
                Request::SetRectangle { .. } => 6,
                Request::Destroy => 7,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::__nonexhaustive => unreachable!(),
                Request::SetMaximized => 1,
                Request::UnsetMaximized => 1,
                Request::SetMinimized => 1,
                Request::UnsetMinimized => 1,
                Request::Activate { .. } => 1,
                Request::Close => 1,
                Request::SetRectangle { .. } => 1,
                Request::Destroy => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::__nonexhaustive => unreachable!(),
                Request::SetMaximized => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: vec![],
                },
                Request::UnsetMaximized => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: vec![],
                },
                Request::SetMinimized => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: vec![],
                },
                Request::UnsetMinimized => Message {
                    sender_id: sender_id,
                    opcode: 3,
                    args: vec![],
                },
                Request::Activate { seat } => Message {
                    sender_id: sender_id,
                    opcode: 4,
                    args: vec![Argument::Object(seat.as_ref().id())],
                },
                Request::Close => Message {
                    sender_id: sender_id,
                    opcode: 5,
                    args: vec![],
                },
                Request::SetRectangle {
                    surface,
                    x,
                    y,
                    width,
                    height,
                } => Message {
                    sender_id: sender_id,
                    opcode: 6,
                    args: vec![
                        Argument::Object(surface.as_ref().id()),
                        Argument::Int(x),
                        Argument::Int(y),
                        Argument::Int(width),
                        Argument::Int(height),
                    ],
                },
                Request::Destroy => Message {
                    sender_id: sender_id,
                    opcode: 7,
                    args: vec![],
                },
            }
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Request, ()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            match self {
                Request::__nonexhaustive => unreachable!(),
                Request::SetMaximized => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                }
                Request::UnsetMaximized => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(1, &mut _args_array)
                }
                Request::SetMinimized => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(2, &mut _args_array)
                }
                Request::UnsetMinimized => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(3, &mut _args_array)
                }
                Request::Activate { seat } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = seat.as_ref().c_ptr() as *mut _;
                    f(4, &mut _args_array)
                }
                Request::Close => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(5, &mut _args_array)
                }
                Request::SetRectangle {
                    surface,
                    x,
                    y,
                    width,
                    height,
                } => {
                    let mut _args_array: [wl_argument; 5] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = surface.as_ref().c_ptr() as *mut _;
                    _args_array[1].i = x;
                    _args_array[2].i = y;
                    _args_array[3].i = width;
                    _args_array[4].i = height;
                    f(6, &mut _args_array)
                }
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(7, &mut _args_array)
                }
            }
        }
    }
    pub enum Event {
        #[doc = "title change\n\nThis event is emitted whenever the title of the toplevel changes."]
        Title { title: String },
        #[doc = "app-id change\n\nThis event is emitted whenever the app-id of the toplevel changes."]
        AppId { app_id: String },
        #[doc = "toplevel entered an output\n\nThis event is emitted whenever the toplevel becomes visible on\nthe given output. A toplevel may be visible on multiple outputs."]
        OutputEnter { output: super::wl_output::WlOutput },
        #[doc = "toplevel left an output\n\nThis event is emitted whenever the toplevel stops being visible on\nthe given output. It is guaranteed that an entered-output event\nwith the same output has been emitted before this event."]
        OutputLeave { output: super::wl_output::WlOutput },
        #[doc = "the toplevel state changed\n\nThis event is emitted immediately after the zlw_foreign_toplevel_handle_v1\nis created and each time the toplevel state changes, either because of a\ncompositor action or because of a request in this protocol."]
        State { state: Vec<u8> },
        #[doc = "all information about the toplevel has been sent\n\nThis event is sent after all changes in the toplevel state have been\nsent.\n\nThis allows changes to the zwlr_foreign_toplevel_handle_v1 properties\nto be seen as atomic, even if they happen via multiple events."]
        Done,
        #[doc = "this toplevel has been destroyed\n\nThis event means the toplevel has been destroyed. It is guaranteed there\nwon't be any more events for this zwlr_foreign_toplevel_handle_v1. The\ntoplevel itself becomes inert so any requests will be ignored except the\ndestroy request."]
        Closed,
        #[doc(hidden)]
        __nonexhaustive,
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "title",
                since: 1,
                signature: &[super::ArgumentType::Str],
            },
            super::MessageDesc {
                name: "app_id",
                since: 1,
                signature: &[super::ArgumentType::Str],
            },
            super::MessageDesc {
                name: "output_enter",
                since: 1,
                signature: &[super::ArgumentType::Object],
            },
            super::MessageDesc {
                name: "output_leave",
                since: 1,
                signature: &[super::ArgumentType::Object],
            },
            super::MessageDesc {
                name: "state",
                since: 1,
                signature: &[super::ArgumentType::Array],
            },
            super::MessageDesc {
                name: "done",
                since: 1,
                signature: &[],
            },
            super::MessageDesc {
                name: "closed",
                since: 1,
                signature: &[],
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Event::__nonexhaustive => unreachable!(),
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Event::__nonexhaustive => unreachable!(),
                Event::Title { .. } => 0,
                Event::AppId { .. } => 1,
                Event::OutputEnter { .. } => 2,
                Event::OutputLeave { .. } => 3,
                Event::State { .. } => 4,
                Event::Done => 5,
                Event::Closed => 6,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::__nonexhaustive => unreachable!(),
                Event::Title { .. } => 1,
                Event::AppId { .. } => 1,
                Event::OutputEnter { .. } => 1,
                Event::OutputLeave { .. } => 1,
                Event::State { .. } => 1,
                Event::Done => 1,
                Event::Closed => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                0 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::Title {
                        title: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| {
                                    String::from_utf8_lossy(&e.into_bytes()).into()
                                });
                                s
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::AppId {
                        app_id: {
                            if let Some(Argument::Str(val)) = args.next() {
                                let s = String::from_utf8(val.into_bytes()).unwrap_or_else(|e| {
                                    String::from_utf8_lossy(&e.into_bytes()).into()
                                });
                                s
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                2 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::OutputEnter {
                        output: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get(val).ok_or(())?.into()
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                3 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::OutputLeave {
                        output: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get(val).ok_or(())?.into()
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                4 => {
                    let mut args = msg.args.into_iter();
                    Ok(Event::State {
                        state: {
                            if let Some(Argument::Array(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                5 => Ok(Event::Done),
                6 => Ok(Event::Closed),
                _ => Err(()),
            }
        }
        fn into_raw(self, sender_id: u32) -> Message {
            panic!("Event::into_raw can not be used Client-side.")
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Event, ()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Title {
                        title: ::std::ffi::CStr::from_ptr(_args[0].s)
                            .to_string_lossy()
                            .into_owned(),
                    })
                }
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::AppId {
                        app_id: ::std::ffi::CStr::from_ptr(_args[0].s)
                            .to_string_lossy()
                            .into_owned(),
                    })
                }
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::OutputEnter {
                        output: Proxy::<super::wl_output::WlOutput>::from_c_ptr(
                            _args[0].o as *mut _,
                        )
                        .into(),
                    })
                }
                3 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::OutputLeave {
                        output: Proxy::<super::wl_output::WlOutput>::from_c_ptr(
                            _args[0].o as *mut _,
                        )
                        .into(),
                    })
                }
                4 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::State {
                        state: {
                            let array = &*_args[0].a;
                            ::std::slice::from_raw_parts(array.data as *const u8, array.size)
                                .to_owned()
                        },
                    })
                }
                5 => Ok(Event::Done),
                6 => Ok(Event::Closed),
                _ => return Err(()),
            }
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }
    #[derive(Clone, Eq, PartialEq)]
    pub struct ZwlrForeignToplevelHandleV1(Proxy<ZwlrForeignToplevelHandleV1>);
    impl AsRef<Proxy<ZwlrForeignToplevelHandleV1>> for ZwlrForeignToplevelHandleV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZwlrForeignToplevelHandleV1>> for ZwlrForeignToplevelHandleV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZwlrForeignToplevelHandleV1(value)
        }
    }
    impl From<ZwlrForeignToplevelHandleV1> for Proxy<ZwlrForeignToplevelHandleV1> {
        #[inline]
        fn from(value: ZwlrForeignToplevelHandleV1) -> Self {
            value.0
        }
    }
    impl Interface for ZwlrForeignToplevelHandleV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwlr_foreign_toplevel_handle_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwlr_foreign_toplevel_handle_v1_interface }
        }
    }
    impl ZwlrForeignToplevelHandleV1 {
        #[doc = "requests that the toplevel be maximized\n\nRequests that the toplevel be maximized. If the maximized state actually\nchanges, this will be indicated by the state event."]
        pub fn set_maximized(&self) -> () {
            let msg = Request::SetMaximized;
            self.0.send(msg);
        }
        #[doc = "requests that the toplevel be unmaximized\n\nRequests that the toplevel be unmaximized. If the maximized state actually\nchanges, this will be indicated by the state event."]
        pub fn unset_maximized(&self) -> () {
            let msg = Request::UnsetMaximized;
            self.0.send(msg);
        }
        #[doc = "requests that the toplevel be minimized\n\nRequests that the toplevel be minimized. If the minimized state actually\nchanges, this will be indicated by the state event."]
        pub fn set_minimized(&self) -> () {
            let msg = Request::SetMinimized;
            self.0.send(msg);
        }
        #[doc = "requests that the toplevel be unminimized\n\nRequests that the toplevel be unminimized. If the minimized state actually\nchanges, this will be indicated by the state event."]
        pub fn unset_minimized(&self) -> () {
            let msg = Request::UnsetMinimized;
            self.0.send(msg);
        }
        #[doc = "activate the toplevel\n\nRequest that this toplevel be activated on the given seat.\nThere is no guarantee the toplevel will be actually activated."]
        pub fn activate(&self, seat: &super::wl_seat::WlSeat) -> () {
            let msg = Request::Activate { seat: seat.clone() };
            self.0.send(msg);
        }
        #[doc = "request that the toplevel be closed\n\nSend a request to the toplevel to close itself. The compositor would\ntypically use a shell-specific method to carry out this request, for\nexample by sending the xdg_toplevel.close event. However, this gives\nno guarantees the toplevel will actually be destroyed. If and when\nthis happens, the zwlr_foreign_toplevel_handle_v1.closed event will\nbe emitted."]
        pub fn close(&self) -> () {
            let msg = Request::Close;
            self.0.send(msg);
        }
        #[doc = "the rectangle which represents the toplevel\n\nThe rectangle of the surface specified in this request corresponds to\nthe place where the app using this protocol represents the given toplevel.\nIt can be used by the compositor as a hint for some operations, e.g\nminimizing. The client is however not required to set this, in which\ncase the compositor is free to decide some default value.\n\nIf the client specifies more than one rectangle, only the last one is\nconsidered.\n\nThe dimensions are given in surface-local coordinates.\nSetting width=height=0 removes the already-set rectangle."]
        pub fn set_rectangle(
            &self,
            surface: &super::wl_surface::WlSurface,
            x: i32,
            y: i32,
            width: i32,
            height: i32,
        ) -> () {
            let msg = Request::SetRectangle {
                surface: surface.clone(),
                x: x,
                y: y,
                width: width,
                height: height,
            };
            self.0.send(msg);
        }
        #[doc = "destroy the zwlr_foreign_toplevel_handle_v1 object\n\nDestroys the zwlr_foreign_toplevel_handle_v1 object.\n\nThis request should be called either when the client does not want to\nuse the toplevel anymore or after the closed event to finalize the\ndestruction of the object.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send(msg);
        }
    }
    #[doc = r" An interface for handling events."]
    pub trait EventHandler {
        #[doc = "title change\n\nThis event is emitted whenever the title of the toplevel changes."]
        fn title(&mut self, object: ZwlrForeignToplevelHandleV1, title: String) {}
        #[doc = "app-id change\n\nThis event is emitted whenever the app-id of the toplevel changes."]
        fn app_id(&mut self, object: ZwlrForeignToplevelHandleV1, app_id: String) {}
        #[doc = "toplevel entered an output\n\nThis event is emitted whenever the toplevel becomes visible on\nthe given output. A toplevel may be visible on multiple outputs."]
        fn output_enter(
            &mut self,
            object: ZwlrForeignToplevelHandleV1,
            output: super::wl_output::WlOutput,
        ) {
        }
        #[doc = "toplevel left an output\n\nThis event is emitted whenever the toplevel stops being visible on\nthe given output. It is guaranteed that an entered-output event\nwith the same output has been emitted before this event."]
        fn output_leave(
            &mut self,
            object: ZwlrForeignToplevelHandleV1,
            output: super::wl_output::WlOutput,
        ) {
        }
        #[doc = "the toplevel state changed\n\nThis event is emitted immediately after the zlw_foreign_toplevel_handle_v1\nis created and each time the toplevel state changes, either because of a\ncompositor action or because of a request in this protocol."]
        fn state(&mut self, object: ZwlrForeignToplevelHandleV1, state: Vec<u8>) {}
        #[doc = "all information about the toplevel has been sent\n\nThis event is sent after all changes in the toplevel state have been\nsent.\n\nThis allows changes to the zwlr_foreign_toplevel_handle_v1 properties\nto be seen as atomic, even if they happen via multiple events."]
        fn done(&mut self, object: ZwlrForeignToplevelHandleV1) {}
        #[doc = "this toplevel has been destroyed\n\nThis event means the toplevel has been destroyed. It is guaranteed there\nwon't be any more events for this zwlr_foreign_toplevel_handle_v1. The\ntoplevel itself becomes inert so any requests will be ignored except the\ndestroy request."]
        fn closed(&mut self, object: ZwlrForeignToplevelHandleV1) {}
    }
    impl<T: EventHandler> HandledBy<T> for ZwlrForeignToplevelHandleV1 {
        #[inline]
        fn handle(__handler: &mut T, event: Event, __object: Self) {
            match event {
                Event::Title { title } => __handler.title(__object, title),
                Event::AppId { app_id } => __handler.app_id(__object, app_id),
                Event::OutputEnter { output } => __handler.output_enter(__object, output),
                Event::OutputLeave { output } => __handler.output_leave(__object, output),
                Event::State { state } => __handler.state(__object, state),
                Event::Done {} => __handler.done(__object),
                Event::Closed {} => __handler.closed(__object),
                Event::__nonexhaustive => unreachable!(),
            }
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_MAXIMIZED_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_UNSET_MAXIMIZED_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_MINIMIZED_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_UNSET_MINIMIZED_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_ACTIVATE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_CLOSE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_RECTANGLE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_TITLE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_APP_ID_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_OUTPUT_ENTER_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_OUTPUT_LEAVE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_STATE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_DONE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_CLOSED_SINCE: u32 = 1u32;
    static mut zwlr_foreign_toplevel_handle_v1_requests_activate_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_seat::wl_seat_interface as *const wl_interface }];
    static mut zwlr_foreign_toplevel_handle_v1_requests_set_rectangle_types: [*const wl_interface;
        5] = [
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwlr_foreign_toplevel_handle_v1_requests: [wl_message; 8] = [
        wl_message {
            name: b"set_maximized\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"unset_maximized\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_minimized\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"unset_minimized\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"activate\0" as *const u8 as *const c_char,
            signature: b"o\0" as *const u8 as *const c_char,
            types: unsafe { &zwlr_foreign_toplevel_handle_v1_requests_activate_types as *const _ },
        },
        wl_message {
            name: b"close\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_rectangle\0" as *const u8 as *const c_char,
            signature: b"oiiii\0" as *const u8 as *const c_char,
            types: unsafe {
                &zwlr_foreign_toplevel_handle_v1_requests_set_rectangle_types as *const _
            },
        },
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    static mut zwlr_foreign_toplevel_handle_v1_events_output_enter_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_output::wl_output_interface as *const wl_interface }];
    static mut zwlr_foreign_toplevel_handle_v1_events_output_leave_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_output::wl_output_interface as *const wl_interface }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwlr_foreign_toplevel_handle_v1_events: [wl_message; 7] = [
        wl_message {
            name: b"title\0" as *const u8 as *const c_char,
            signature: b"s\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"app_id\0" as *const u8 as *const c_char,
            signature: b"s\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"output_enter\0" as *const u8 as *const c_char,
            signature: b"o\0" as *const u8 as *const c_char,
            types: unsafe {
                &zwlr_foreign_toplevel_handle_v1_events_output_enter_types as *const _
            },
        },
        wl_message {
            name: b"output_leave\0" as *const u8 as *const c_char,
            signature: b"o\0" as *const u8 as *const c_char,
            types: unsafe {
                &zwlr_foreign_toplevel_handle_v1_events_output_leave_types as *const _
            },
        },
        wl_message {
            name: b"state\0" as *const u8 as *const c_char,
            signature: b"a\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"done\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"closed\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwlr_foreign_toplevel_handle_v1_interface: wl_interface = wl_interface {
        name: b"zwlr_foreign_toplevel_handle_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 8,
        requests: unsafe { &zwlr_foreign_toplevel_handle_v1_requests as *const _ },
        event_count: 7,
        events: unsafe { &zwlr_foreign_toplevel_handle_v1_events as *const _ },
    };
}
