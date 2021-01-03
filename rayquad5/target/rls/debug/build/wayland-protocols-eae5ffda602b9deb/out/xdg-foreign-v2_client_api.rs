use std::os::raw::{c_char, c_void};
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const sys::common::wl_interface; 1] =
    [NULLPTR as *const sys::common::wl_interface];
#[doc = "interface for exporting surfaces\n\nA global interface used for exporting surfaces that can later be imported\nusing xdg_importer."]
pub mod zxdg_exporter_v2 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        types_null, AnonymousObject, Argument, ArgumentType, HandledBy, Interface, Message,
        MessageDesc, MessageGroup, NewProxy, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    pub enum Request {
        #[doc = "destroy the xdg_exporter object\n\nNotify the compositor that the xdg_exporter object will no longer be\nused.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "export a toplevel surface\n\nThe export_toplevel request exports the passed surface so that it can later be\nimported via xdg_importer. When called, a new xdg_exported object will\nbe created and xdg_exported.handle will be sent immediately. See the\ncorresponding interface and event for details.\n\nA surface may be exported multiple times, and each exported handle may\nbe used to create a xdg_imported multiple times. Only xdg_toplevel\nequivalent surfaces may be exported."]
        ExportToplevel {
            id: super::zxdg_exported_v2::ZxdgExportedV2,
            surface: super::wl_surface::WlSurface,
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
                name: "export_toplevel",
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
                Request::ExportToplevel { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::__nonexhaustive => unreachable!(),
                Request::Destroy => 1,
                Request::ExportToplevel { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<
                    super::zxdg_exported_v2::ZxdgExportedV2,
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
                Request::ExportToplevel { id, surface } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: vec![
                        Argument::NewId(id.as_ref().id()),
                        Argument::Object(surface.as_ref().id()),
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
                Request::ExportToplevel { id, surface } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.as_ref().c_ptr() as *mut _;
                    _args_array[1].o = surface.as_ref().c_ptr() as *mut _;
                    f(1, &mut _args_array)
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
    pub struct ZxdgExporterV2(Proxy<ZxdgExporterV2>);
    impl AsRef<Proxy<ZxdgExporterV2>> for ZxdgExporterV2 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZxdgExporterV2>> for ZxdgExporterV2 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZxdgExporterV2(value)
        }
    }
    impl From<ZxdgExporterV2> for Proxy<ZxdgExporterV2> {
        #[inline]
        fn from(value: ZxdgExporterV2) -> Self {
            value.0
        }
    }
    impl Interface for ZxdgExporterV2 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zxdg_exporter_v2";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zxdg_exporter_v2_interface }
        }
    }
    impl ZxdgExporterV2 {
        #[doc = "destroy the xdg_exporter object\n\nNotify the compositor that the xdg_exporter object will no longer be\nused.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send(msg);
        }
        #[doc = "export a toplevel surface\n\nThe export_toplevel request exports the passed surface so that it can later be\nimported via xdg_importer. When called, a new xdg_exported object will\nbe created and xdg_exported.handle will be sent immediately. See the\ncorresponding interface and event for details.\n\nA surface may be exported multiple times, and each exported handle may\nbe used to create a xdg_imported multiple times. Only xdg_toplevel\nequivalent surfaces may be exported."]
        pub fn export_toplevel<F>(
            &self,
            surface: &super::wl_surface::WlSurface,
            implementor: F,
        ) -> Result<super::zxdg_exported_v2::ZxdgExportedV2, ()>
        where
            F: FnOnce(
                NewProxy<super::zxdg_exported_v2::ZxdgExportedV2>,
            ) -> super::zxdg_exported_v2::ZxdgExportedV2,
        {
            let msg = Request::ExportToplevel {
                id: self.0.child_placeholder(),
                surface: surface.clone(),
            };
            self.0.send_constructor(msg, implementor, None)
        }
    }
    #[doc = r" An interface for handling events."]
    pub trait EventHandler {}
    impl<T: EventHandler> HandledBy<T> for ZxdgExporterV2 {
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
    pub const REQ_EXPORT_TOPLEVEL_SINCE: u32 = 1u32;
    static mut zxdg_exporter_v2_requests_export_toplevel_types: [*const wl_interface; 2] = [
        unsafe { &super::zxdg_exported_v2::zxdg_exported_v2_interface as *const wl_interface },
        unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zxdg_exporter_v2_requests: [wl_message; 2] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"export_toplevel\0" as *const u8 as *const c_char,
            signature: b"no\0" as *const u8 as *const c_char,
            types: unsafe { &zxdg_exporter_v2_requests_export_toplevel_types as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zxdg_exporter_v2_interface: wl_interface = wl_interface {
        name: b"zxdg_exporter_v2\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 2,
        requests: unsafe { &zxdg_exporter_v2_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "interface for importing surfaces\n\nA global interface used for importing surfaces exported by xdg_exporter.\nWith this interface, a client can create a reference to a surface of\nanother client."]
pub mod zxdg_importer_v2 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        types_null, AnonymousObject, Argument, ArgumentType, HandledBy, Interface, Message,
        MessageDesc, MessageGroup, NewProxy, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    pub enum Request {
        #[doc = "destroy the xdg_importer object\n\nNotify the compositor that the xdg_importer object will no longer be\nused.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "import a toplevel surface\n\nThe import_toplevel request imports a surface from any client given a handle\nretrieved by exporting said surface using xdg_exporter.export_toplevel.\nWhen called, a new xdg_imported object will be created. This new object\nrepresents the imported surface, and the importing client can\nmanipulate its relationship using it. See xdg_imported for details."]
        ImportToplevel {
            id: super::zxdg_imported_v2::ZxdgImportedV2,
            handle: String,
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
                name: "import_toplevel",
                since: 1,
                signature: &[super::ArgumentType::NewId, super::ArgumentType::Str],
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
                Request::ImportToplevel { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::__nonexhaustive => unreachable!(),
                Request::Destroy => 1,
                Request::ImportToplevel { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                1 => Some(Object::from_interface::<
                    super::zxdg_imported_v2::ZxdgImportedV2,
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
                Request::ImportToplevel { id, handle } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: vec![
                        Argument::NewId(id.as_ref().id()),
                        Argument::Str(unsafe {
                            ::std::ffi::CString::from_vec_unchecked(handle.into())
                        }),
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
                Request::ImportToplevel { id, handle } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.as_ref().c_ptr() as *mut _;
                    let _arg_1 = ::std::ffi::CString::new(handle).unwrap();
                    _args_array[1].s = _arg_1.as_ptr();
                    f(1, &mut _args_array)
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
    pub struct ZxdgImporterV2(Proxy<ZxdgImporterV2>);
    impl AsRef<Proxy<ZxdgImporterV2>> for ZxdgImporterV2 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZxdgImporterV2>> for ZxdgImporterV2 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZxdgImporterV2(value)
        }
    }
    impl From<ZxdgImporterV2> for Proxy<ZxdgImporterV2> {
        #[inline]
        fn from(value: ZxdgImporterV2) -> Self {
            value.0
        }
    }
    impl Interface for ZxdgImporterV2 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zxdg_importer_v2";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zxdg_importer_v2_interface }
        }
    }
    impl ZxdgImporterV2 {
        #[doc = "destroy the xdg_importer object\n\nNotify the compositor that the xdg_importer object will no longer be\nused.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send(msg);
        }
        #[doc = "import a toplevel surface\n\nThe import_toplevel request imports a surface from any client given a handle\nretrieved by exporting said surface using xdg_exporter.export_toplevel.\nWhen called, a new xdg_imported object will be created. This new object\nrepresents the imported surface, and the importing client can\nmanipulate its relationship using it. See xdg_imported for details."]
        pub fn import_toplevel<F>(
            &self,
            handle: String,
            implementor: F,
        ) -> Result<super::zxdg_imported_v2::ZxdgImportedV2, ()>
        where
            F: FnOnce(
                NewProxy<super::zxdg_imported_v2::ZxdgImportedV2>,
            ) -> super::zxdg_imported_v2::ZxdgImportedV2,
        {
            let msg = Request::ImportToplevel {
                id: self.0.child_placeholder(),
                handle: handle,
            };
            self.0.send_constructor(msg, implementor, None)
        }
    }
    #[doc = r" An interface for handling events."]
    pub trait EventHandler {}
    impl<T: EventHandler> HandledBy<T> for ZxdgImporterV2 {
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
    pub const REQ_IMPORT_TOPLEVEL_SINCE: u32 = 1u32;
    static mut zxdg_importer_v2_requests_import_toplevel_types: [*const wl_interface; 2] = [
        unsafe { &super::zxdg_imported_v2::zxdg_imported_v2_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zxdg_importer_v2_requests: [wl_message; 2] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"import_toplevel\0" as *const u8 as *const c_char,
            signature: b"ns\0" as *const u8 as *const c_char,
            types: unsafe { &zxdg_importer_v2_requests_import_toplevel_types as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zxdg_importer_v2_interface: wl_interface = wl_interface {
        name: b"zxdg_importer_v2\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 2,
        requests: unsafe { &zxdg_importer_v2_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "an exported surface handle\n\nA xdg_exported object represents an exported reference to a surface. The\nexported surface may be referenced as long as the xdg_exported object not\ndestroyed. Destroying the xdg_exported invalidates any relationship the\nimporter may have established using xdg_imported."]
pub mod zxdg_exported_v2 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        types_null, AnonymousObject, Argument, ArgumentType, HandledBy, Interface, Message,
        MessageDesc, MessageGroup, NewProxy, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    pub enum Request {
        #[doc = "unexport the exported surface\n\nRevoke the previously exported surface. This invalidates any\nrelationship the importer may have set up using the xdg_imported created\ngiven the handle sent via xdg_exported.handle.\n\nThis is a destructor, once sent this object cannot be used any longer."]
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
        #[doc = "the exported surface handle\n\nThe handle event contains the unique handle of this exported surface\nreference. It may be shared with any client, which then can use it to\nimport the surface by calling xdg_importer.import_toplevel. A handle\nmay be used to import the surface multiple times."]
        Handle { handle: String },
        #[doc(hidden)]
        __nonexhaustive,
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "handle",
            since: 1,
            signature: &[super::ArgumentType::Str],
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
                Event::Handle { .. } => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::__nonexhaustive => unreachable!(),
                Event::Handle { .. } => 1,
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
                    Ok(Event::Handle {
                        handle: {
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
                    Ok(Event::Handle {
                        handle: ::std::ffi::CStr::from_ptr(_args[0].s)
                            .to_string_lossy()
                            .into_owned(),
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
    pub struct ZxdgExportedV2(Proxy<ZxdgExportedV2>);
    impl AsRef<Proxy<ZxdgExportedV2>> for ZxdgExportedV2 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZxdgExportedV2>> for ZxdgExportedV2 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZxdgExportedV2(value)
        }
    }
    impl From<ZxdgExportedV2> for Proxy<ZxdgExportedV2> {
        #[inline]
        fn from(value: ZxdgExportedV2) -> Self {
            value.0
        }
    }
    impl Interface for ZxdgExportedV2 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zxdg_exported_v2";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zxdg_exported_v2_interface }
        }
    }
    impl ZxdgExportedV2 {
        #[doc = "unexport the exported surface\n\nRevoke the previously exported surface. This invalidates any\nrelationship the importer may have set up using the xdg_imported created\ngiven the handle sent via xdg_exported.handle.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send(msg);
        }
    }
    #[doc = r" An interface for handling events."]
    pub trait EventHandler {
        #[doc = "the exported surface handle\n\nThe handle event contains the unique handle of this exported surface\nreference. It may be shared with any client, which then can use it to\nimport the surface by calling xdg_importer.import_toplevel. A handle\nmay be used to import the surface multiple times."]
        fn handle(&mut self, object: ZxdgExportedV2, handle: String) {}
    }
    impl<T: EventHandler> HandledBy<T> for ZxdgExportedV2 {
        #[inline]
        fn handle(__handler: &mut T, event: Event, __object: Self) {
            match event {
                Event::Handle { handle } => __handler.handle(__object, handle),
                Event::__nonexhaustive => unreachable!(),
            }
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_HANDLE_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zxdg_exported_v2_requests: [wl_message; 1] = [wl_message {
        name: b"destroy\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zxdg_exported_v2_events: [wl_message; 1] = [wl_message {
        name: b"handle\0" as *const u8 as *const c_char,
        signature: b"s\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zxdg_exported_v2_interface: wl_interface = wl_interface {
        name: b"zxdg_exported_v2\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 1,
        requests: unsafe { &zxdg_exported_v2_requests as *const _ },
        event_count: 1,
        events: unsafe { &zxdg_exported_v2_events as *const _ },
    };
}
#[doc = "an imported surface handle\n\nA xdg_imported object represents an imported reference to surface exported\nby some client. A client can use this interface to manipulate\nrelationships between its own surfaces and the imported surface."]
pub mod zxdg_imported_v2 {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        types_null, AnonymousObject, Argument, ArgumentType, HandledBy, Interface, Message,
        MessageDesc, MessageGroup, NewProxy, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    pub enum Request {
        #[doc = "destroy the xdg_imported object\n\nNotify the compositor that it will no longer use the xdg_imported\nobject. Any relationship that may have been set up will at this point\nbe invalidated.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Destroy,
        #[doc = "set as the parent of some surface\n\nSet the imported surface as the parent of some surface of the client.\nThe passed surface must be a xdg_toplevel equivalent. Calling this\nfunction sets up a surface to surface relation with the same stacking\nand positioning semantics as xdg_toplevel.set_parent."]
        SetParentOf {
            surface: super::wl_surface::WlSurface,
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
                name: "set_parent_of",
                since: 1,
                signature: &[super::ArgumentType::Object],
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
                Request::SetParentOf { .. } => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::__nonexhaustive => unreachable!(),
                Request::Destroy => 1,
                Request::SetParentOf { .. } => 1,
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
                Request::SetParentOf { surface } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: vec![Argument::Object(surface.as_ref().id())],
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
                Request::SetParentOf { surface } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = surface.as_ref().c_ptr() as *mut _;
                    f(1, &mut _args_array)
                }
            }
        }
    }
    pub enum Event {
        #[doc = "the imported surface handle has been destroyed\n\nThe imported surface handle has been destroyed and any relationship set\nup has been invalidated. This may happen for various reasons, for\nexample if the exported surface or the exported surface handle has been\ndestroyed, if the handle used for importing was invalid."]
        Destroyed,
        #[doc(hidden)]
        __nonexhaustive,
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "destroyed",
            since: 1,
            signature: &[],
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
                Event::Destroyed => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::__nonexhaustive => unreachable!(),
                Event::Destroyed => 1,
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
                0 => Ok(Event::Destroyed),
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
                0 => Ok(Event::Destroyed),
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
    pub struct ZxdgImportedV2(Proxy<ZxdgImportedV2>);
    impl AsRef<Proxy<ZxdgImportedV2>> for ZxdgImportedV2 {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<ZxdgImportedV2>> for ZxdgImportedV2 {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            ZxdgImportedV2(value)
        }
    }
    impl From<ZxdgImportedV2> for Proxy<ZxdgImportedV2> {
        #[inline]
        fn from(value: ZxdgImportedV2) -> Self {
            value.0
        }
    }
    impl Interface for ZxdgImportedV2 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zxdg_imported_v2";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &zxdg_imported_v2_interface }
        }
    }
    impl ZxdgImportedV2 {
        #[doc = "destroy the xdg_imported object\n\nNotify the compositor that it will no longer use the xdg_imported\nobject. Any relationship that may have been set up will at this point\nbe invalidated.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn destroy(&self) -> () {
            let msg = Request::Destroy;
            self.0.send(msg);
        }
        #[doc = "set as the parent of some surface\n\nSet the imported surface as the parent of some surface of the client.\nThe passed surface must be a xdg_toplevel equivalent. Calling this\nfunction sets up a surface to surface relation with the same stacking\nand positioning semantics as xdg_toplevel.set_parent."]
        pub fn set_parent_of(&self, surface: &super::wl_surface::WlSurface) -> () {
            let msg = Request::SetParentOf {
                surface: surface.clone(),
            };
            self.0.send(msg);
        }
    }
    #[doc = r" An interface for handling events."]
    pub trait EventHandler {
        #[doc = "the imported surface handle has been destroyed\n\nThe imported surface handle has been destroyed and any relationship set\nup has been invalidated. This may happen for various reasons, for\nexample if the exported surface or the exported surface handle has been\ndestroyed, if the handle used for importing was invalid."]
        fn destroyed(&mut self, object: ZxdgImportedV2) {}
    }
    impl<T: EventHandler> HandledBy<T> for ZxdgImportedV2 {
        #[inline]
        fn handle(__handler: &mut T, event: Event, __object: Self) {
            match event {
                Event::Destroyed {} => __handler.destroyed(__object),
                Event::__nonexhaustive => unreachable!(),
            }
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SET_PARENT_OF_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_DESTROYED_SINCE: u32 = 1u32;
    static mut zxdg_imported_v2_requests_set_parent_of_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_surface::wl_surface_interface as *const wl_interface }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zxdg_imported_v2_requests: [wl_message; 2] = [
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"set_parent_of\0" as *const u8 as *const c_char,
            signature: b"o\0" as *const u8 as *const c_char,
            types: unsafe { &zxdg_imported_v2_requests_set_parent_of_types as *const _ },
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zxdg_imported_v2_events: [wl_message; 1] = [wl_message {
        name: b"destroyed\0" as *const u8 as *const c_char,
        signature: b"\0" as *const u8 as *const c_char,
        types: unsafe { &types_null as *const _ },
    }];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zxdg_imported_v2_interface: wl_interface = wl_interface {
        name: b"zxdg_imported_v2\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 2,
        requests: unsafe { &zxdg_imported_v2_requests as *const _ },
        event_count: 1,
        events: unsafe { &zxdg_imported_v2_events as *const _ },
    };
}
