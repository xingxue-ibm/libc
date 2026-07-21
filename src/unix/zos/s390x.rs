use crate::{
    c_char, c_int, c_long, c_short, c_uint, c_ulong, c_ushort, c_void, off_t, size_t,
    c_double, c_uchar,
};

pub type wchar_t = c_uint;

pub type pthread_rwlockattr_t = [c_char; 8];
pub type pthread_condattr_t = [c_char; 8];
pub type pthread_mutexattr_t = [c_char; 8];
pub type pthread_attr_t = [c_double; 13];

pub type mcontext_t =  [c_long; 85];

// sys/resource.h: _LP64 branch
pub type rlim_t = c_ulong;

s! {
    pub struct msghdr {
        pub msg_name: *mut c_void,
        pub msg_iov: *mut crate::iovec,
        pub msg_control: *mut c_void,
        pub msg_flags: c_int,
        pub msg_namelen: crate::socklen_t,
        pub msg_iovlen: c_int,
        pub msg_controllen: crate::socklen_t,
    }

    pub struct addrinfo {
        pub ai_flags: c_int,
        pub ai_family: c_int,
        pub ai_socktype: c_int,
        pub ai_protocol: c_int,
        pub ai_addrlen: c_uint,
        __ai_reserved: c_int,
        pub ai_canonname: *mut c_char,
        pub ai_addr: *mut crate::sockaddr,
        pub ai_next: *mut addrinfo,
        pub ai_eflags: c_int,
    }

    #[repr(C, packed(4))] // Needed for off_t fields.
    pub struct sf_parms {
        pub socket_descriptor: c_int,
        pub header_length: c_int,
        pub header_alet: c_int,
        pub header_data_31: [c_char; 4],
        pub file_descriptor: c_int,
        pub file_bytes: off_t,
        pub file_offset: off_t,
        pub file_size: off_t,
        pub trailer_length: c_int,
        pub trailer_alet: c_int,
        pub trailer_data_31: [c_char; 4],
        pub bytes_sent: off_t,
        pub options: c_int,
        pub rsv1: [c_char; 12],
        pub header_data: *mut c_void,
        pub trailer_data: *mut c_void,
    }

    pub struct shmid_ds {
        pub shm_perm: crate::ipc_perm,
        pub shm_segsz_31: [c_char; 4],
        pub shm_lpid: crate::pid_t,
        pub shm_cpid: crate::pid_t,
        pub shm_nattch: crate::shmatt_t,
        pub shm_atime_31: [c_char; 4],
        pub shm_dtime_31: [c_char; 4],
        pub shm_ctime_31: [c_char; 4],
        shm_rsvd1: c_int,
        // `sys/shm.h` uses bitfields here on modern z/OS LE:
        // unsigned shm_seg64 : 1;
        // unsigned shm_rsvd2 : 7;
        // unsigned char shm_dump_prio64;
        // unsigned short shm_rsvd3;
        // Model the entire 32-bit storage unit directly and provide accessors.
        shm_flags: c_uint,
        pub shm_segaddr64: *mut c_char,
        pub shm_segsz: size_t,
        pub shm_atime: crate::time_t,
        pub shm_dtime: crate::time_t,
        pub shm_ctime: crate::time_t,
    }

    #[repr(C, packed(4))] // Needed for off_t fields.
    pub struct flock {
        pub l_type: c_short,
        pub l_whence: c_short,
        pub l_start: off_t,
        pub l_len: off_t,
        pub l_pid: crate::pid_t,
    }

    pub struct statvfs {
        pub f_OEcbid: [c_char; 4],
        pub f_OEcblen: c_int,
        pub f_bsize: c_ulong,
        pub f_blocks: crate::fsblkcnt_t,
        pub f_OEusedspace: crate::fsblkcnt_t,
        pub f_bavail: crate::fsblkcnt_t,
        pub f_flag: c_ulong,
        pub f_OEmaxfilesize_u: [c_char; 8],
        pub f_reserved7: [c_char; 16],
        pub f_frsize: c_ulong,
        pub f_bfree: crate::fsblkcnt_t,
        pub f_files: crate::fsfilcnt_t,
        pub f_ffree: crate::fsfilcnt_t,
        pub f_favail: crate::fsfilcnt_t,
        pub f_namemax_31: c_uint,
        pub f_OEinvarsec: c_uint,
        pub f_reserved10: [c_char; 4],
        pub f_fsid: c_ulong,
        pub f_namemax: c_ulong,
    }

    pub struct pthread_rwlock_t {
        __: [c_char; 8],
    }

    pub struct stat {
        pub st_eye: [c_char; 4],
        pub st_length: c_ushort,
        pub st_version: c_ushort,
        pub st_mode: crate::mode_t,
        pub st_ino: crate::ino_t,
        pub st_dev: crate::dev_t,
        pub st_nlink: crate::nlink_t,
        pub st_uid: crate::uid_t,
        pub st_gid: crate::gid_t,
        pub st_size: crate::off_t,
        pub st_atime_31: [c_char; 4],
        pub st_mtime_31: [c_char; 4],
        pub st_ctime_31: [c_char; 4],
        pub st_rdev: crate::dev_t,
        pub st_auditoraudit: c_uint,
        pub st_useraudit: c_uint,
        pub st_blksize: crate::blksize_t,
        pub st_createtime_31: [c_char; 4],
        pub st_auditid: [c_char; 16],
        pub st_rsrvd1: [c_char; 4],
        pub st_charsetid: [c_char; 12],
        pub st_blocks: crate::blkcnt_t,
        pub st_genvalue: c_uint,
        pub st_reftime_31: [c_char; 4],
        pub st_fid: [c_char; 8],
        pub st_filefmt: c_char,
        pub st_fspflag2: c_char,
        pub st_rsrvd2: [c_char; 2],
        pub st_ctimemsec: c_int,
        pub st_seclabel: [c_char; 8],
        pub st_rsrvd3: [c_char; 4],
        pub st_rsrvd4: [c_char; 4],
        pub st_atime: crate::time_t,
        pub st_mtime: crate::time_t,
        pub st_ctime: crate::time_t,
        pub st_createtime: crate::time_t,
        pub st_reftime: crate::time_t,
        pub st_rsrvd5: [c_char; 24],
    }

    pub struct aiocb {
        pub aio_fildes: c_int,
        pub aio_buffalet: c_int,
        pub aio_nbytes: size_t,
        pub aio_offset: off_t,
        pub __aio_u1: __aio_u1_packed,
        pub aio_reqprio: c_int,
        pub aio_lio_opcode: c_int,
        pub aio_notifytype: c_short,
        pub aio_cflags: c_char,
        pub aio_cflags2: c_char,
        pub aio_msgiovalet: c_int,
        pub aio_iovbufalet: c_int,
        pub aio_rv: c_int, // volatile
        pub aio_rc: c_int, // volatile
        pub aio_rsn: c_int, // volatile
        pub aio_posixflags: c_int,
        pub aio_exitptr_31: [c_char; 4],
        pub aio_exitdata: [c_char; 8],
        pub aio_ecbptr: c_uint,
        pub aio_sockaddrlen: c_int,
        pub aio_sockaddrptr_31: [c_char; 4],
        pub aio_timeout: c_int,
        pub aio_acee: c_uint,
        pub aio_sicode: c_ushort,
        pub aio_rsvd2: [c_char; 14],
        pub aio_buf: *mut c_void, // volatile
        pub aio_exitptr: *mut c_void,
        pub aio_sigevent: crate::sigevent,
        pub aio_sockaddrptr: crate::sockaddr_in,
        pub aio_rsvd3: [c_char; 8],
        pub aio_locsockaddrptr: crate::sockaddr_in,
        pub aio_locsockaddrlen: c_int,
        pub aio_anrdes: c_int,
        pub aio_rsvd6: [c_char; 48],
    }

    pub struct ucontext_t {
        pub uc_mcontext: crate::mcontext_t,
        pub uc_stack: crate::stack_t,
        pub uc_sigmask: crate::sigset_t,
        pub uc_link: *mut ucontext_t,
    }

    pub struct utmpx {
        pub ut_user: [c_char; 9],
        pub ut_id: [c_char; 34],
        pub ut_line: [c_char; 33],
        pub ut_pid: crate::pid_t,
        pub ut_type: c_short,
        pub ut_version: c_short,
        pub ut_tv32: crate::__timeval32,
        pub ut_exit: crate::ut_exit_status,
        pub ut_reserved1: c_ushort,
        pub ut_host: [c_char; 1024],
        pub ut_tv_pad: [u8; 6],
        pub ut_tv: crate::timeval,
    }

    pub struct msqid_ds {
        pub msg_perm: crate::ipc_perm,
        pub msg_qnum: crate::msgqnum_t,
        pub msg_qbytes: crate::msglen_t,
        pub msg_lspid: crate::pid_t,
        pub msg_lrpid: crate::pid_t,
        pub msg_stime_31: [c_char; 4],
        pub msg_rtime_31: [c_char; 4],
        pub msg_ctime_31: [c_char; 4],
        pub msg_stime: crate::time_t,
        pub msg_rtime: crate::time_t,
        pub msg_ctime: crate::time_t,
    }
}

s_no_extra_traits! {
    pub struct siginfo_t {
        pub si_signo: c_int,
        pub si_errno: c_int,
        pub si_code: c_int,
        pub si_pid: crate::pid_t,
        pub si_uid: crate::uid_t,
        pub si_addr_31: [c_char; 4],
        pub si_status: c_int,
        pub si_band_value_31: [c_char; 8],
        pub si_resv: [c_char; 4],
        pub si_addr: *mut c_void,
        pub si_value: crate::sigval,
        pub si_band: c_long,
    }

    #[repr(C, packed)]
    pub union __aio_u1_packed {
        __aio_u1_sigevent_31: [c_char; 20],
        __aio_u1_msgevent_s: [c_char; 12],  // Size: 4+2+2+4 = 12 bytes (or use appropriate size)
    }

    pub struct file_tag {
        pub ft_ccsid: c_ushort,
        ft_flags: c_ushort, // FIXME(union): this field is actually a union
    }

    pub struct f_attributes {
        pub att_id: [c_char; 4usize],
        pub att_version: c_short,
        pub att_res01: [c_char; 2usize],
        pub att_bitfields1: [u8; 4usize], // ATT_SET_CONTROL_BIT() sets.
        pub att_mode: crate::mode_t,
        pub att_uid: c_int,
        pub att_gid: c_int,
        pub att_bitfields2: [u8; 8usize],
        pub att_size: off_t,
        pub att_atime_31: [c_char; 4usize],
        pub att_mtime_31: [c_char; 4usize],
        pub att_auditoraudit: c_int,
        pub att_useraudit: c_int,
        pub att_ctime_31: [c_char; 4usize],
        pub att_reftime_31: [c_char; 4usize],
        pub att_filefmt: c_char,
        pub att_res02: [c_char; 3usize],
        pub att_filetag: crate::file_tag,
        pub att_res03: [c_char; 8usize],
        pub att_atime: crate::time_t,
        pub att_mtime: crate::time_t,
        pub att_ctime: crate::time_t,
        pub att_reftime: crate::time_t,
        pub att_seclabel: [c_char; 8usize],
        pub att_res05: [c_char; 8usize],
    }
}

impl siginfo_t {
    pub unsafe fn si_addr(&self) -> *mut c_void {
        self.si_addr
    }

    pub unsafe fn si_value(&self) -> crate::sigval {
        self.si_value
    }

    pub unsafe fn si_pid(&self) -> crate::pid_t {
        self.si_pid
    }

    pub unsafe fn si_uid(&self) -> crate::uid_t {
        self.si_uid
    }

    pub unsafe fn si_status(&self) -> c_int {
        self.si_status
    }
}

impl shmid_ds {
    const SHM_SEG64_MASK: c_uint = 0x8000_0000;
    const SHM_DUMP_PRIO64_MASK: c_uint = 0x00ff_0000;

    pub fn shm_seg64(&self) -> bool {
        (self.shm_flags & Self::SHM_SEG64_MASK) != 0
    }

    pub fn set_shm_seg64(&mut self, value: bool) {
        if value {
            self.shm_flags |= Self::SHM_SEG64_MASK;
        } else {
            self.shm_flags &= !Self::SHM_SEG64_MASK;
        }
    }

    pub fn shm_dump_prio64(&self) -> c_uchar {
        ((self.shm_flags & Self::SHM_DUMP_PRIO64_MASK) >> 16) as c_uchar
    }

    pub fn set_shm_dump_prio64(&mut self, value: c_uchar) {
        self.shm_flags = (self.shm_flags & !Self::SHM_DUMP_PRIO64_MASK)
            | ((value as c_uint) << 16);
    }
}

impl file_tag {
    // On big-endian z/OS, C bit-fields are packed MSB-first within their
    // storage unit.  The C struct is:
    //   unsigned int ft_txtflag :1;   /* bit 15 (MSB) of ft_flags → 0x8000 */
    //   unsigned int ft_deferred:1;   /* bit 14           → 0x4000 */
    //   unsigned int ft_rsvflags:14;  /* bits 13..0       → 0x3FFF */
    const TXT_FLAG_MASK: u16 = 0x8000;
    const DEFERRED_MASK: u16 = 0x4000;
    const RSVFLAGS_MASK: u16 = 0x3FFF;

    pub fn new(ccsid: u16, txt_flag: bool, deferred: bool, rsv_flags: u16) -> Self {
        let mut flags = 0u16;
        if txt_flag { flags |= Self::TXT_FLAG_MASK; }
        if deferred { flags |= Self::DEFERRED_MASK; }
        flags |= rsv_flags & Self::RSVFLAGS_MASK;
        Self { ft_ccsid: ccsid, ft_flags: flags }
    }

    pub fn ft_txtflag(&self) -> bool {
        (self.ft_flags & Self::TXT_FLAG_MASK) != 0
    }

    pub fn set_ft_txtflag(&mut self, value: bool) {
        if value { self.ft_flags |= Self::TXT_FLAG_MASK; }
        else { self.ft_flags &= !Self::TXT_FLAG_MASK; }
    }

    pub fn ft_deferred(&self) -> bool {
        (self.ft_flags & Self::DEFERRED_MASK) != 0
    }

    pub fn set_ft_deferred(&mut self, value: bool) {
        if value { self.ft_flags |= Self::DEFERRED_MASK; }
        else { self.ft_flags &= !Self::DEFERRED_MASK; }
    }

    pub fn ft_rsvflags(&self) -> u16 {
        self.ft_flags & Self::RSVFLAGS_MASK
    }

    pub fn set_ft_rsvflags(&mut self, value: u16) {
        self.ft_flags = (self.ft_flags & !Self::RSVFLAGS_MASK) |
                        (value & Self::RSVFLAGS_MASK);
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for siginfo_t {
            fn eq(&self, other: &siginfo_t) -> bool {
                self.si_signo == other.si_signo
                    && self.si_errno == other.si_errno
                    && self.si_code == other.si_code
                    && self.si_pid == other.si_pid
                    && self.si_uid == other.si_uid
                    && self.si_addr_31 == other.si_addr_31
                    && self.si_status == other.si_status
                    && self.si_band_value_31 == other.si_band_value_31
                    && self.si_resv == other.si_resv
                    && self.si_addr == other.si_addr
                    && self.si_value == other.si_value
                    && self.si_band == other.si_band
            }
        }
        impl Eq for siginfo_t {}
        impl core::hash::Hash for siginfo_t {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.si_signo.hash(state);
                self.si_errno.hash(state);
                self.si_code.hash(state);
                self.si_pid.hash(state);
                self.si_uid.hash(state);
                self.si_addr_31.hash(state);
                self.si_status.hash(state);
                self.si_band_value_31.hash(state);
                self.si_resv.hash(state);
                self.si_addr.hash(state);
                self.si_value.hash(state);
                self.si_band.hash(state);
            }
        }

        impl PartialEq for __aio_u1_packed {
            fn eq(&self, other: &__aio_u1_packed) -> bool {
                unsafe {
                    self.__aio_u1_sigevent_31 == other.__aio_u1_sigevent_31
                }
            }
        }
        impl Eq for __aio_u1_packed {}
        impl core::hash::Hash for __aio_u1_packed {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                unsafe {
                    self.__aio_u1_sigevent_31.hash(state);
                }
            }
        }
    }
}

// Helper methods for statvfs
impl statvfs {
    /// Get f_OEmaxfilesize as off_t from the union field
    pub fn get_f_OEmaxfilesize(&self) -> off_t {
        unsafe { *(self.f_OEmaxfilesize_u.as_ptr() as *const off_t) }
    }

    /// Get the high and low words from f_OEmaxfilesize_s
    pub fn get_f_OEmaxfilesize_parts(&self) -> (c_int, c_int) {
        unsafe {
            let ptr = self.f_OEmaxfilesize_u.as_ptr() as *const c_int;
            (*ptr, *ptr.offset(1))
        }
    }
}

// pthread.h
pub const PTHREAD_STACK_MIN: size_t = 1048576;

pub const PTHREAD_MUTEX_INITIALIZER: super::pthread_mutex_t = super::pthread_mutex_t {
    __m: 0x00000000FBD7C9D6,
};

pub const PTHREAD_COND_INITIALIZER: super::pthread_cond_t = super::pthread_cond_t {
    __m: 0x00000000FBC3C9D6,
};

pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
    __: [0, 0, 0, 0, 0xFB, 0xD7, 0xC9, 0xD6],
};

// signal.h
pub const SIGSTKSZ: size_t = 2113536;
pub const MINSIGSTKSZ: size_t = 2105344;

// stropt.h
pub const I_PUSH: c_int = 0x4008e201;
pub const I_STR: c_int = 0xc018e20eu32 as c_int;
pub const I_PEEK: c_int = 0xc028e209u32 as c_int;
pub const I_FDINSERT: c_int = 0x4030e20d;
pub const I_LIST: c_int = 0xc010e213u32 as c_int;

// sys/resource.h
pub const RLIM_INFINITY: rlim_t = 2147483647;

// sys/stat.h
//
// Control settings for use with ATT_SET_CONTROL_BIT()
// to set a particular bit of the bitfields1 member of the
// f_attributes structure.
//
pub const ATT_CHANGE_TO_MODE: usize = 0;
pub const ATT_CHANGE_TO_OWNER: usize = 1;
pub const ATT_SET_GENERAL_ATTRUBYTES: usize = 2;
pub const ATT_TRUNCATE_SIZE: usize = 3;
pub const ATT_CHANGE_ATIME: usize = 4;
pub const ATT_CHANGE_ATIME_TOD: usize = 5;
pub const ATT_CHANGE_MTIME: usize = 6;
pub const ATT_CHANGE_MTIME_TOD: usize = 7;
pub const ATT_MODIFY_AUDITOR_AUDIT_INFO: usize = 8;
pub const ATT_MODIFY_USER_AUDIT_INFO: usize = 9;
pub const ATT_CHANGE_CTIME: usize = 10;
pub const ATT_CHANGE_CTIME_TOD: usize = 11;
pub const ATT_CHANGE_REFTIME: usize = 12;
pub const ATT_CHANGE_REFTIME_TOD: usize = 13;
pub const ATT_CHANGE_FILEFMT: usize = 14;

pub const ATT_CHANGE_FILETAG: usize = 17;
pub const ATT_USE_64BIT_TIME_VALUES: usize = 18;
pub const ATT_CHANGE_SECLABEL: usize = 19;
