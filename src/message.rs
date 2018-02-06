//! This module corresponds to `mach/message.h`.

use kern_return::{kern_return_t};
use port::{mach_port_name_t, mach_port_t};
use vm_types::{integer_t, natural_t};

pub type mach_msg_timeout_t = natural_t;

pub type mach_msg_bits_t = ::libc::c_uint;
pub type mach_msg_id_t = integer_t;
pub type mach_msg_size_t = natural_t;

pub type mach_msg_copy_options_t = ::libc::c_uint;
pub type mach_msg_descriptor_type_t = ::libc::c_uint;
pub type mach_msg_type_name_t = ::libc::c_uint;

pub type mach_msg_trailer_type_t = ::libc::c_uint;
pub type mach_msg_trailer_size_t = ::libc::c_uint;

pub type mach_msg_option_t = integer_t;

pub type mach_msg_type_number_t = natural_t;
pub type mach_msg_type_size_t = natural_t;

pub type mach_msg_return_t = kern_return_t;

pub const MACH_MSG_TIMEOUT_NONE: mach_msg_timeout_t = 0;

pub const MACH_MSGH_BITS_ZERO:          mach_msg_bits_t = 0x00000000;

pub const MACH_MSGH_BITS_REMOTE_MASK:   mach_msg_bits_t = 0x0000001f;
pub const MACH_MSGH_BITS_LOCAL_MASK:    mach_msg_bits_t = 0x00001f00;
pub const MACH_MSGH_BITS_VOUCHER_MASK:  mach_msg_bits_t = 0x001f0000;

pub const MACH_MSGH_BITS_PORTS_MASK:    mach_msg_bits_t =
    MACH_MSGH_BITS_REMOTE_MASK | MACH_MSGH_BITS_LOCAL_MASK | MACH_MSGH_BITS_VOUCHER_MASK;

pub const MACH_MSGH_BITS_COMPLEX:       mach_msg_bits_t = 0x80000000;

pub const MACH_MSGH_BITS_USER:          mach_msg_bits_t = 0x801f1f1f;

#[allow(non_snake_case)]
pub fn MACH_MSGH_BITS(remote: mach_msg_bits_t, local: mach_msg_bits_t) -> mach_msg_bits_t {
    remote | (local << 8)
}

pub const MACH_MSG_TYPE_MOVE_RECEIVE      : mach_msg_type_name_t = 16;
pub const MACH_MSG_TYPE_MOVE_SEND         : mach_msg_type_name_t = 17;
pub const MACH_MSG_TYPE_MOVE_SEND_ONCE    : mach_msg_type_name_t = 18;
pub const MACH_MSG_TYPE_COPY_SEND         : mach_msg_type_name_t = 19;
pub const MACH_MSG_TYPE_MAKE_SEND         : mach_msg_type_name_t = 20;
pub const MACH_MSG_TYPE_MAKE_SEND_ONCE    : mach_msg_type_name_t = 21;
pub const MACH_MSG_TYPE_COPY_RECEIVE      : mach_msg_type_name_t = 22;
pub const MACH_MSG_TYPE_DISPOSE_RECEIVE   : mach_msg_type_name_t = 24;
pub const MACH_MSG_TYPE_DISPOSE_SEND      : mach_msg_type_name_t = 25;
pub const MACH_MSG_TYPE_DISPOSE_SEND_ONCE : mach_msg_type_name_t = 26;

pub const MACH_MSG_PHYSICAL_COPY: mach_msg_copy_options_t = 0;
pub const MACH_MSG_VIRTUAL_COPY:  mach_msg_copy_options_t = 1;
pub const MACH_MSG_ALLOCATE:      mach_msg_copy_options_t = 2;

pub const MACH_MSG_PORT_DESCRIPTOR:         mach_msg_descriptor_type_t = 0;
pub const MACH_MSG_OOL_DESCRIPTOR:          mach_msg_descriptor_type_t = 1;
pub const MACH_MSG_OOL_PORTS_DESCRIPTOR:    mach_msg_descriptor_type_t = 2;
pub const MACH_MSG_OOL_VOLATILE_DESCRIPTOR: mach_msg_descriptor_type_t = 3;

pub const MACH_MSG_OPTION_NONE:     mach_msg_option_t = 0x00000000;

pub const MACH_SEND_MSG:            mach_msg_option_t = 0x00000001;
pub const MACH_RCV_MSG:             mach_msg_option_t = 0x00000002;

pub const MACH_RCV_LARGE:           mach_msg_option_t = 0x00000004;
pub const MACH_RCV_LARGE_IDENTITY:  mach_msg_option_t = 0x00000008;

pub const MACH_SEND_TIMEOUT:        mach_msg_option_t = 0x00000010;
pub const MACH_SEND_INTERRUPT:      mach_msg_option_t = 0x00000040;
pub const MACH_SEND_NOTIFY:         mach_msg_option_t = 0x00000080;
pub const MACH_SEND_ALWAYS:         mach_msg_option_t = 0x00010000;
pub const MACH_SEND_TRAILER:        mach_msg_option_t = 0x00020000;
pub const MACH_SEND_NOIMPORTANCE:   mach_msg_option_t = 0x00040000;
pub const MACH_SEND_NODENAP:        mach_msg_option_t = MACH_SEND_NOIMPORTANCE;
pub const MACH_SEND_IMPORTANCE:     mach_msg_option_t = 0x00080000;

pub const MACH_RCV_TIMEOUT:         mach_msg_option_t = 0x00000100;
pub const MACH_RCV_NOTIFY:          mach_msg_option_t = 0x00000200;
pub const MACH_RCV_INTERRUPT:       mach_msg_option_t = 0x00000400;
pub const MACH_RCV_VOUCHER:         mach_msg_option_t = 0x00000800;
pub const MACH_RCV_OVERWRITE:       mach_msg_option_t = 0x00001000;

pub const MACH_MSG_SUCCESS:               mach_msg_return_t = 0x00000000;

pub const MACH_MSG_MASK:                  mach_msg_return_t = 0x00003e00;
pub const MACH_MSG_IPC_SPACE:             mach_msg_return_t = 0x00002000;
pub const MACH_MSG_VM_SPACE:              mach_msg_return_t = 0x00001000;
pub const MACH_MSG_IPC_KERNEL:            mach_msg_return_t = 0x00000800;
pub const MACH_MSG_VM_KERNEL:             mach_msg_return_t = 0x00000400;

pub const MACH_SEND_IN_PROGRESS:          mach_msg_return_t = 0x10000001;
pub const MACH_SEND_INVALID_DATA:         mach_msg_return_t = 0x10000002;
pub const MACH_SEND_INVALID_DEST:         mach_msg_return_t = 0x10000003;
pub const MACH_SEND_TIMED_OUT:            mach_msg_return_t = 0x10000004;
pub const MACH_SEND_INVALID_VOUCHER:      mach_msg_return_t = 0x10000005;
pub const MACH_SEND_INTERRUPTED:          mach_msg_return_t = 0x10000007;
pub const MACH_SEND_MSG_TOO_SMALL:        mach_msg_return_t = 0x10000008;
pub const MACH_SEND_INVALID_REPLY:        mach_msg_return_t = 0x10000009;
pub const MACH_SEND_INVALID_RIGHT:        mach_msg_return_t = 0x1000000a;
pub const MACH_SEND_INVALID_NOTIFY:       mach_msg_return_t = 0x1000000b;
pub const MACH_SEND_INVALID_MEMORY:       mach_msg_return_t = 0x1000000c;
pub const MACH_SEND_NO_BUFFER:            mach_msg_return_t = 0x1000000d;
pub const MACH_SEND_TOO_LARGE:            mach_msg_return_t = 0x1000000e;
pub const MACH_SEND_INVALID_TYPE:         mach_msg_return_t = 0x1000000f;
pub const MACH_SEND_INVALID_HEADER:       mach_msg_return_t = 0x10000010;
pub const MACH_SEND_INVALID_TRAILER:      mach_msg_return_t = 0x10000011;
pub const MACH_SEND_INVALID_RT_OOL_SIZE:  mach_msg_return_t = 0x10000015;

pub const MACH_RCV_IN_PROGRESS:           mach_msg_return_t = 0x10004001;
pub const MACH_RCV_INVALID_NAME:          mach_msg_return_t = 0x10004002;
pub const MACH_RCV_TIMED_OUT:             mach_msg_return_t = 0x10004003;
pub const MACH_RCV_TOO_LARGE:             mach_msg_return_t = 0x10004004;
pub const MACH_RCV_INTERRUPTED:           mach_msg_return_t = 0x10004005;
pub const MACH_RCV_PORT_CHANGED:          mach_msg_return_t = 0x10004006;
pub const MACH_RCV_INVALID_NOTIFY:        mach_msg_return_t = 0x10004007;
pub const MACH_RCV_INVALID_DATA:          mach_msg_return_t = 0x10004008;
pub const MACH_RCV_PORT_DIED:             mach_msg_return_t = 0x10004009;
pub const MACH_RCV_IN_SET:                mach_msg_return_t = 0x1000400a;
pub const MACH_RCV_HEADER_ERROR:          mach_msg_return_t = 0x1000400b;
pub const MACH_RCV_BODY_ERROR:            mach_msg_return_t = 0x1000400c;
pub const MACH_RCV_INVALID_TYPE:          mach_msg_return_t = 0x1000400d;
pub const MACH_RCV_SCATTER_SMALL:         mach_msg_return_t = 0x1000400e;
pub const MACH_RCV_INVALID_TRAILER:       mach_msg_return_t = 0x1000400f;
pub const MACH_RCV_IN_PROGRESS_TIMED:     mach_msg_return_t = 0x10004011;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct mach_msg_header_t {
    pub msgh_bits: mach_msg_bits_t,
    pub msgh_size: mach_msg_size_t,
    pub msgh_remote_port: mach_port_t,
    pub msgh_local_port: mach_port_t,
    pub msgh_voucher_port: mach_port_name_t,
    pub msgh_id: mach_msg_id_t,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct mach_msg_body_t {
    pub msgh_descriptor_count: mach_msg_size_t,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct mach_msg_base_t {
    pub header: mach_msg_header_t,
    pub body: mach_msg_body_t,
}

pub const MACH_MSG_TRAILER_FORMAT_0: mach_msg_trailer_type_t = 0;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct mach_msg_trailer_t {
    pub msgh_trailer_type: mach_msg_trailer_type_t,
    pub msgh_trailer_size: mach_msg_trailer_size_t,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct mach_msg_port_descriptor_t {
    pub name: mach_port_t,
    pub pad1: mach_msg_size_t,
    pub pad2: u16,
    pub disposition: u8, // mach_msg_type_name_t bitfield
    pub type_: u8, // mach_msg_descriptor_type_t bitfield
}

impl mach_msg_port_descriptor_t {
    pub fn new(name: mach_port_t, disposition: mach_msg_type_name_t) -> Self {
        mach_msg_port_descriptor_t {
            name: name,
            pad1: 0,
            pad2: 0,
            disposition: disposition as u8,
            type_: MACH_MSG_PORT_DESCRIPTOR as u8,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct mach_msg_ool_descriptor_t {
    pub address: *mut ::libc::c_void,
    #[cfg(not(target_pointer_width = "64"))]
    pub size: mach_msg_size_t,
    pub deallocate: u8, // boolean_t bitfield
    pub copy: u8, // mach_msg_copy_options_t bitfield
    pub pad1: u8,
    pub type_: u8, // mach_msg_descriptor_type_t bitfield
    #[cfg(target_pointer_width = "64")]
    pub size: mach_msg_size_t,
}

impl mach_msg_ool_descriptor_t {
    pub fn new(address: *mut ::libc::c_void,
               deallocate: bool,
               copy: mach_msg_copy_options_t,
               size: u32)
               -> Self {
        mach_msg_ool_descriptor_t {
            address: address,
            deallocate: if deallocate { 1 } else { 0 },
            copy: copy as u8,
            pad1: 0,
            type_: MACH_MSG_OOL_DESCRIPTOR as u8,
            size: size,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct mach_msg_ool_ports_descriptor_t {
    pub address: *mut ::libc::c_void,
    #[cfg(not(target_pointer_width = "64"))]
    pub count: mach_msg_size_t,
    pub deallocate: u8, // boolean_t bitfield
    pub copy: u8, // mach_msg_copy_options_t bitfield
    pub disposition: u8, // mach_msg_type_name_t bitfield
    pub type_: u8, // mach_msg_descriptor_type_t bitfield
    #[cfg(target_pointer_width = "64")]
    pub count: mach_msg_size_t,
}

extern "C" {
    pub fn mach_msg(msg: *mut mach_msg_header_t,
                    option: mach_msg_option_t,
                    send_size: mach_msg_size_t,
                    recv_size: mach_msg_size_t,
                    recv_name: mach_port_name_t,
                    timeout: mach_msg_timeout_t,
                    notify: mach_port_name_t)
        -> mach_msg_return_t;

    // from mach/mach.h
    pub fn mach_msg_send(msg: *mut mach_msg_header_t) -> mach_msg_return_t;
    pub fn mach_msg_destroy(msg: *mut mach_msg_header_t);
}
