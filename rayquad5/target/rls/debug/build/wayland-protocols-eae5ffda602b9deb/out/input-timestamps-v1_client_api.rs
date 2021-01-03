use std::os::raw::{c_char, c_void};
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const sys::common::wl_interface; 3] = [
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
];
#[doc = "context object for high-resolution input timestamps\n\nA global interface used for requesting high-resolution timestamps\nfor input events."]
pub mod zwp_input_timestamps_manager_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        types_null, AnonymousObject, Argument, ArgumentType, HandledBy, Interface, Message,
        MessageDesc, MessageGroup, NewProxy, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    pub enum Request {
        #[doc = "destroy the input timestamps manager object\n\nInforms the server that the client will no longer be using this\nprotocol object. Existing objects created by this object are not\naffected.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "subscribe to high-resolution keyboard timestamp events\n\nCreates a new input timestamps object that represents a subscription\nto high-resolution timestamp events for all wl_keyboard events that\ncarry a timestamp.\n\nIf the associated wl_keyboard object is invalidated, either through\nclient action (e.g. release) or server-side changes, the input\ntimestamps object becomes inert and the client should destroy it\nby calling zwp_input_timestamps_v1.destroy."]
        GetKeyboardTimestamps {
            id: super::zwp_input_timestamps_v1::ZwpInputTimestampsV1,
            keyboard: super::wl_keyboard::WlKeyboard,
        },
        #[doc = "subscribe to high-resolution pointer timestamp events\n\nCreates a new input timestamps object that represents a subscription\nto high-resolution timestamp events for all wl_pointer events that\ncarry a timestamp.\n\nIf the associated wl_pointer object is invalidated, either through\nclient action (e.g. release) or server-side changes, the input\ntimestamps object becomes inert and the client should destroy it\nby calling zwp_input_timestamps_v1.destroy."]
        GetPointerTimestamps {
            id: super::zwp_input_timestamps_v1::ZwpInputTimestampsV1,
            pointer: super::wl_pointer::WlPointer,
        },
        #[doc = "subscribe to high-resolution touch timestamp events\n\nCreates a new input timestamps object that represents a subscription\nto high-resolution timestamp events for all wl_touch events that\ncarry a timestamp.\n\nIf the associated wl_touch object becomes invalid, either through\nclient action (e.g. release) or server-side changes, the input\ntimestamps object becomes inert and the client should destroy it\nby calling zwp_input_timestamps_v1.destroy."]
        GetTouchTimestamps {
            id: super::zwp_input_timestamps_v1::ZwpInputTimestampsV1,
            touch: super::wl_touch::WlTouch,
        },
        #[doc(hidden)]
        __nonexhaustive,
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "destroy",
                since: 1,
                signature: &[],
            },
            super::MessageDesc {
                name: "get_keyboard_timestamps",
                since: 1,
                signature: &[super::ArgumentType::NewId, super::ArgumentType::Object],
            },
            super::MessageDesc {
                name: "get_pointer_timestamps",
                since: 1,
                signature: &[super::ArgumentType::NewId, super::ArgumentType::Object],
            },
            super::MessageDesc {
                name: "get_touch_timestamps",
                since: 1,
                signature: &[super::ArgumentType::NewId, super::ArgumentType::Object],
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
                Request::Destroy => 0,
                Request::GetKeyboardTimestamps { .. } => 1,
                Request::GetPointerTimestamps { .. } => 2,
                Request::GetTouchTimestamps { .. } => 3,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::__nonexhaustive => unreachable!(),
                Request::Destroy => 1,
                Request::GetKeyboardTimestamps { .. } => 1,
                Request::GetPointerTimestamps { .. } => 1,
                Request::GetTouchTimestamps { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<
                    super::zwp_input_timestamps_v1::ZwpInputTimestampsV1,
                >(version, meta.child())),
                2 => Some(Object::from_interface::<
                    super::zwp_input_timestamps_v1::ZwpInputTimestampsV1,
                >(version, meta.child())),
                3 => Some(Object::from_interface::<
                    super::zwp_input_timestamps_v1::ZwpInputTimestampsV1,
                >(version, meta.child())),
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::__nonexhaustive => unreachable!(),
                Request::Destroy => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: vec![],
                },
                Request::GetKeyboardTimestamps { id, keyboard } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: vec![
                        Argument::NewId(id.as_ref().id()),
                        Argument::Object(keyboard.as_ref().id()),
                    ],
                },
                Request::GetPointerTimestamps { id, pointer } => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: vec![
                        Argument::NewId(id.as_ref().id()),
                        Argument::Object(pointer.as_ref().id()),
                    ],
                },
                Request::GetTouchTimestamps { id, touch } => Message {
                    sender_id: sender_id,
                    opcode: 3,
                    args: vec![
                        Argument::NewId(id.as_ref().id()),
                        Argument::Object(touch.as_ref().id()),
                    ],
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
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                }
                Request::GetKeyboardTimestamps { id, keyboard } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.as_ref().c_ptr() as *mut _;
                    _args_array[1].o = keyboard.as_ref().c_ptr() as *mut _;
                    f(1, &mut _args_array)
                }
                Request::GetPointerTimestamps { id, pointer } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.as_ref().c_ptr() as *mut _;
                    _args_array[1].o = pointer.as_ref().c_ptr() as *mut _;
                    f(2, &mut _args_array)
                }
                Request::GetTouchTimestamps { id, touch } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.as_ref().c_ptr() as *mut _;
                    _args_array[1].o = touch.as_ref().c_ptr() as *mut _;
                    f(3, &mut _args_array)
                }
            }
        }
    }
    pub enum Event {
        #[doc(hidden)]
        __nonexhaustive,
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Event::__nonexhaustive => unreachable!(),
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Event::__nonexhaustive => unreachable!(),
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::__nonexhaustive => unreachable!(),
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
    pub struct ZwpInputTimestampsManagerV1(Proxy<ZwpInputTimestampsManagerV1>);
    impl AsRef<Proxy<ZwpInputTimestampsManagerV1>> for ZwpInputTimestampsManagerV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZwpInputTimestampsManagerV1>> for ZwpInputTimestampsManagerV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZwpInputTimestampsManagerV1(value)
        }
    }
    impl From<ZwpInputTimestampsManagerV1> for Proxy<ZwpInputTimestampsManagerV1> {
        #[inline]
        fn from(value: ZwpInputTimestampsManagerV1) -> Self {
            value.0
        }
    }
    impl Interface for ZwpInputTimestampsManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_input_timestamps_manager_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwp_input_timestamps_manager_v1_interface }
        }
    }
    impl ZwpInputTimestampsManagerV1 {
        #[doc = "destroy the input timestamps manager object\n\nInforms the server that the client will no longer be using this\nprotocol object. Existing objects created by this object are not\naffected.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send(msg);
        }
        #[doc = "subscribe to high-resolution keyboard timestamp events\n\nCreates a new input timestamps object that represents a subscription\nto high-resolution timestamp events for all wl_keyboard events that\ncarry a timestamp.\n\nIf the associated wl_keyboard object is invalidated, either through\nclient action (e.g. release) or server-side changes, the input\ntimestamps object becomes inert and the client should destroy it\nby calling zwp_input_timestamps_v1.destroy."]
        pub fn get_keyboard_timestamps<F>(
            &self,
            keyboard: &super::wl_keyboard::WlKeyboard,
            implementor: F,
        ) -> Result<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1, ()>
        where
            F: FnOnce(
                NewProxy<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1>,
            ) -> super::zwp_input_timestamps_v1::ZwpInputTimestampsV1,
        {
            let msg = Request::GetKeyboardTimestamps {
                id: self.0.child_placeholder(),
                keyboard: keyboard.clone(),
            };
            self.0.send_constructor(msg, implementor, None)
        }
        #[doc = "subscribe to high-resolution pointer timestamp events\n\nCreates a new input timestamps object that represents a subscription\nto high-resolution timestamp events for all wl_pointer events that\ncarry a timestamp.\n\nIf the associated wl_pointer object is invalidated, either through\nclient action (e.g. release) or server-side changes, the input\ntimestamps object becomes inert and the client should destroy it\nby calling zwp_input_timestamps_v1.destroy."]
        pub fn get_pointer_timestamps<F>(
            &self,
            pointer: &super::wl_pointer::WlPointer,
            implementor: F,
        ) -> Result<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1, ()>
        where
            F: FnOnce(
                NewProxy<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1>,
            ) -> super::zwp_input_timestamps_v1::ZwpInputTimestampsV1,
        {
            let msg = Request::GetPointerTimestamps {
                id: self.0.child_placeholder(),
                pointer: pointer.clone(),
            };
            self.0.send_constructor(msg, implementor, None)
        }
        #[doc = "subscribe to high-resolution touch timestamp events\n\nCreates a new input timestamps object that represents a subscription\nto high-resolution timestamp events for all wl_touch events that\ncarry a timestamp.\n\nIf the associated wl_touch object becomes invalid, either through\nclient action (e.g. release) or server-side changes, the input\ntimestamps object becomes inert and the client should destroy it\nby calling zwp_input_timestamps_v1.destroy."]
        pub fn get_touch_timestamps<F>(
            &self,
            touch: &super::wl_touch::WlTouch,
            implementor: F,
        ) -> Result<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1, ()>
        where
            F: FnOnce(
                NewProxy<super::zwp_input_timestamps_v1::ZwpInputTimestampsV1>,
            ) -> super::zwp_input_timestamps_v1::ZwpInputTimestampsV1,
        {
            let msg = Request::GetTouchTimestamps {
                id: self.0.child_placeholder(),
                touch: touch.clone(),
            };
            self.0.send_constructor(msg, implementor, None)
        }
    }
    #[doc = r" An interface for handling events."]
    pub trait EventHandler {}
    impl<T: EventHandler> HandledBy<T> for ZwpInputTimestampsManagerV1 {
        #[inline]
        fn handle(__handler: &mut T, event: Event, __object: Self) {
            match event {
                Event::__nonexhaustive => unreachable!(),
            }
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_KEYBOARD_TIMESTAMPS_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_POINTER_TIMESTAMPS_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_TOUCH_TIMESTAMPS_SINCE: u32 = 1u32;
    static mut zwp_input_timestamps_manager_v1_requests_get_keyboard_timestamps_types:
        [*const wl_interface; 2] = [
        unsafe {
            &super::zwp_input_timestamps_v1::zwp_input_timestamps_v1_interface
                as *const wl_interface
        },
        unsafe { &super::wl_keyboard::wl_keyboard_interface as *const wl_interface },
    ];
    static mut zwp_input_timestamps_manager_v1_requests_get_pointer_timestamps_types:
        [*const wl_interface; 2] = [
        unsafe {
            &super::zwp_input_timestamps_v1::zwp_input_timestamps_v1_interface
                as *const wl_interface
        },
        unsafe { &super::wl_pointer::wl_pointer_interface as *const wl_interface },
    ];
    static mut zwp_input_timestamps_manager_v1_requests_get_touch_timestamps_types:
        [*const wl_interface; 2] = [
        unsafe {
            &super::zwp_input_timestamps_v1::zwp_input_timestamps_v1_interface
                as *const wl_interface
        },
        unsafe { &super::wl_touch::wl_touch_interface as *const wl_interface },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_input_timestamps_manager_v1_requests: [wl_message; 4] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"get_keyboard_timestamps\0" as *const u8 as *const c_char,
            signature: b"no\0" as *const u8 as *const c_char,
            types: unsafe {
                &zwp_input_timestamps_manager_v1_requests_get_keyboard_timestamps_types as *const _
            },
        },
        wl_message {
            name: b"get_pointer_timestamps\0" as *const u8 as *const c_char,
            signature: b"no\0" as *const u8 as *const c_char,
            types: unsafe {
                &zwp_input_timestamps_manager_v1_requests_get_pointer_timestamps_types as *const _
            },
        },
        wl_message {
            name: b"get_touch_timestamps\0" as *const u8 as *const c_char,
            signature: b"no\0" as *const u8 as *const c_char,
            types: unsafe {
                &zwp_input_timestamps_manager_v1_requests_get_touch_timestamps_types as *const _
            },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwp_input_timestamps_manager_v1_interface: wl_interface = wl_interface {
        name: b"zwp_input_timestamps_manager_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 4,
        requests: unsafe { &zwp_input_timestamps_manager_v1_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "context object for input timestamps\n\nProvides high-resolution timestamp events for a set of subscribed input\nevents. The set of subscribed input events is determined by the\nzwp_input_timestamps_manager_v1 request used to create this object."]
pub mod zwp_input_timestamps_v1 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        types_null, AnonymousObject, Argument, ArgumentType, HandledBy, Interface, Message,
        MessageDesc, MessageGroup, NewProxy, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    pub enum Request {
        #[doc = "destroy the input timestamps object\n\nInforms the server that the client will no longer be using this\nprotocol object. After the server processes the request, no more\ntimestamp events will be emitted.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc(hidden)]
        __nonexhaustive,
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "destroy",
            since: 1,
            signature: &[],
        }];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::__nonexhaustive => unreachable!(),
                Request::Destroy => true,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::__nonexhaustive => unreachable!(),
                Request::Destroy => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::__nonexhaustive => unreachable!(),
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
                Request::Destroy => Message {
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
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                }
            }
        }
    }
    pub enum Event {
        #[doc = "high-resolution timestamp event\n\nThe timestamp event is associated with the first subsequent input event\ncarrying a timestamp which belongs to the set of input events this\nobject is subscribed to.\n\nThe timestamp provided by this event is a high-resolution version of\nthe timestamp argument of the associated input event. The provided\ntimestamp is in the same clock domain and is at least as accurate as\nthe associated input event timestamp.\n\nThe timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,\neach component being an unsigned 32-bit value. Whole seconds are in\ntv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,\nand the additional fractional part in tv_nsec as nanoseconds. Hence,\nfor valid timestamps tv_nsec must be in [0, 999999999]."]
        Timestamp {
            tv_sec_hi: u32,
            tv_sec_lo: u32,
            tv_nsec: u32,
        },
        #[doc(hidden)]
        __nonexhaustive,
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "timestamp",
            since: 1,
            signature: &[
                super::ArgumentType::Uint,
                super::ArgumentType::Uint,
                super::ArgumentType::Uint,
            ],
        }];
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
                Event::Timestamp { .. } => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::__nonexhaustive => unreachable!(),
                Event::Timestamp { .. } => 1,
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
                    Ok(Event::Timestamp {
                        tv_sec_hi: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        tv_sec_lo: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        tv_nsec: {
                            if let Some(Argument::Uint(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
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
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Timestamp {
                        tv_sec_hi: _args[0].u,
                        tv_sec_lo: _args[1].u,
                        tv_nsec: _args[2].u,
                    })
                }
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
    pub struct ZwpInputTimestampsV1(Proxy<ZwpInputTimestampsV1>);
    impl AsRef<Proxy<ZwpInputTimestampsV1>> for ZwpInputTimestampsV1 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZwpInputTimestampsV1>> for ZwpInputTimestampsV1 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZwpInputTimestampsV1(value)
        }
    }
    impl From<ZwpInputTimestampsV1> for Proxy<ZwpInputTimestampsV1> {
        #[inline]
        fn from(value: ZwpInputTimestampsV1) -> Self {
            value.0
        }
    }
    impl Interface for ZwpInputTimestampsV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_input_timestamps_v1";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwp_input_timestamps_v1_interface }
        }
    }
    impl ZwpInputTimestampsV1 {
        #[doc = "destroy the input timestamps object\n\nInforms the server that the client will no longer be using this\nprotocol object. After the server processes the request, no more\ntimestamp events will be emitted.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send(msg);
        }
    }
    #[doc = r" An interface for handling events."]
    pub trait EventHandler {
        #[doc = "high-resolution timestamp event\n\nThe timestamp event is associated with the first subsequent input event\ncarrying a timestamp which belongs to the set of input events this\nobject is subscribed to.\n\nThe timestamp provided by this event is a high-resolution version of\nthe timestamp argument of the associated input event. The provided\ntimestamp is in the same clock domain and is at least as accurate as\nthe associated input event timestamp.\n\nThe timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,\neach component being an unsigned 32-bit value. Whole seconds are in\ntv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,\nand the additional fractional part in tv_nsec as nanoseconds. Hence,\nfor valid timestamps tv_nsec must be in [0, 999999999]."]
        fn timestamp(
            &mut self,
            object: ZwpInputTimestampsV1,
            tv_sec_hi: u32,
            tv_sec_lo: u32,
            tv_nsec: u32,
        ) {
        }
    }
    impl<T: EventHandler> HandledBy<T> for ZwpInputTimestampsV1 {
        #[inline]
        fn handle(__handler: &mut T, event: Event, __object: Self) {
            match event {
                Event::Timestamp {
                    tv_sec_hi,
                    tv_sec_lo,
                    tv_nsec,
                } => __handler.timestamp(__object, tv_sec_hi, tv_sec_lo, tv_nsec),
                Event::__nonexhaustive => unreachable!(),
            }
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_TIMESTAMP_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_input_timestamps_v1_requests: [wl_message; 1] = [wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwp_input_timestamps_v1_events: [wl_message; 1] = [wl_message {
        name: b"timestamp\0" as *const u8 as *const c_char,
        signature: b"uuu\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwp_input_timestamps_v1_interface: wl_interface = wl_interface {
        name: b"zwp_input_timestamps_v1\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 1,
        requests: unsafe { &zwp_input_timestamps_v1_requests as *const _ },
        event_count: 1,
        events: unsafe { &zwp_input_timestamps_v1_events as *const _ },
    };
}
