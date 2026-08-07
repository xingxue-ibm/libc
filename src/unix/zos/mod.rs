use crate::prelude::*;

pub type caddr_t = *mut c_char;
pub type clockid_t = c_uint;
pub type blkcnt_t = c_long;
pub type clock_t = c_uint;
pub type dev_t = c_uint;
pub type fsblkcnt_t = c_ulong;
pub type fsfilcnt_t = c_uint;
pub type idtype_t = c_uint;
pub type ino_t = c_uint;
pub type key_t = c_int;
pub type mode_t = c_int;
pub type nlink_t = c_int;
pub type time_t = c_long;
pub type time64_t = c_longlong;
pub type nfds_t = c_uint;
pub type id_t = c_int;
pub type msgqnum_t = c_uint;
pub type msglen_t = c_uint;
pub type speed_t = c_uint;
pub type tcflag_t = c_uint;
pub type mtm_t = c_uint;
extern_ty! {
    pub type __iconv_rec;
}

pub type iconv_t = *mut __iconv_rec;

pub type __fd_mask = c_int;

pub type suseconds_t = c_int;
pub type useconds_t = c_uint;
pub type off_t = c_longlong;
pub type off64_t = c_longlong;

pub type socklen_t = c_uint;
pub type sa_family_t = c_uchar;

pub type pthread_once_t = c_int;
pub type blksize_t = c_int;
pub type nl_item = c_int;
pub type shmatt_t = c_uint;
pub type regoff_t = c_long;

c_enum! {
    #[repr(u32)]
    pub enum uio_rw {
        pub UIO_READ = 0,
        pub UIO_WRITE,
    }
}

s! {
    pub struct timezone {
        pub tz_minuteswest: c_int,
        pub tz_dsttime: c_int,
    }

    pub struct ip_mreq {
        pub imr_multiaddr: in_addr,
        pub imr_interface: in_addr,
    }

    pub struct termios {
        pub c_iflag: crate::tcflag_t,
        pub c_oflag: crate::tcflag_t,
        pub c_cflag: crate::tcflag_t,
        pub c_lflag: crate::tcflag_t,
        pub c_cc: [crate::cc_t; crate::NCCS],
    }

    pub struct lconv {
        pub decimal_point: *mut c_char,
        pub thousands_sep: *mut c_char,
        pub grouping: *mut c_char,
        pub int_curr_symbol: *mut c_char,
        pub currency_symbol: *mut c_char,
        pub mon_decimal_point: *mut c_char,
        pub mon_thousands_sep: *mut c_char,
        pub mon_grouping: *mut c_char,
        pub positive_sign: *mut c_char,
        pub negative_sign: *mut c_char,
        pub int_frac_digits: c_char,
        pub frac_digits: c_char,
        pub p_cs_precedes: c_char,
        pub p_sep_by_space: c_char,
        pub n_cs_precedes: c_char,
        pub n_sep_by_space: c_char,
        pub p_sign_posn: c_char,
        pub n_sign_posn: c_char,
        pub left_parenthesis: *mut c_char,
        pub right_parenthesis: *mut c_char,
        pub int_p_cs_precedes: c_char,
        pub int_p_sep_by_space: c_char,
        pub int_n_cs_precedes: c_char,
        pub int_n_sep_by_space: c_char,
        pub int_p_sign_posn: c_char,
        pub int_n_sign_posn: c_char,
    }

    pub struct tm {
        pub tm_sec: c_int,
        pub tm_min: c_int,
        pub tm_hour: c_int,
        pub tm_mday: c_int,
        pub tm_mon: c_int,
        pub tm_year: c_int,
        pub tm_wday: c_int,
        pub tm_yday: c_int,
        pub tm_isdst: c_int,
        pub tm_gmtoff: c_long,  // _XPLATFORM_SOURCE version
        pub tm_zone: *const c_char, // _XPLATFORM_SOURCE version
    }

    pub struct in_addr {
        pub s_addr: crate::in_addr_t,
    }

    pub struct ip_mreq_source {
        pub imr_multiaddr: in_addr,
        pub imr_sourceaddr: in_addr,
        pub imr_interface: in_addr,
    }

    pub struct sockaddr {
        pub sa_len: c_uchar,
        pub sa_family: sa_family_t,
        pub sa_data: [c_char; 14],
    }

    pub struct sockaddr_in {
        pub sin_len: c_uchar,
        pub sin_family: sa_family_t,
        pub sin_port: crate::in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [c_uchar; 8],
    }

    pub struct sockaddr_in6 {
        pub sin6_len: c_uchar,
        pub sin6_family: c_uchar,
        pub sin6_port: c_ushort,
        pub sin6_flowinfo: crate::uint32_t,
        pub sin6_addr: crate::in6_addr,
        pub sin6_scope_id: crate::uint32_t,
    }

    pub struct sockaddr_storage {
        pub ss_len: c_uchar,
        pub ss_family: sa_family_t,
        __ss_pad1: [c_char; 6],
        __ss_align: crate::int64_t,
        __ss_pad2: [c_char; 112],
    }

    pub struct sockaddr_un {
        pub sun_len: c_uchar,
        pub sun_family: sa_family_t,
        pub sun_path: [c_char; 108],
    }

    pub struct passwd {
        pub pw_name: *mut c_char,
        pub pw_uid: crate::uid_t,
        pub pw_gid: crate::gid_t,
        pub pw_dir: *mut c_char,
        pub pw_shell: *mut c_char,
    }

    pub struct utsname {
        pub sysname: [c_char; 16],
        pub nodename: [c_char; 32],
        pub release: [c_char; 8],
        pub version: [c_char; 8],
        pub machine: [c_char; 16],
    }

    pub struct cmsghdr {
        pub cmsg_len: crate::socklen_t,
        pub cmsg_level: c_int,
        pub cmsg_type: c_int,
    }

    pub struct sigevent {
        pub sigev_notify: c_int,
        pub sigev_signo: c_int,
        pub sigev_value: crate::sigval,
        pub sigev_notify_function: extern "C" fn(val: crate::sigval),
        pub sigev_notify_attributes: *mut pthread_attr_t,
    }

    pub struct sched_param {
        pub sched_priority: c_int,
    }

    pub struct stack_t {
        pub ss_sp: *mut c_void,
        pub ss_size: size_t,
        pub ss_flags: c_int,
    }

    pub struct glob_t {
        pub gl_pathc: size_t,
        pub gl_pathv: *mut *mut c_char,
        pub gl_offs: size_t,
        pub gl_padr: *mut c_void,
        pub gl_ptx: *mut c_void,
    }

    pub struct regmatch_t {
        pub rm_so: regoff_t,
        pub rm_eo: regoff_t,
    }

    pub struct regex_t {
        pub re_nsub: size_t,
        pub re_comp: *mut c_void,
        pub re_cflags: c_int,
        pub re_erroff: size_t,
        pub re_len: size_t,
        pub re_ucoll: [crate::wchar_t; 2],
        pub re_lsub: [*mut c_void; 24],
        pub re_esub: [*mut c_void; 24],
        pub re_map: *mut c_uchar,
        pub __maxsub: c_int,
        pub __unused: [*mut c_void; 34],
    }

    pub struct stat64 {
        pub st_eye: [c_char; 4],
        pub st_length: c_ushort,
        pub st_version: c_ushort,
        pub st_mode: mode_t,
        pub st_ino: ino_t,
        pub st_dev: dev_t,
        pub st_nlink: nlink_t,
        pub st_uid: crate::uid_t,
        pub st_gid: crate::gid_t,
        pub st_size: c_longlong,
        pub st_atime_31: [c_char; 4],
        pub st_mtime_31: [c_char; 4],
        pub st_ctime_31: [c_char; 4],
        pub st_rdev: dev_t,
        pub st_auditoraudit: c_uint,
        pub st_useraudit: c_uint,
        pub st_blksize: blksize_t,
        pub st_createtime_31: [c_char; 4],
        pub st_auditid: [c_char; 16],
        pub st_rsrvd1: [c_char; 4],
        pub st_charsetid: [c_char; 12],
        pub st_blocks: c_longlong,
        pub st_genvalue: c_uint,
        pub st_reftime_31: [c_char; 4],
        pub st_fid: [c_char; 8],
        pub st_filefmt: c_char,
        pub st_fspflag2: c_char,
        pub st_rsrvd2: [c_char; 2],
        pub st_ctimemsec: c_int,
        pub st_seclabel: [c_char; 8],
        pub st_rsrvd3: [c_char; 4],
        pub st_mptdev: dev_t,
        pub st_atime: time64_t,
        pub st_mtime: time64_t,
        pub st_ctime: time64_t,
        pub st_createtime: time64_t,
        pub st_reftime: time64_t,
        pub st_rsrvd5: [c_char; 24],
    }

    pub struct ipc_perm {
        pub uid: crate::uid_t,
        pub gid: crate::gid_t,
        pub cuid: crate::uid_t,
        pub cgid: crate::gid_t,
        pub mode: mode_t,
    }

    pub struct entry {
      pub key: *mut c_char,
      pub data: *mut c_void,
    }

    pub struct sembuf {
      pub sem_num: c_ushort,
      pub sem_op: c_short,
      pub sem_flg: c_short,
    }

    pub struct if_nameindex {
      pub if_index: c_uint,
      pub if_name: *mut c_char,
    }

    pub struct fd_set {
        pub fds_bits: [__fd_mask; 64],
    }

    pub struct statfs {
        pub f_type: c_ulong,
        pub f_bsize: c_ulong,
        pub f_blocks: crate::fsblkcnt_t,
        pub f_bfree: crate::fsblkcnt_t,
        pub f_bavail: crate::fsblkcnt_t,
        pub f_files: crate::fsfilcnt_t,
        pub f_ffree: crate::fsfilcnt_t,
        pub f_fsid: c_ulong,
        pub f_namelen: c_ulong,
        pub f_frsize: c_ulong,
        pub f_flags: c_ulong,
        pub f_spare: [c_ulong;4],
    }

    pub struct ut_exit_status {
        pub ut_e_termination: c_short,
        pub ut_e_exit: c_short,
    }

    pub struct __timeval32 {
        pub tv_sec: c_int,
        pub tv_usec: c_int,
    }

    pub struct sigaction {
        pub sa_handler: crate::sighandler_t,
        pub sa_mask: sigset_t,
        pub sa_flags: c_int,
        pub sa_sigaction: crate::sighandler_t,
    }

    pub struct sigset_t {
        pub __sigs_0: c_uint,
        pub __sigs_1: c_uint,
    }

    // <dirent.h>
    pub struct dirent {
        pub d_reclen: crate::c_ushort,
        pub d_namlen: crate::c_ushort,
        pub d_ino: crate::ino_t,
        pub d_extra: *mut c_void,
        pub d_name: [c_char; 256],
    }

    // <sys/types.h>
    #[repr(C, align(1))]
    pub struct pthread_t {
        __: c_ulonglong,
    }

    pub struct pthread_key_t {
        __private: [u8; 8],  // 8 bytes for s390x (64-bit)
    }
}

s_no_extra_traits! {
   // #[cfg(libc_union)]
    pub union pthread_cond_t {
        __m: c_ulong,
        __: [c_double; 8],
    }

   //  #[cfg(libc_union)]
    pub union pthread_mutex_t {
        __m: c_ulong,
        __d: [c_double; 8],
    }

    pub union epoll_data {
        pub ptr: *mut c_void,
        pub fd: c_int,
        pub u32: c_uint,
        pub u64: c_ulonglong,
    }

    // <sys/epoll.h>
    #[repr(C, packed)]
    pub struct epoll_event {
        pub events: u32,
        pub data: epoll_data,
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for epoll_data {
            fn eq(&self, _other: &epoll_data) -> bool {
                unimplemented!("traits")
            }
        }
        impl Eq for epoll_data {}
        impl hash::Hash for epoll_data {
            fn hash<H: hash::Hasher>(&self, _state: &mut H) {
                unimplemented!("traits")
            }
        }

        impl PartialEq for epoll_event {
            fn eq(&self, _other: &epoll_event) -> bool {
                unimplemented!("traits")
            }
        }
        impl Eq for epoll_event {}
        impl hash::Hash for epoll_event {
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                let events = self.events;
                let data = self.data;
                events.hash(state);
                data.hash(state);
            }
        }
    }
}

// dlfcn.h
pub const RTLD_LAZY: c_int = 0x4;
pub const RTLD_NOW: c_int = 0x2;
pub const RTLD_GLOBAL: c_int = 0x10000;
pub const RTLD_LOCAL: c_int = 0x80000;

// fcntl.h
pub const O_RDONLY: c_int = 0x2;
pub const O_WRONLY: c_int = 0x1;
pub const O_RDWR: c_int = 0x3;
pub const O_NDELAY: c_int = 0x4;
pub const O_APPEND: c_int = 0x8;
pub const O_CREAT: c_int = 0x80;
pub const O_EXCL: c_int = 0x40;
pub const O_NOCTTY: c_int = 0x20;
pub const O_TRUNC: c_int = 0x10;
pub const O_NOFOLLOW: c_int = 0x4000;
pub const O_DIRECTORY: c_int = 0x8000;
pub const O_CLOEXEC: c_int = 0x1000;
pub const O_ACCMODE: c_int = 0x3;
pub const O_DIRECT: c_int = 0x2000;
pub const O_LARGEFILE: c_int = 0x400;
pub const O_PATH: c_int = 0x00080000;
pub const F_GETLK: c_int = 5;
pub const F_SETLK: c_int = 6;
pub const F_SETLKW: c_int = 7;
pub const F_GETOWN: c_int = 10;
pub const F_SETOWN: c_int = 11;
pub const AT_FDCWD: c_int = -100;
pub const AT_SYMLINK_NOFOLLOW: c_int = 0x100;
pub const AT_EACCESS: c_int = 0x200;
pub const AT_REMOVEDIR: c_int = 0x200;
pub const AT_SYMLINK_FOLLOW: c_int = 0x400;
pub const AT_EMPTY_PATH: c_int = 0x1000;
pub const F_DUPFD: c_int = 0;
pub const F_GETFD: c_int = 1;
pub const F_SETFD: c_int = 2;
pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;
pub const O_SYNC: c_int = 256;
pub const O_NONBLOCK: c_int = 4;

// glob.h
pub const GLOB_APPEND: c_int = 0x1;
pub const GLOB_DOOFFS: c_int = 0x2;
pub const GLOB_ERR: c_int = 0x4;
pub const GLOB_MARK: c_int = 0x8;
pub const GLOB_NOCHECK: c_int = 0x10;
pub const GLOB_NOSORT: c_int = 0x20;
pub const GLOB_NOESCAPE: c_int = 0x80;
pub const GLOB_NOSPACE: c_int = 0x2000;
pub const GLOB_ABORTED: c_int = 0x1000;
pub const GLOB_NOMATCH: c_int = 0x4000;
pub const GLOB_NOSYS: c_int = 0x8000;

// langinfo.h
pub const DAY_1: crate::nl_item  = 15;
pub const DAY_2: crate::nl_item  = 16;
pub const DAY_3: crate::nl_item  = 17;
pub const DAY_4: crate::nl_item  = 18;
pub const DAY_5: crate::nl_item  = 19;
pub const DAY_6: crate::nl_item  = 20;
pub const DAY_7: crate::nl_item  = 21;
pub const ABDAY_1: crate::nl_item  = 8;
pub const ABDAY_2: crate::nl_item  = 9;
pub const ABDAY_3: crate::nl_item  = 10;
pub const ABDAY_4: crate::nl_item  = 11;
pub const ABDAY_5: crate::nl_item  = 12;
pub const ABDAY_6: crate::nl_item  = 13;
pub const ABDAY_7: crate::nl_item  = 14;
pub const MON_1: crate::nl_item  = 34;
pub const MON_2: crate::nl_item  = 35;
pub const MON_3: crate::nl_item  = 36;
pub const MON_4: crate::nl_item  = 37;
pub const MON_5: crate::nl_item  = 38;
pub const MON_6: crate::nl_item  = 39;
pub const MON_7: crate::nl_item  = 40;
pub const MON_8: crate::nl_item  = 41;
pub const MON_9: crate::nl_item  = 42;
pub const MON_10: crate::nl_item  = 43;
pub const MON_11: crate::nl_item  = 44;
pub const MON_12: crate::nl_item  = 45;
pub const ABMON_1: crate::nl_item  = 22;
pub const ABMON_2: crate::nl_item  = 23;
pub const ABMON_3: crate::nl_item  = 24;
pub const ABMON_4: crate::nl_item  = 25;
pub const ABMON_5: crate::nl_item  = 26;
pub const ABMON_6: crate::nl_item  = 27;
pub const ABMON_7: crate::nl_item  = 28;
pub const ABMON_8: crate::nl_item  = 29;
pub const ABMON_9: crate::nl_item  = 30;
pub const ABMON_10: crate::nl_item  = 31;
pub const ABMON_11: crate::nl_item  = 32;
pub const ABMON_12: crate::nl_item  = 33;
pub const RADIXCHAR: crate::nl_item  = 46;
pub const THOUSEP: crate::nl_item  = 47;
pub const YESSTR: crate::nl_item  = 56;
pub const NOSTR: crate::nl_item  = 57;
pub const CRNCYSTR: crate::nl_item  = 50;
pub const D_T_FMT: crate::nl_item  = 2;
pub const D_FMT: crate::nl_item  = 3;
pub const T_FMT: crate::nl_item  = 4;
pub const AM_STR: crate::nl_item  = 6;
pub const PM_STR: crate::nl_item  = 7;
pub const CODESET: crate::nl_item  = 1;
pub const T_FMT_AMPM: crate::nl_item  = 5;
pub const ERA: crate::nl_item  = 51;
pub const ERA_D_FMT: crate::nl_item  = 52;
pub const ERA_D_T_FMT: crate::nl_item  = 53;
pub const ERA_T_FMT: crate::nl_item  = 54;
pub const ALT_DIGITS: crate::nl_item  = 55;
pub const YESEXPR: crate::nl_item  = 48;
pub const NOEXPR: crate::nl_item  = 49;

// locale.h
pub const LC_CTYPE: c_int = 1;
pub const LC_NUMERIC: c_int = 3;
pub const LC_TIME: c_int = 4;
pub const LC_COLLATE: c_int = 0;
pub const LC_MONETARY: c_int = 2;
pub const LC_MESSAGES: c_int = 5;
pub const LC_ALL: c_int = -1;

// netdb.h
pub const NI_MAXHOST: crate::socklen_t  = 64;
pub const NI_NOFQDN: crate::socklen_t = 0x1;
pub const NI_NUMERICHOST: crate::socklen_t = 0x2;
pub const NI_NAMEREQD: crate::socklen_t = 0x4;
pub const NI_NUMERICSERV: crate::socklen_t = 0x8;
pub const NI_DGRAM: crate::socklen_t = 0x10;
pub const NI_NUMERICSCOPE: crate::socklen_t = 0x20;
pub const EAI_AGAIN: c_int = 2;
pub const EAI_BADFLAGS: c_int = 7;
pub const EAI_FAIL: c_int = 3;
pub const EAI_FAMILY: c_int = 5;
pub const EAI_MEMORY: c_int = 6;
pub const EAI_NONAME: c_int = 1;
pub const EAI_SERVICE: c_int = 8;
pub const EAI_SOCKTYPE: c_int = 9;
pub const EAI_SYSTEM: c_int = 10;
pub const EAI_OVERFLOW: c_int = 4;
pub const AI_CANONNAME: c_int = 0x2;
pub const AI_PASSIVE: c_int = 0x1;
pub const AI_NUMERICHOST: c_int = 0x4;
pub const AI_ADDRCONFIG: c_int = 0x40;
pub const AI_V4MAPPED: c_int = 0x10;
pub const AI_ALL: c_int = 0x20;
pub const AI_NUMERICSERV: c_int = 0x8;
pub const AI_EXTFLAGS: c_int = 0x80;
pub const IPV6_ADDR_PREFERENCES: c_int = 32;
pub const IPV6_CHECKSUM: c_int = 19;
pub const IPV6_DONTFRAG: c_int = 29;
pub const IPV6_DSTOPTS: c_int = 23;
pub const IPV6_HOPLIMIT: c_int = 11;
pub const IPV6_HOPOPTS: c_int = 22;
pub const IPV6_NEXTHOP: c_int = 20;
pub const IPV6_PATHMTU: c_int = 12;
pub const IPV6_PKTINFO: c_int = 13;
pub const IPV6_PREFER_SRC_CGA: c_int = 16;
pub const IPV6_PREFER_SRC_COA: c_int = 2;
pub const IPV6_PREFER_SRC_HOME: c_int = 1;
pub const IPV6_PREFER_SRC_NONCGA: c_int = 32;
pub const IPV6_PREFER_SRC_PUBLIC: c_int = 8;
pub const IPV6_PREFER_SRC_TMP: c_int = 4;
pub const IPV6_RECVDSTOPTS: c_int = 28;
pub const IPV6_RECVHOPLIMIT: c_int = 14;
pub const IPV6_RECVHOPOPTS: c_int = 26;
pub const IPV6_RECVPATHMTU: c_int = 16;
pub const IPV6_RECVRTHDR: c_int = 25;
pub const IPV6_RECVTCLASS: c_int = 31;
pub const IPV6_RTHDR: c_int = 21;
pub const IPV6_RTHDRDSTOPTS: c_int = 24;
pub const IPV6_TCLASS: c_int = 30;

// net/rtrouteh.h
pub const RTF_UP: c_int = 0x1;
pub const RTF_GATEWAY: c_int = 0x2;
pub const RTF_HOST: c_int = 0x4;
pub const RTF_DYNAMIC: c_int = 0x10;
pub const RTF_MODIFIED: c_int = 0x20;

// netinet/in.h
pub const IPPROTO_HOPOPTS: c_int = 0;
pub const IPPROTO_GGP: c_int = 2;
pub const IPPROTO_EGP: c_int = 8;
pub const IPPROTO_PUP: c_int = 12;
pub const IPPROTO_IDP: c_int = 22;
pub const IPPROTO_ROUTING: c_int = 43;
pub const IPPROTO_FRAGMENT: c_int = 44;
pub const IPPROTO_ESP: c_int = 50;
pub const IPPROTO_AH: c_int = 51;
pub const IPPROTO_NONE: c_int = 59;
pub const IPPROTO_DSTOPTS: c_int = 60;
pub const IPPROTO_RAW: c_int = 255;
pub const IPPROTO_MAX: c_int = 256;
pub const IP_OPTIONS: c_int = 1;
pub const IP_TOS: c_int = 2;
pub const IP_TTL: c_int = 14;
pub const IP_MULTICAST_IF: c_int = 7;
pub const IP_MULTICAST_TTL: c_int = 3;
pub const IP_MULTICAST_LOOP: c_int = 4;
pub const IP_ADD_MEMBERSHIP: c_int = 5;
pub const IP_DROP_MEMBERSHIP: c_int = 6;
pub const IP_BLOCK_SOURCE: c_int = 10;
pub const IP_UNBLOCK_SOURCE: c_int = 11;
pub const IP_ADD_SOURCE_MEMBERSHIP: c_int = 12;
pub const IP_DROP_SOURCE_MEMBERSHIP: c_int = 13;
pub const IP_DEFAULT_MULTICAST_TTL: c_int = 1;
pub const IP_DEFAULT_MULTICAST_LOOP: c_int = 1;
pub const IPV6_UNICAST_HOPS: c_int = 3;
pub const IPV6_MULTICAST_IF: c_int = 7;
pub const IPV6_MULTICAST_HOPS: c_int = 9;
pub const IPV6_MULTICAST_LOOP: c_int = 4;
pub const IPV6_RECVPKTINFO: c_int = 15;
pub const IPV6_V6ONLY: c_int = 10;
pub const IPV6_JOIN_GROUP: c_int = 5;
pub const IPV6_LEAVE_GROUP: c_int = 6;
pub const MCAST_BLOCK_SOURCE: c_int = 44;
pub const MCAST_EXCLUDE: c_int = 1;
pub const MCAST_INCLUDE: c_int = 0;
pub const MCAST_JOIN_GROUP: c_int = 40;
pub const MCAST_JOIN_SOURCE_GROUP: c_int = 42;
pub const MCAST_LEAVE_GROUP: c_int = 41;
pub const MCAST_LEAVE_SOURCE_GROUP: c_int = 43;
pub const MCAST_UNBLOCK_SOURCE: c_int = 45;

// netinet/tcp.h
pub const TCP_NODELAY: c_int = 0x1;

// xti.h
pub const TCP_MAXSEG: c_int = 0x2;
pub const TCP_KEEPALIVE: c_int = 0x8;

// pthread.h
pub const PTHREAD_CREATE_JOINABLE: c_int = 0;
pub const PTHREAD_CREATE_DETACHED: c_int = 1;
pub const PTHREAD_PROCESS_SHARED: c_int = 8;
pub const PTHREAD_PROCESS_PRIVATE: c_int = 0;
pub const PTHREAD_MUTEX_NORMAL: c_int = 4;
pub const PTHREAD_MUTEX_ERRORCHECK: c_int = 0;
pub const PTHREAD_MUTEX_RECURSIVE: c_int = 1;
pub const PTHREAD_MUTEX_DEFAULT: c_int = 0;
pub const PTHREAD_CANCEL_ENABLE: c_int = 0;
pub const PTHREAD_CANCEL_DISABLE: c_int = 1;
pub const PTHREAD_CANCEL_DEFERRED: c_int = 0;
pub const PTHREAD_CANCEL_ASYNCHRONOUS: c_int = 1;
pub const PTHREAD_EXPLICIT_SCHED: c_int = 0;
pub const PTHREAD_INHERIT_SCHED: c_int = 1;
pub const PTHREAD_ONCE_INIT: c_int = 0;
pub const PTHREAD_SCOPE_GLOBAL: c_int = 1;
pub const PTHREAD_SCOPE_LOCAL: c_int = 2;

// regex.h
pub const REG_EXTENDED: c_int = 1;
pub const REG_ICASE: c_int = 2;
pub const REG_NEWLINE: c_int = 4;
pub const REG_NOSUB: c_int = 8;
pub const REG_NOTBOL: c_int = 0x100;
pub const REG_NOTEOL: c_int = 0x200;
pub const REG_NOMATCH: c_int = 1;
pub const REG_BADPAT: c_int = 2;
pub const REG_ECOLLATE: c_int = 3;
pub const REG_ECTYPE: c_int = 4;
pub const REG_EESCAPE: c_int = 5;
pub const REG_ESUBREG: c_int = 6;
pub const REG_EBRACK: c_int = 7;
pub const REG_EPAREN: c_int = 8;
pub const REG_EBRACE: c_int = 9;
pub const REG_BADBR: c_int = 10;
pub const REG_ERANGE: c_int = 11;
pub const REG_ESPACE: c_int = 12;
pub const REG_BADRPT: c_int = 13;
pub const REG_ECHAR: c_int = 14;
pub const REG_EBOL: c_int = 15;
pub const REG_EEOL: c_int = 16;
pub const REG_ENOSYS: c_int = 17;

// search.h
pub const FIND: c_int = 0;
pub const ENTER: c_int = 1;

// stdio.h
pub const EOF: c_int = -1;
pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;
pub const _IOFBF: c_int = 1;
pub const _IONBF: c_int = 3;
pub const _IOLBF: c_int = 2;
pub const BUFSIZ: c_uint = 512;
pub const FOPEN_MAX: c_uint = 64;
pub const FILENAME_MAX: c_uint = 1024;
pub const L_tmpnam: c_uint = 1024;
pub const TMP_MAX: c_uint = 10000;

// stdlib.h
pub const EXIT_FAILURE: c_int = 8;
pub const EXIT_SUCCESS: c_int = 0;
pub const RAND_MAX: c_int = 32767;

// unistd.h
pub const F_OK: c_int = 0x8;
pub const R_OK: c_int = 0x4;
pub const W_OK: c_int = 0x2;
pub const X_OK: c_int = 0x1;

// aio.h
pub const LIO_NOP: c_int = 0;
pub const LIO_WAIT: c_int = 1;
pub const LIO_NOWAIT: c_int = 2;
pub const LIO_READ: c_int = 43;
pub const LIO_WRITE: c_int = 54;
pub const AIO_CANCELED: c_int = 1;
pub const AIO_NOTCANCELED: c_int = 2;
pub const AIO_ALLDONE: c_int = 3;

// errno.h
pub const EDOM: c_int = 1;
pub const ERANGE: c_int = 2;
pub const EOWNERDEAD: c_int = 104;
pub const ENOTRECOVERABLE: c_int = 105;
pub const EACCES: c_int = 111;
pub const EAGAIN: c_int = 112;
pub const EBADF: c_int = 113;
pub const EBUSY: c_int = 114;
pub const ECHILD: c_int = 115;
pub const EDEADLK: c_int = 116;
pub const EEXIST: c_int = 117;
pub const EFAULT: c_int = 118;
pub const EFBIG: c_int = 119;
pub const EINTR: c_int = 120;
pub const EINVAL: c_int = 121;
pub const EIO: c_int = 122;
pub const EISDIR: c_int = 123;
pub const EMFILE: c_int = 124;
pub const EMLINK: c_int = 125;
pub const ENAMETOOLONG: c_int = 126;
pub const ENFILE: c_int = 127;
pub const ENODEV: c_int = 128;
pub const ENOENT: c_int = 129;
pub const ENOEXEC: c_int = 130;
pub const ENOLCK: c_int = 131;
pub const ENOMEM: c_int = 132;
pub const ENOSPC: c_int = 133;
pub const ENOSYS: c_int = 134;
pub const ENOTDIR: c_int = 135;
pub const ENOTEMPTY: c_int = 136;
pub const ENOTTY: c_int = 137;
pub const ENXIO: c_int = 138;
pub const EPERM: c_int = 139;
pub const EPIPE: c_int = 140;
pub const EROFS: c_int = 141;
pub const ESPIPE: c_int = 142;
pub const ESRCH: c_int = 143;
pub const EXDEV: c_int = 144;
pub const E2BIG: c_int = 145;
pub const ELOOP: c_int = 146;
pub const EILSEQ: c_int = 147;
pub const ENODATA: c_int = 148;
pub const EOVERFLOW: c_int = 149;
pub const ENOTSUP: c_int = 247;
pub const ENOATTR: c_int = 265;
pub const ENOTBLK: c_int = 1100;
pub const ETXTBSY: c_int = 1101;
pub const EWOULDBLOCK: c_int = 1102;
pub const EINPROGRESS: c_int = 1103;
pub const EALREADY: c_int = 1104;
pub const ENOTSOCK: c_int = 1105;
pub const EDESTADDRREQ: c_int = 1106;
pub const EMSGSIZE: c_int = 1107;
pub const EPROTOTYPE: c_int = 1108;
pub const ENOPROTOOPT: c_int = 1109;
pub const EPROTONOSUPPORT: c_int = 1110;
pub const ESOCKTNOSUPPORT: c_int = 1111;
pub const EOPNOTSUPP: c_int = 1112;
pub const EPFNOSUPPORT: c_int = 1113;
pub const EAFNOSUPPORT: c_int = 1114;
pub const EADDRINUSE: c_int = 1115;
pub const EADDRNOTAVAIL: c_int = 1116;
pub const ENETDOWN: c_int = 1117;
pub const ENETUNREACH: c_int = 1118;
pub const ENETRESET: c_int = 1119;
pub const ECONNABORTED: c_int = 1120;
pub const ECONNRESET: c_int = 1121;
pub const ENOBUFS: c_int = 1122;
pub const EISCONN: c_int = 1123;
pub const ENOTCONN: c_int = 1124;
pub const ESHUTDOWN: c_int = 1125;
pub const ETOOMANYREFS: c_int = 1126;
pub const ETIMEDOUT: c_int = 1127;
pub const ECONNREFUSED: c_int = 1128;
pub const EHOSTDOWN: c_int = 1129;
pub const EHOSTUNREACH: c_int = 1130;
pub const EPROCLIM: c_int = 1131;
pub const EUSERS: c_int = 1132;
pub const EDQUOT: c_int = 1133;
pub const ESTALE: c_int = 1134;
pub const EREMOTE: c_int = 1135;
pub const ENOSTR: c_int = 1136;
pub const ETIME: c_int = 1137;
pub const ENOSR: c_int = 1138;
pub const ENOMSG: c_int = 1139;
pub const EBADMSG: c_int = 1140;
pub const EIDRM: c_int = 1141;
pub const ENONET: c_int = 1142;
pub const ENOLINK: c_int = 1144;
pub const EADV: c_int = 1145;
pub const ESRMNT: c_int = 1146;
pub const ECOMM: c_int = 1147;
pub const EPROTO: c_int = 1148;
pub const EMULTIHOP: c_int = 1149;
pub const EDOTDOT: c_int = 1150;
pub const EREMCHG: c_int = 1151;
pub const ECANCELED: c_int = 1152;
pub const EINTRNODATA: c_int = 1159;
pub const ENOREUSE: c_int = 1160;
pub const ENOMOVE: c_int = 1161;

// IBM TCP/IP stack errors (errno.h)
pub const EIBMBADCALL: c_int = 1000;
pub const EIBMBADPARM: c_int = 1001;
pub const EIBMSOCKOUTOFRANGE: c_int = 1002;
pub const EIBMSOCKINUSE: c_int = 1003;
pub const EIBMIUCVERR: c_int = 1004;
pub const EOFFLOADboxERROR: c_int = 1005;
pub const EOFFLOADboxRESTART: c_int = 1006;
pub const EOFFLOADboxDOWN: c_int = 1007;
pub const EIBMCONFLICT: c_int = 1008;
pub const EIBMCANCELLED: c_int = 1009;
pub const EIBMBADTCPNAME: c_int = 1011;

// z/OS MVS extension errors (errno.h)
pub const EMVSNOTUP: c_int = 150;
pub const EMVSDYNALC: c_int = 151;
pub const EMVSCVAF: c_int = 152;
pub const EMVSCATLG: c_int = 153;
pub const EMVSINITIAL: c_int = 156;
pub const EMVSERR: c_int = 157;
pub const EMVSPARM: c_int = 158;
pub const EMVSPFSFILE: c_int = 159;
pub const EMVSBADCHAR: c_int = 160;
pub const EMVSPFSPERM: c_int = 162;
pub const EMVSSAFEXTRERR: c_int = 163;
pub const EMVSSAF2ERR: c_int = 164;
pub const EMVSTODNOTSET: c_int = 165;
pub const EMVSPATHOPTS: c_int = 166;
pub const EMVSNORTL: c_int = 167;
pub const EMVSEXPIRE: c_int = 168;
pub const EMVSPASSWORD: c_int = 169;
pub const EMVSWLMERROR: c_int = 170;
pub const EMVSCPLERROR: c_int = 171;
pub const EMVSARMERROR: c_int = 172;

// sys/file.h
pub const LOCK_SH: c_int = 1;
pub const LOCK_EX: c_int = 2;
pub const LOCK_NB: c_int = 4;
pub const LOCK_UN: c_int = 8;

// fcntl.h
pub const F_RDLCK: c_short = 1;
pub const F_WRLCK: c_short = 2;
pub const F_UNLCK: c_short = 3;

// sys/ioctl.h
pub const _IOCPARM_MASK: c_int = 0x7f;
pub const _IOC_VOID: c_int = 0x20000000;
pub const _IOC_OUT: c_int = 0x40000000;
pub const _IOC_IN: c_int = 0x80000000u32 as c_int;
pub const _IOC_INOUT: c_int = _IOC_IN | _IOC_OUT;
pub const FIOCLEX: c_int = 536913665;
pub const FIONCLEX: c_int = 536913666;
pub const FIONREAD: c_int = 1074046847;
pub const FIONBIO: c_int = 2147788670u32 as c_int;
pub const FIOASYNC: c_int = 2147788669u32 as c_int;
pub const FIOSETOWN: c_int = 2147788668u32 as c_int;
pub const FIOGETOWN: c_int = 1074046843;
pub const TIOCGETD: c_int = 0x4004a700;
pub const TIOCSETD: c_int = 0x8004a701u32 as c_int;
pub const TIOCHPCL: c_int = 0x2000a702;
pub const TIOCMODG: c_int = 0x4004a703;
pub const TIOCMODS: c_int = 0x8004a704u32 as c_int;
pub const TIOCM_LE: c_int = 0x1;
pub const TIOCM_DTR: c_int = 0x2;
pub const TIOCM_RTS: c_int = 0x4;
pub const TIOCM_ST: c_int = 0x8;
pub const TIOCM_SR: c_int = 0x10;
pub const TIOCM_CTS: c_int = 0x20;
pub const TIOCM_CAR: c_int = 0x40;
pub const TIOCM_CD: c_int = 0x40;
pub const TIOCM_RNG: c_int = 0x80;
pub const TIOCM_RI: c_int = 0x80;
pub const TIOCM_DSR: c_int = 0x100;
pub const TIOCGETP: c_int = 0x4006a708;
pub const TIOCSETP: c_int = 0x8006a709u32 as c_int;
pub const TIOCSETN: c_int = 0x8006a70au32 as c_int;
pub const TIOCEXCL: c_int = 0x2000a70d;
pub const TIOCNXCL: c_int = 0x2000a70e;
pub const TIOCFLUSH: c_int = 0x8004a710u32 as c_int;
pub const TIOCSETC: c_int = 0x8006a711u32 as c_int;
pub const TIOCGETC: c_int = 0x4006a712;
pub const TANDEM: c_int = 0x1;
pub const CBREAK: c_int = 0x2;
pub const LCASE: c_int = 0x4;
pub const MDMBUF: c_int = 0x100000;
pub const XTABS: c_int = 0xc00;
pub const SIOCADDRT: c_int = 0x8030a70au32 as c_int;
pub const SIOCDELRT: c_int = 0x8030a70bu32 as c_int;
pub const TIOCUCNTL: c_int = 2147788646u32 as c_int;
pub const TIOCPKT: c_int = 2147788656u32 as c_int;
pub const TIOCPKT_DATA: c_int = 0;
pub const TIOCPKT_FLUSHREAD: c_int = 1;
pub const TIOCPKT_FLUSHWRITE: c_int = 2;
pub const TIOCPKT_NOSTOP: c_int = 16;
pub const TIOCPKT_DOSTOP: c_int = 32;
pub const TIOCPKT_START: c_int = 8;
pub const TIOCPKT_STOP: c_int = 4;

// sys/ipc.h
pub const IPC_CREAT: c_int = 0x1000000;
pub const IPC_EXCL: c_int = 0x2000000;
pub const IPC_NOWAIT: c_int = 1;
pub const IPC_RMID: c_int = 1;
pub const IPC_SET: c_int = 2;
pub const IPC_STAT: c_int = 3;
pub const IPC_PRIVATE: crate::key_t  = 0;

// limits.h
pub const PATH_MAX: c_int = 1024;
pub const PAGESIZE: c_int = 4096;
pub const IOV_MAX: c_int = 128;
pub const _POSIX_PIPE_BUF: usize = 512;
pub const _POSIX_OPEN_MAX: c_int = 20;
pub const _POSIX_MAX_INPUT: c_int = 255;
pub const _POSIX_MAX_CANON: c_int = 255;
pub const _POSIX_ARG_MAX: c_int = 4096;
pub const BC_BASE_MAX: c_int = 32767;
pub const BC_DIM_MAX: c_int = 32768;
pub const BC_SCALE_MAX: c_int = 32767;
pub const BC_STRING_MAX: c_int = 2048;
pub const CHARCLASS_NAME_MAX: c_int = 14;
pub const _POSIX_CHILD_MAX: c_int = 25;
pub const COLL_WEIGHTS_MAX: c_int = 4;
pub const EXPR_NEST_MAX: c_int = 32;
pub const NZERO: c_int = 20;

// unistd.h
pub const F_LOCK: c_int = 1;
pub const F_TEST: c_int = 3;
pub const F_TLOCK: c_int = 2;
pub const F_ULOCK: c_int = 0;

// sys/endian.h
pub const BIG_ENDIAN: c_int = 4321;
pub const LITTLE_ENDIAN: c_int = 1234;

// sys/mman.h
pub const PROT_NONE: c_int = 4;
pub const PROT_READ: c_int = 1;
pub const PROT_WRITE: c_int = 2;
pub const PROT_EXEC: c_int = 8;
pub const MAP_SHARED: c_int = 2;
pub const MAP_PRIVATE: c_int = 1;
pub const MAP_FIXED: c_int = 4;
pub const MAP_ANONYMOUS: c_int = 32;
pub const MAP_ANON: c_int = 32;
pub const MAP_FAILED: *mut c_void = !0 as *mut c_void;
pub const MS_SYNC: c_int = 0x1;
pub const MS_ASYNC: c_int = 0x2;
pub const MS_INVALIDATE: c_int = 0x4;

// sys/modes.h
pub const S_IFMT: mode_t = 0xff000000u32 as mode_t;
pub const S_IFMST: mode_t = 0x00FF0000;
pub const S_IFDIR: mode_t = 0x01000000;
pub const S_IFCHR: mode_t = 0x02000000;
pub const S_IFREG: mode_t = 0x03000000;
pub const S_IFIFO: mode_t = 0x04000000;
pub const S_IFFIFO: mode_t = 0x04000000; // canonical name in sys/modes.h; S_IFIFO is the XPG4 alias
pub const S_IFLNK: mode_t = 0x05000000;
pub const S_IFBLK: mode_t = 0x06000000;
pub const S_IFSOCK: mode_t = 0x07000000;
pub const S_IFVMEXTL: mode_t = 0xFE000000u32 as mode_t;
pub const S_IFVMEXTL_EXEC: mode_t = 0x00010000;
pub const S_IFVMEXTL_DATA: mode_t = 0x00020000;
pub const S_IFVMEXTL_MEL: mode_t = 0x00030000;
// st_genvalue mask bits (_OPEN_SYS)
pub const S_IFEXTL: mode_t = 0x00000001;
pub const S_IFPROGCTL: mode_t = 0x00000002;
pub const S_IFAPFCTL: mode_t = 0x00000004;
pub const S_IFNOSHARE: mode_t = 0x00000008;
pub const S_IFSHARELIB: mode_t = 0x00000010;
// owner/group/other permission bits
pub const S_IRWXU: mode_t = 0x01c0;
pub const S_IRUSR: mode_t = 0x0100;
pub const S_IWUSR: mode_t = 0x0080;
pub const S_IXUSR: mode_t = 0x0040;
pub const S_IRWXG: mode_t = 0x0038;
pub const S_IRGRP: mode_t = 0x0020;
pub const S_IWGRP: mode_t = 0x0010;
pub const S_IXGRP: mode_t = 0x0008;
pub const S_IRWXO: mode_t = 0x0007;
pub const S_IROTH: mode_t = 0x0004;
pub const S_IWOTH: mode_t = 0x0002;
pub const S_IXOTH: mode_t = 0x0001;
// _ALL_SOURCE aliases (sys/modes.h)
pub const S_IREAD: mode_t = S_IRUSR;
pub const S_IWRITE: mode_t = S_IWUSR;
pub const S_IEXEC: mode_t = S_IXUSR;

// sys/msg.h
pub const MSG_NOERROR: c_int = 4;

// resolv.h
pub const MAXHOSTNAMELEN: c_int = 64;

// limits.h
pub const NGROUPS_MAX: c_int = 300;

// poll.h
pub const POLLIN: c_short = 0x0003;
pub const POLLPRI: c_short = 0x0010;
pub const POLLOUT: c_short = 0x0004;
pub const POLLERR: c_short = 0x0020;
pub const POLLHUP: c_short = 0x0040;
pub const POLLNVAL: c_short = 0x0080;
pub const POLLRDNORM: c_short = 0x0001;
pub const POLLWRNORM: c_short = 0x0004;
pub const POLLRDBAND: c_short = 0x0002;
pub const POLLWRBAND: c_short = 0x0008;

// sys/resource.h
pub const RLIMIT_CPU: c_int = 0;
pub const RLIMIT_FSIZE: c_int = 1;
pub const RLIMIT_DATA: c_int = 2;
pub const RLIMIT_STACK: c_int = 3;
pub const RLIMIT_CORE: c_int = 4;
pub const RLIMIT_AS: c_int = 5;
pub const RLIMIT_NOFILE: c_int = 6;
pub const RLIMIT_MEMLIMIT: c_int = 7;
pub const RUSAGE_SELF: c_int = 0;
pub const RUSAGE_CHILDREN: c_int = -1;
pub const PRIO_PROCESS: c_int = 1;
pub const PRIO_PGRP: c_int = 2;
pub const PRIO_USER: c_int = 3;

// sched.h
pub const SCHED_OTHER: c_int = 1;
pub const SCHED_FIFO: c_int = 2;
pub const SCHED_RR: c_int = 3;

// sys/sem.h
pub const SEM_UNDO: c_int = 2;
pub const GETNCNT: c_int = 24;
pub const GETPID: c_int = 23;
pub const GETVAL: c_int = 21;
pub const GETALL: c_int = 26;
pub const GETZCNT: c_int = 25;
pub const SETVAL: c_int = 22;
pub const SETALL: c_int = 27;

// sys/shm.h
pub const SHMLBA: c_int = 4096;
pub const SHM_RDONLY: c_int = 1;
pub const SHM_RND: c_int = 2;

// signal.h
pub const SA_ONSTACK: c_int = 0x20000000;
pub const SA_RESETHAND: c_int = 0x10000000;
pub const SA_RESTART: c_int = 0x8000000;
pub const SA_SIGINFO: c_int = 0x4000000;
pub const SA_NODEFER: c_int = 0x1000000;
pub const SA_NOCLDWAIT: c_int = 0x2000000;
pub const SA_NOCLDSTOP: c_int = 0x80000000u32 as c_int;
pub const SS_ONSTACK: c_int = 0x2;
pub const SS_DISABLE: c_int = 0x1;
pub const SIGCHLD: c_int = 20;
pub const SIGBUS: c_int = 10;
pub const SIG_BLOCK: c_int = 0;
pub const SIG_UNBLOCK: c_int = 1;
pub const SIG_SETMASK: c_int = 2;
pub const SIGEV_NONE: c_int = 1;
pub const SIGEV_SIGNAL: c_int = 0;
pub const SIGHUP: c_int = 1;
pub const SIGINT: c_int = 2;
pub const SIGQUIT: c_int = 24;
pub const SIGILL: c_int = 4;
pub const SIGABRT: c_int = 3;
pub const SIGFPE: c_int = 8;
pub const SIGKILL: c_int = 9;
pub const SIGSEGV: c_int = 11;
pub const SIGSYS: c_int = 12;
pub const SIGPIPE: c_int = 13;
pub const SIGALRM: c_int = 14;
pub const SIGTERM: c_int = 15;
pub const SIGUSR1: c_int = 16;
pub const SIGUSR2: c_int = 17;
pub const SIGWINCH: c_int = 28;
pub const SIGURG: c_int = 6;
pub const SIGPOLL: c_int = 5;
pub const SIGIO: c_int = 23;
pub const SIGSTOP: c_int = 7;
pub const SIGTSTP: c_int = 25;
pub const SIGCONT: c_int = 19;
pub const SIGTTIN: c_int = 21;
pub const SIGTTOU: c_int = 22;
pub const SIGVTALRM: c_int = 31;
pub const SIGPROF: c_int = 32;
pub const SIGXCPU: c_int = 29;
pub const SIGXFSZ: c_int = 30;
pub const SIGTRAP: c_int = 26;
pub const SIGCLD: c_int = 20;
// z/OS-specific signals (signal.h)
pub const SIGABND: c_int = 18;
pub const SIGIOERR: c_int = 27;
pub const SIGDANGER: c_int = 33;
pub const SIGTHSTOP: c_int = 34;
pub const SIGTHCONT: c_int = 35;
pub const SIGTRACE: c_int = 37;
pub const SIGDCE: c_int = 38;
pub const SIGDUMP: c_int = 39;
pub const NSIG: c_int = 42;
pub const BUS_ADRALN: c_int = 71;
pub const BUS_ADRERR: c_int = 72;
pub const BUS_OBJERR: c_int = 73;
pub const CLD_EXITED: c_int = 101;
pub const CLD_KILLED: c_int = 102;
pub const CLD_DUMPED: c_int = 103;
pub const CLD_TRAPPED: c_int = 104;
pub const CLD_STOPPED: c_int = 105;
pub const CLD_CONTINUED: c_int = 106;
pub const FPE_INTDIV: c_int = 31;
pub const FPE_INTOVF: c_int = 32;
pub const FPE_FLTDIV: c_int = 33;
pub const FPE_FLTOVF: c_int = 34;
pub const FPE_FLTUND: c_int = 35;
pub const FPE_FLTRES: c_int = 36;
pub const FPE_FLTINV: c_int = 37;
pub const FPE_FLTSUB: c_int = 38;
pub const ILL_ILLOPC: c_int = 11;
pub const ILL_ILLOPN: c_int = 12;
pub const ILL_ILLADR: c_int = 13;
pub const ILL_ILLTRP: c_int = 14;
pub const ILL_PRVOPC: c_int = 15;
pub const ILL_PRVREG: c_int = 16;
pub const ILL_COPROC: c_int = 17;
pub const ILL_BADSTK: c_int = 18;
pub const POLL_IN: c_int = 111;
pub const POLL_OUT: c_int = 112;
pub const POLL_MSG: c_int = 113;
pub const POLL_ERR: c_int = 114;
pub const POLL_PRI: c_int = 115;
pub const POLL_HUP: c_int = 116;
pub const SEGV_MAPERR: c_int = 51;
pub const SEGV_ACCERR: c_int = 52;
pub const TRAP_BRKPT: c_int = 91;
pub const TRAP_TRACE: c_int = 92;
pub const SI_QUEUE: c_int = 176;
pub const SI_ASYNCIO: c_int = 175;

// sys/socket.h
pub const AF_UNSPEC: c_int = 0;
pub const AF_UNIX: c_int = 1;
pub const AF_INET: c_int = 2;
pub const AF_IMPLINK: c_int = 3;
pub const AF_PUP: c_int = 4;
pub const AF_CHAOS: c_int = 5;
pub const AF_NS: c_int = 6;
pub const AF_ECMA: c_int = 8;
pub const AF_DATAKIT: c_int = 9;
pub const AF_CCITT: c_int = 10;
pub const AF_SNA: c_int = 11;
pub const AF_DECnet: c_int = 12;
pub const AF_DLI: c_int = 13;
pub const AF_LAT: c_int = 14;
pub const SOMAXCONN: c_int = 10;
pub const AF_LOCAL: c_int = 1;
pub const AF_HYLINK: c_int = 15;
pub const AF_APPLETALK: c_int = 16;
pub const AF_NBS: c_int = 7;
pub const AF_ISO: c_int = AF_NBS; // alias on z/OS
pub const AF_IUCV: c_int = 17;
pub const AF_ROUTE: c_int = 20;
pub const AF_LINK: c_int = 18;
pub const AF_INET6: c_int = 19;
pub const AF_INTF: c_int = 20;
pub const AF_RIF: c_int = 21;
pub const AF_NDD: c_int = 23;
pub const AF_MAX: c_int = 30;
pub const PF_UNSPEC: c_int = AF_UNSPEC ;
pub const PF_UNIX: c_int = AF_UNIX ;
pub const PF_INET: c_int = AF_INET ;
pub const PF_IMPLINK: c_int = AF_IMPLINK ;
pub const PF_PUP: c_int = AF_PUP ;
pub const PF_CHAOS: c_int = AF_CHAOS ;
pub const PF_NS: c_int = AF_NS ;
pub const PF_ECMA: c_int = AF_ECMA ;
pub const PF_DATAKIT: c_int = AF_DATAKIT ;
pub const PF_CCITT: c_int = AF_CCITT ;
pub const PF_SNA: c_int = AF_SNA ;
pub const PF_DECnet: c_int = AF_DECnet ;
pub const PF_DLI: c_int = AF_DLI ;
pub const PF_LAT: c_int = AF_LAT ;
pub const PF_HYLINK: c_int = AF_HYLINK ;
pub const PF_APPLETALK: c_int = AF_APPLETALK ;
pub const PF_ROUTE: c_int = AF_ROUTE ;
pub const PF_MAX: c_int = AF_MAX ;
pub const SF_CLOSE: c_int = 2;
pub const SF_REUSE: c_int = 1;
pub const SOCK_DGRAM: c_int = 2;
pub const SOCK_STREAM: c_int = 1;
pub const SOCK_RAW: c_int = 3;
pub const SOCK_RDM: c_int = 4;
pub const SOCK_SEQPACKET: c_int = 5;
pub const SOL_SOCKET: c_int = 0xffff;
pub const SO_DEBUG: c_int = 0x0001;
pub const SO_ACCEPTCONN: c_int = 0x0002;
pub const SO_REUSEADDR: c_int = 0x0004;
pub const SO_KEEPALIVE: c_int = 0x0008;
pub const SO_DONTROUTE: c_int = 0x0010;
pub const SO_BROADCAST: c_int = 0x0020;
pub const SO_USELOOPBACK: c_int = 0x0040;
pub const SO_LINGER: c_int = 0x0080;
pub const SO_OOBINLINE: c_int = 0x0100;
pub const SO_REUSEPORT: c_int = 0x0200;
pub const SO_USE_IFBUFS: c_int = 0x0400;
pub const SO_CKSUMRECV: c_int = 0x0800;
pub const SO_NOREUSEADDR: c_int = 0x1000;
pub const SO_SNDBUF: c_int = 0x1001;
pub const SO_RCVBUF: c_int = 0x1002;
pub const SO_SNDLOWAT: c_int = 0x1003;
pub const SO_RCVLOWAT: c_int = 0x1004;
pub const SO_SNDTIMEO: c_int = 0x1005;
pub const SO_RCVTIMEO: c_int = 0x1006;
pub const SO_ERROR: c_int = 0x1007;
pub const SO_TYPE: c_int = 0x1008;
pub const SCM_RIGHTS: c_int = 0x01;
pub const MSG_OOB: c_int = 0x1;
pub const MSG_PEEK: c_int = 0x2;
pub const MSG_DONTROUTE: c_int = 0x4;
pub const MSG_EOR: c_int = 0x8;
pub const MSG_TRUNC: c_int = 0x10;
pub const MSG_CTRUNC: c_int = 0x20;
pub const MSG_WAITALL: c_int = 0x40;
pub const MSG_NONBLOCK: c_int = 0x4000;
pub const MSG_MAXIOVLEN: c_int = 16;
pub const SHUT_RD: c_int = 0;
pub const SHUT_WR: c_int = 1;
pub const SHUT_RDWR: c_int = 2;
pub const SOCK_CLOEXEC: c_int = 0x00001000;
pub const SOCK_NONBLOCK: c_int = 0x00000800;

// net/if.h
pub const IFF_UP: c_int = 0x1;
pub const IFF_BROADCAST: c_int = 0x2;
pub const IFF_DEBUG: c_int = 0x4;
pub const IFF_LOOPBACK: c_int = 0x8;
pub const IFF_POINTOPOINT: c_int = 0x10;
pub const IFF_NOTRAILERS: c_int = 0x20;
pub const IFF_RUNNING: c_int = 0x40;
pub const IFF_NOARP: c_int = 0x80;
pub const IFF_PROMISC: c_int = 0x100;
pub const IFF_ALLMULTI: c_int = 0x200;
pub const IFF_CHECKSUM: c_int = 0x0400;
pub const IFF_MULTICAST: c_int = 0x0400;
pub const IFF_POINTOMULTIPT: c_int = 0x0800;
pub const IFF_BRIDGE: c_int = 0x1000;
pub const IFF_SNAP: c_int = 0x2000;
pub const IFF_VIRTUAL: c_int = 0x4000;

// sys/stat.h
pub const UTIME_NOW: c_int = -1;
pub const UTIME_OMIT: c_int = -2;

// sys/statvfs.h
pub const ST_RDONLY: c_ulong = 0x00000001;
pub const ST_NOSUID: c_ulong = 0x00000002;

// stropts.h
pub const I_NREAD: c_int = 0x8004e20cu32 as c_int;
pub const I_POP: c_int = 0x2000e202;
pub const I_LOOK: c_int = 0x8041e203u32 as c_int;
pub const I_FLUSH: c_int = 0x4004e204;
pub const I_SRDOPT: c_int = 0x4004e20a;
pub const I_GRDOPT: c_int = 0x8004e20bu32 as c_int;
pub const I_SETSIG: c_int = 0x4004e206;
pub const I_GETSIG: c_int = 0x8004e207u32 as c_int;
pub const I_FIND: c_int = 0x4041e208;
pub const I_LINK: c_int = 0x4004e21a;
pub const I_UNLINK: c_int = 0x4004e21b;
pub const I_SENDFD: c_int = 0x4004e211;
pub const I_RECVFD: c_int = 0x800ce212u32 as c_int;
pub const I_SWROPT: c_int = 0x4004e20f;
pub const I_GWROPT: c_int = 0x8004e210u32 as c_int;
pub const I_PLINK: c_int = 0x4004e21c;
pub const I_PUNLINK: c_int = 0x4004e21d;
pub const I_FLUSHBAND: c_int = 0x4004e205;
pub const I_CKBAND: c_int = 0x4004e215;
pub const I_GETBAND: c_int = 0x8004e216u32 as c_int;
pub const I_ATMARK: c_int = 0x4004e214;
pub const I_SETCLTIME: c_int = 0x4004e218;
pub const I_GETCLTIME: c_int = 0x8004e219u32 as c_int;
pub const I_CANPUT: c_int = 0x4004e217;

// syslog.h
pub const LOG_CRON: c_int = 9 << 3;

// sys/time.h
pub const FD_SETSIZE: c_int = 2048;
pub const ITIMER_REAL: c_int = 0;
pub const ITIMER_VIRTUAL: c_int = 1;
pub const ITIMER_PROF: c_int = 2;

// time.h
pub const CLOCK_REALTIME: clockid_t = 0;
pub const CLOCK_MONOTONIC: clockid_t = 1;

// termios.h
pub const NCCS: usize = 11;
pub const TCSADRAIN: c_int = 1;
pub const TCSAFLUSH: c_int = 2;
pub const TCIFLUSH: c_int = 0;
pub const TCOFLUSH: c_int = 1;
pub const TCIOFLUSH: c_int = 2;
pub const TCOOFF: c_int = 0;
pub const TCOON: c_int = 1;
pub const TCIOFF: c_int = 2;
pub const TCION: c_int = 3;
pub const TCSANOW: c_int = 0;

// sys/ioctl.h
pub const TIOCGWINSZ: c_int = 0x4008a368;
pub const TIOCSWINSZ: c_int = 0x8008a367u32 as c_int;
pub const TIOCLBIS: c_int = 0x8004a77fu32 as c_int;
pub const TIOCLBIC: c_int = 0x8004a77eu32 as c_int;
pub const TIOCLSET: c_int = 0x8004a77du32 as c_int;
pub const TIOCLGET: c_int = 0x4004a77c;
pub const TIOCSBRK: c_int = 0x2000a77b;
pub const TIOCCBRK: c_int = 0x2000a77a;
pub const TIOCSDTR: c_int = 0x2000a779;
pub const TIOCCDTR: c_int = 0x2000a778;
pub const TIOCSLTC: c_int = 0x8006a775u32 as c_int;
pub const TIOCGLTC: c_int = 0x4006a774;
pub const TIOCOUTQ: c_int = 0x4004a773;
pub const TIOCNOTTY: c_int = 0x2000a771;
pub const TIOCSTOP: c_int = 0x2000a76f;
pub const TIOCSTART: c_int = 0x2000a76e;
pub const TIOCGPGRP: c_int = 0x4004a777;
pub const TIOCSPGRP: c_int = 0x8004a776u32 as c_int;
pub const TIOCSTI: c_int = 0x8001a772u32 as c_int;
pub const TIOCMSET: c_int = 0x8004a76du32 as c_int;
pub const TIOCMBIS: c_int = 0x8004a76cu32 as c_int;
pub const TIOCMBIC: c_int = 0x8004a76bu32 as c_int;
pub const TIOCMGET: c_int = 0x4004a76a;
pub const TIOCREMOTE: c_int = 0x8004a769u32 as c_int;

// sys/dbx_plugin.h
pub const MAXCOMLEN: c_int = 32;

// sys/epoll.h
pub const EPOLL_CLOEXEC: c_int = 0x00001000;
pub const EPOLL_CTL_ADD: c_int = 0;
pub const EPOLL_CTL_MOD: c_int = 1;
pub const EPOLL_CTL_DEL: c_int = 2;
pub const EPOLLRDNORM: c_int = 0x0001;
pub const EPOLLRDBAND: c_int = 0x0002;
pub const EPOLLIN: c_int = 0x0003;
pub const EPOLLOUT: c_int = 0x0004;
pub const EPOLLWRBAND: c_int = 0x0008;
pub const EPOLLPRI: c_int = 0x0010;
pub const EPOLLERR: c_int = 0x0020;
pub const EPOLLHUP: c_int = 0x0040;
pub const EPOLLEXCLUSIVE: c_int = 0x20000000;
pub const EPOLLONESHOT: c_int = 0x40000000;

// sys/eventfd.h
pub const EFD_SEMAPHORE: c_int = 0x00002000;
pub const EFD_CLOEXEC: c_int = 0x00001000;
pub const EFD_NONBLOCK: c_int = 0x00000004;

// sys/wait.h
pub const P_ALL: c_int = 2;
pub const P_PID: c_int = 0;
pub const P_PGID: c_int = 1;
pub const WNOHANG: c_int = 0x0001;
pub const WUNTRACED: c_int = 0x0002;
pub const WEXITED: c_int = 0x0008;
pub const WCONTINUED: c_int = 0x0004;
pub const WNOWAIT: c_int = 0x0020;
pub const WSTOPPED: c_int = 0x0010;

// termios.h
// pub const NCCS: usize = 11; // Declared previously.
pub const OLCUC: crate::tcflag_t = 2;
pub const CSIZE: crate::tcflag_t = 0x00000030;
pub const CS5: crate::tcflag_t = 0x00000000;
pub const CS6: crate::tcflag_t = 0x00000010;
pub const CS7: crate::tcflag_t = 0x00000020;
pub const CS8: crate::tcflag_t = 0x00000030;
pub const CSTOPB: crate::tcflag_t = 0x00000080;
pub const ECHO: crate::tcflag_t = 0x00000008;
pub const ECHOE: crate::tcflag_t = 0x00000002;
pub const ECHOK: crate::tcflag_t = 0x00000004;
pub const ECHONL: crate::tcflag_t = 0x00000001;
pub const IGNBRK: crate::tcflag_t = 0x00000004;
pub const BRKINT: crate::tcflag_t = 0x00000001;
pub const IGNPAR: crate::tcflag_t = 0x00000010;
pub const PARMRK: crate::tcflag_t = 0x00000400;
pub const INPCK: crate::tcflag_t = 0x00000040;
pub const ISTRIP: crate::tcflag_t = 0x00000080;
pub const INLCR: crate::tcflag_t = 0x00000020;
pub const IGNCR: crate::tcflag_t = 0x00000008;
pub const ICRNL: crate::tcflag_t = 0x00000002;
pub const IXON: crate::tcflag_t = 0x00000200;
pub const IXOFF: crate::tcflag_t = 0x00000100;
pub const IXANY: crate::tcflag_t = 0x00001000;
pub const OPOST: crate::tcflag_t = 0x00000001;
pub const ONLCR: crate::tcflag_t = 0x00000004;
pub const OCRNL: crate::tcflag_t = 0x00000008;
pub const ONOCR: crate::tcflag_t = 0x00000010;
pub const ONLRET: crate::tcflag_t = 0x00000020;
pub const CREAD: crate::tcflag_t = 0x00000002;
pub const IEXTEN: crate::tcflag_t = 0x00000020;
pub const TOSTOP: crate::tcflag_t = 0x00400000;
pub const NOFLSH: crate::tcflag_t = 0x80000000;
pub const VINTR: usize = 0;
pub const VQUIT: usize = 1;
pub const VERASE: usize = 2;
pub const VKILL: usize = 3;
pub const VEOF: usize = 4;
pub const VEOL: usize = 5;
pub const VSTART: usize = 7;
pub const VSTOP: usize = 8;
pub const VSUSP: usize = 9;
pub const VMIN: usize = 6;
pub const VTIME: usize = 10;
pub const B0: crate::speed_t = 0x0;
pub const B50: crate::speed_t = 0x1;
pub const B75: crate::speed_t = 0x2;
pub const B110: crate::speed_t = 0x3;
pub const B134: crate::speed_t = 0x4;
pub const B150: crate::speed_t = 0x5;
pub const B200: crate::speed_t = 0x6;
pub const B300: crate::speed_t = 0x7;
pub const B600: crate::speed_t = 0x8;
pub const B1200: crate::speed_t = 0x9;
pub const B1800: crate::speed_t = 0xa;
pub const B2400: crate::speed_t = 0xb;
pub const B4800: crate::speed_t = 0xc;
pub const B9600: crate::speed_t = 0xd;
pub const B19200: crate::speed_t = 0xe;
pub const B38400: crate::speed_t = 0xf;
pub const IUCLC: crate::tcflag_t = 0x00000800;
pub const OFILL: crate::tcflag_t = 0x00000040;
pub const OFDEL: crate::tcflag_t = 0x00000080;
pub const CRDLY: crate::tcflag_t = 0x00003000;
pub const CR0: crate::tcflag_t = 0x00000000;
pub const CR1: crate::tcflag_t = 0x00001000;
pub const CR2: crate::tcflag_t = 0x00002000;
pub const CR3: crate::tcflag_t = 0x00003000;
pub const TABDLY: crate::tcflag_t = 0x00000c00;
pub const TAB0: crate::tcflag_t = 0x00000000;
pub const TAB1: crate::tcflag_t = 0x00000400;
pub const TAB2: crate::tcflag_t = 0x00000800;
pub const TAB3: crate::tcflag_t = 0x00000c00;
pub const BSDLY: crate::tcflag_t = 0x00008000;
pub const BS0: crate::tcflag_t = 0x00000000;
pub const BS1: crate::tcflag_t = 0x00008000;
pub const FFDLY: crate::tcflag_t = 0x00004000;
pub const FF0: crate::tcflag_t = 0x00000000;
pub const FF1: crate::tcflag_t = 0x00004000;
pub const NLDLY: crate::tcflag_t = 0x00000100;
pub const NL0: crate::tcflag_t = 0x00000000;
pub const NL1: crate::tcflag_t = 0x00000100;
pub const VTDLY: crate::tcflag_t = 0x00010000;
pub const VT0: crate::tcflag_t = 0x00000000;
pub const VT1: crate::tcflag_t = 0x00010000;
pub const PARENB: crate::tcflag_t = 0x00000200;
pub const PARODD: crate::tcflag_t = 0x00000400;
pub const HUPCL: crate::tcflag_t = 0x00000100;
pub const CLOCAL: crate::tcflag_t = 0x00000001;
pub const ISIG: crate::tcflag_t = 0x00000040;
pub const ICANON: crate::tcflag_t = 0x00000010;
pub const XCASE: crate::tcflag_t = 0x00000080;

// sys/ioctl.h
pub const FLUSHO: crate::tcflag_t = 0x00800000;
pub const PENDIN: crate::tcflag_t = 0x20000000;

// unistd.h
pub const _PC_LINK_MAX: c_int = 2;
pub const _PC_MAX_CANON: c_int = 3;
pub const _PC_MAX_INPUT: c_int = 4;
pub const _PC_NAME_MAX: c_int = 5;
pub const _PC_PATH_MAX: c_int = 7;
pub const _PC_PIPE_BUF: c_int = 8;
pub const _PC_NO_TRUNC: c_int = 6;
pub const _PC_VDISABLE: c_int = 9;
pub const _PC_CHOWN_RESTRICTED: c_int = 1;
pub const _SC_ARG_MAX: c_int = 1;
pub const _SC_CHILD_MAX: c_int = 2;
pub const _SC_CLK_TCK: c_int = 3;
pub const _SC_NGROUPS_MAX: c_int = 5;
pub const _SC_OPEN_MAX: c_int = 6;
pub const _SC_JOB_CONTROL: c_int = 4;
pub const _SC_SAVED_IDS: c_int = 7;
pub const _SC_VERSION: c_int = 10;
pub const _SC_PASS_MAX: c_int = 132;
pub const _SC_PAGESIZE: c_int = 135;
pub const _SC_PAGE_SIZE: c_int = 136;
pub const _SC_XOPEN_VERSION: c_int = 130;
pub const _SC_STREAM_MAX: c_int = 118;
pub const _SC_TZNAME_MAX: c_int = 9;
pub const _SC_AIO_LISTIO_MAX: c_int = 140;
pub const _SC_AIO_MAX: c_int = 141;
pub const _SC_AIO_PRIO_DELTA_MAX: c_int = 142;
pub const _SC_ASYNCHRONOUS_IO: c_int = 150;
pub const _SC_DELAYTIMER_MAX: c_int = 143;
pub const _SC_FSYNC: c_int = 153;
pub const _SC_MAPPED_FILES: c_int = 155;
pub const _SC_MEMLOCK: c_int = 156;
pub const _SC_MEMLOCK_RANGE: c_int = 157;
pub const _SC_MEMORY_PROTECTION: c_int = 158;
pub const _SC_MESSAGE_PASSING: c_int = 159;
pub const _SC_MQ_OPEN_MAX: c_int = 146;
pub const _SC_MQ_PRIO_MAX: c_int = 147;
pub const _SC_PRIORITIZED_IO: c_int = 161;
pub const _SC_PRIORITY_SCHEDULING: c_int = 162;
pub const _SC_REALTIME_SIGNALS: c_int = 165;
pub const _SC_RTSIG_MAX: c_int = 209;
pub const _SC_SEMAPHORES: c_int = 167;
pub const _SC_SEM_NSEMS_MAX: c_int = 210;
pub const _SC_SEM_VALUE_MAX: c_int = 211;
pub const _SC_SHARED_MEMORY_OBJECTS: c_int = 168;
pub const _SC_SIGQUEUE_MAX: c_int = 212;
pub const _SC_SYNCHRONIZED_IO: c_int = 174;
pub const _SC_TIMERS: c_int = 186;
pub const _SC_TIMER_MAX: c_int = 214;
pub const _SC_2_C_BIND: c_int = 115;
pub const _SC_2_C_DEV: c_int = 111;
pub const _SC_2_C_VERSION: c_int = 116;
pub const _SC_2_FORT_DEV: c_int = 112;
pub const _SC_2_FORT_RUN: c_int = 110;
pub const _SC_2_LOCALEDEF: c_int = 114;
pub const _SC_2_SW_DEV: c_int = 113;
pub const _SC_2_UPE: c_int = 117;
pub const _SC_2_VERSION: c_int = 109;
pub const _SC_BC_BASE_MAX: c_int = 101;
pub const _SC_BC_DIM_MAX: c_int = 102;
pub const _SC_BC_SCALE_MAX: c_int = 103;
pub const _SC_BC_STRING_MAX: c_int = 104;
pub const _SC_COLL_WEIGHTS_MAX: c_int = 105;
pub const _SC_EXPR_NEST_MAX: c_int = 106;
pub const _SC_LINE_MAX: c_int = 107;
pub const _SC_RE_DUP_MAX: c_int = 108;
pub const _SC_XOPEN_CRYPT: c_int = 127;
pub const _SC_XOPEN_ENH_I18N: c_int = 128;
pub const _SC_XOPEN_SHM: c_int = 129;
pub const _SC_2_CHAR_TERM: c_int = 12;
pub const _SC_XOPEN_XCU_VERSION: c_int = 131;
pub const _SC_ATEXIT_MAX: c_int = 133;
pub const _SC_IOV_MAX: c_int = 134;
pub const _SC_XOPEN_UNIX: c_int = 137;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: c_int = 206;
pub const _SC_GETGR_R_SIZE_MAX: c_int = 139;
pub const _SC_GETPW_R_SIZE_MAX: c_int = 138;
pub const _SC_LOGIN_NAME_MAX: c_int = 145;
pub const _SC_THREAD_KEYS_MAX: c_int = 207;
pub const _SC_THREAD_STACK_MIN: c_int = 208;
pub const _SC_THREAD_THREADS_MAX: c_int = 13;
pub const _SC_TTY_NAME_MAX: c_int = 215;
pub const _SC_THREADS: c_int = 184;
pub const _SC_THREAD_ATTR_STACKADDR: c_int = 175;
pub const _SC_THREAD_ATTR_STACKSIZE: c_int = 176;
pub const _SC_THREAD_PRIORITY_SCHEDULING: c_int = 180;
pub const _SC_THREAD_PRIO_INHERIT: c_int = 178;
pub const _SC_THREAD_PRIO_PROTECT: c_int = 179;
pub const _SC_THREAD_PROCESS_SHARED: c_int = 181;
pub const _SC_THREAD_SAFE_FUNCTIONS: c_int = 182;
pub const _SC_XOPEN_LEGACY: c_int = 220;
pub const _SC_XOPEN_REALTIME: c_int = 221;
pub const _SC_XOPEN_REALTIME_THREADS: c_int = 222;
pub const _SC_XBS5_ILP32_OFF32: c_int = 216;
pub const _SC_XBS5_ILP32_OFFBIG: c_int = 217;
pub const _SC_XBS5_LP64_OFF64: c_int = 218;
pub const _SC_XBS5_LPBIG_OFFBIG: c_int = 219;
pub const _SC_2_PBS: c_int = 200;
pub const _SC_2_PBS_ACCOUNTING: c_int = 201;
pub const _SC_2_PBS_CHECKPOINT: c_int = 202;
pub const _SC_2_PBS_LOCATE: c_int = 203;
pub const _SC_2_PBS_MESSAGE: c_int = 204;
pub const _SC_2_PBS_TRACK: c_int = 205;
pub const _SC_ADVISORY_INFO: c_int = 148;
pub const _SC_BARRIERS: c_int = 149;
pub const _SC_CLOCK_SELECTION: c_int = 151;
pub const _SC_CPUTIME: c_int = 152;
pub const _SC_HOST_NAME_MAX: c_int = 144;
pub const _SC_MONOTONIC_CLOCK: c_int = 160;
pub const _SC_READER_WRITER_LOCKS: c_int = 164;
pub const _SC_REGEXP: c_int = 166;
pub const _SC_SHELL: c_int = 169;
pub const _SC_SPAWN: c_int = 170;
pub const _SC_SPIN_LOCKS: c_int = 171;
pub const _SC_SPORADIC_SERVER: c_int = 172;
pub const _SC_SS_REPL_MAX: c_int = 173;
pub const _SC_SYMLOOP_MAX: c_int = 213;
pub const _SC_THREAD_CPUTIME: c_int = 177;
pub const _SC_THREAD_SPORADIC_SERVER: c_int = 183;
pub const _SC_TIMEOUTS: c_int = 185;
pub const _SC_TRACE: c_int = 187;
pub const _SC_TRACE_EVENT_FILTER: c_int = 188;
pub const _SC_TRACE_EVENT_NAME_MAX: c_int = 189;
pub const _SC_TRACE_INHERIT: c_int = 190;
pub const _SC_TRACE_LOG: c_int = 191;
pub const _SC_TRACE_NAME_MAX: c_int = 192;
pub const _SC_TRACE_SYS_MAX: c_int = 193;
pub const _SC_TRACE_USER_EVENT_MAX: c_int = 194;
pub const _SC_TYPED_MEMORY_OBJECTS: c_int = 195;
pub const _SC_V6_ILP32_OFF32: c_int = 196;
pub const _SC_V6_ILP32_OFFBIG: c_int = 197;
pub const _SC_V6_LP64_OFF64: c_int = 198;
pub const _SC_V6_LPBIG_OFFBIG: c_int = 199;
pub const _SC_XOPEN_STREAMS: c_int = 223;
pub const _SC_IPV6: c_int = 154;
pub const _SC_RAW_SOCKETS: c_int = 163;

// utmpx.h
pub const EMPTY: c_short = 0;
pub const __RUN_LVL: c_short = 1;
pub const BOOT_TIME: c_short = 2;
pub const OLD_TIME: c_short = 3;
pub const NEW_TIME: c_short = 4;
pub const INIT_PROCESS: c_short = 5;
pub const LOGIN_PROCESS: c_short = 6;
pub const USER_PROCESS: c_short = 7;
pub const DEAD_PROCESS: c_short = 8;
pub const __ACCOUNTING: c_short = 9;

// Mirrors z/OS sys/socket.h _CMSG_ALIGN: rounds up to sizeof(int)=4, not pointer size.
const fn CMSG_ALIGN(len: usize) -> usize {
    (len + size_of::<c_int>() - 1) & !(size_of::<c_int>() - 1)
}

f! {
    pub fn CMSG_FIRSTHDR(mhdr: *const msghdr) -> *mut cmsghdr {
        if (*mhdr).msg_controllen > 0 {
            (*mhdr).msg_control as *mut cmsghdr
        } else {
            0 as *mut cmsghdr
        }
    }

    pub fn CMSG_NXTHDR(mhdr: *const msghdr, cmsg: *const cmsghdr) -> *mut cmsghdr {
        if (cmsg as c_ulong + (*cmsg).cmsg_len as c_ulong)
            >= ((*mhdr).msg_control as c_ulong + (*mhdr).msg_controllen as c_ulong)
        {
            0 as *mut cmsghdr
        } else {
            (cmsg as c_ulong + (*cmsg).cmsg_len as c_ulong) as *mut cmsghdr
        }
    }

    pub fn CMSG_DATA(cmsg: *const cmsghdr) -> *mut c_uchar {
        (cmsg as *mut c_uchar).wrapping_add(size_of::<cmsghdr>())
    }

    pub const fn CMSG_LEN(length: c_uint) -> c_uint {
        // _CMSG_ALIGN(sizeof(cmsghdr)) + length
        (CMSG_ALIGN(size_of::<cmsghdr>()) + length as usize) as c_uint
    }

    pub const fn CMSG_SPACE(length: c_uint) -> c_uint {
        // _CMSG_ALIGN(sizeof(cmsghdr)) + _CMSG_ALIGN(length)
        (CMSG_ALIGN(size_of::<cmsghdr>()) + CMSG_ALIGN(length as usize)) as c_uint
    }

    pub fn FD_ZERO(set: *mut fd_set) -> () {
        for slot in (*set).fds_bits.iter_mut() {
            *slot = 0;
        }
    }

    pub fn FD_SET(fd: c_int, set: *mut fd_set) -> () {
        // __fd_mask = c_int = 32 bits on z/OS _LP64
        let bits = size_of::<__fd_mask>() * 8;
        let fd = fd as usize;
        (*set).fds_bits[fd / bits] |= 1 << (fd % bits);
        return
    }

    pub fn FD_CLR(fd: c_int, set: *mut fd_set) -> () {
        // __fd_mask = c_int = 32 bits on z/OS _LP64
        let bits = size_of::<__fd_mask>() * 8;
        let fd = fd as usize;
        (*set).fds_bits[fd / bits] &= !(1 << (fd % bits));
        return
    }

    pub fn FD_ISSET(fd: c_int, set: *const fd_set) -> bool {
        // __fd_mask = c_int = 32 bits on z/OS _LP64
        let bits = size_of::<__fd_mask>() * 8;
        let fd = fd as usize;
        return ((*set).fds_bits[fd / bits] & (1 << (fd % bits))) != 0
    }
}

safe_f! {
    pub const fn WIFSTOPPED(stat_val: c_int) -> bool {
        ((stat_val & 0x000000FF) == 0x0000007F) ||
        ((stat_val & 0x000000FF) == 0x0000007E) ||
        ((stat_val & 0x000000FF) == 0x0000007D) ||
        ((stat_val & 0x000000FF) == 0x0000007B) ||
        ((stat_val & 0x000000FF) == 0x0000007A) ||
        ((stat_val & 0x000000FF) == 0x00000078) ||
        ((stat_val & 0x000000FF) == 0x00000077)
    }

    pub const fn WSTOPSIG(stat_val: c_int) -> c_int {
        (stat_val & 0x0000FF00) >> 8
    }

    pub const fn WIFEXITED(stat_val: c_int) -> bool {
        if (stat_val & 0x000000FF) != 0 {
            false
        } else {
            true
        }
    }

    pub const fn WEXITSTATUS(stat_val: c_int) -> c_int {
        if WIFEXITED(stat_val) {
            (((stat_val) & 0x0000FF00) >> 8) as c_int
        } else {
            0
        }
    }

    pub const fn WIFSIGNALED(stat_val: c_int) -> bool {
        ((!(WIFSTOPPED(stat_val))) && ((stat_val & 0x000000FF) != 0)) &&
        !(((stat_val) & 0x000000FF) == 0x00000079)
    }

    pub const fn WTERMSIG(stat_val: c_int) -> c_int {
        stat_val & 0x0000007F
    }

    pub const fn WIFCONTINUED(stat_val: c_int) -> bool {
        (stat_val & 0x000000FF) == 0x00000079
    }

    // z/OS doesn't have native WCOREDUMP.
    pub const fn WCOREDUMP(_status: c_int) -> bool {
        false
    }

    // For use with __fchattr() to set att_bitfield1 member.
    pub fn ATT_SET_CONTROL_BIT(attributes: &mut crate::f_attributes, control_bit_number: usize) -> () {
        // control_bit_number is one of ATT_x settings.
        let control_byte_num = control_bit_number / 8;
        let control_byte_bit_num = control_bit_number - (control_byte_num*8);
        let mut control_byte = attributes.att_bitfields1[control_byte_num];
        control_byte |= 128 >> control_byte_bit_num;
        attributes.att_bitfields1[control_byte_num] = control_byte;
        return
    }
}

extern "C" {
    #[link_name = "@@PT@AF"]
    pub fn pthread_atfork(
        prepare: Option<unsafe extern "C" fn()>,
        parent: Option<unsafe extern "C" fn()>,
        child: Option<unsafe extern "C" fn()>,
    ) -> c_int;
    #[link_name = "@@PT@AGG"]
    pub fn pthread_attr_getguardsize(
        attr: *const crate::pthread_attr_t,
        guardsize: *mut size_t,
    ) -> c_int;
    #[link_name = "@@PT@ASG"]
    pub fn pthread_attr_setguardsize(attr: *mut crate::pthread_attr_t, guardsize: size_t) -> c_int;
    #[link_name = "@@PT@GSP"]
    pub fn pthread_attr_getschedparam(
        attr: *const crate::pthread_attr_t,
        param: *mut sched_param,
    ) -> c_int;
    #[link_name = "@@PT@AGK"]
    pub fn pthread_attr_getstack(
        attr: *const crate::pthread_attr_t,
        stackaddr: *mut *mut c_void,
        stacksize: *mut size_t,
    ) -> c_int;
    #[link_name = "@@PT@SSP"]
    pub fn pthread_attr_setschedparam(
        attr: *mut crate::pthread_attr_t,
        param: *const sched_param,
    ) -> c_int;
    #[link_name = "@@PT@SM"]
    pub fn pthread_sigmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int;
    #[link_name = "@@PT3CAN"]
    pub fn pthread_cancel(thread: crate::pthread_t) -> c_int;
    #[link_name = "@@PT3CGP"]
    pub fn pthread_condattr_getpshared(
        attr: *const pthread_condattr_t,
        pshared: *mut c_int,
    ) -> c_int;
    #[link_name = "@@PCSTCL"]
    pub fn pthread_condattr_setclock(
        attr: *mut pthread_condattr_t,
        clock_id: clockid_t,
    ) -> c_int;
    #[link_name = "@@PT3CSP"]
    pub fn pthread_condattr_setpshared(attr: *mut pthread_condattr_t, pshared: c_int) -> c_int;
    #[link_name = "@@PT3C"]
    pub fn pthread_create(
        native: *mut crate::pthread_t,
        attr: *const crate::pthread_attr_t,
        f: extern "C" fn(*mut c_void) -> *mut c_void,
        value: *mut c_void,
    ) -> c_int;
    #[link_name = "@@PT@KIL"]
    pub fn pthread_kill(thread: crate::pthread_t, signal: c_int) -> c_int;
    #[link_name = "@@P3MPG"]
    pub fn pthread_mutexattr_getpshared(
        attr: *const pthread_mutexattr_t,
        pshared: *mut c_int,
    ) -> c_int;
    #[link_name = "@@P3MPS"]
    pub fn pthread_mutexattr_setpshared(attr: *mut pthread_mutexattr_t, pshared: c_int) -> c_int;
    #[link_name = "@@P3RAG"]
    pub fn pthread_rwlockattr_getpshared(
        attr: *const pthread_rwlockattr_t,
        val: *mut c_int,
    ) -> c_int;
    #[link_name = "@@P3RAS"]
    pub fn pthread_rwlockattr_setpshared(attr: *mut pthread_rwlockattr_t, val: c_int) -> c_int;
    #[link_name = "@@PT@GSA"]
    pub fn pthread_attr_getstackaddr(
        arg1: *const crate::pthread_attr_t,
        arg2: *mut *mut c_void,
    ) ->c_int;
    #[link_name = "@@PT@SSA"]
    pub fn pthread_attr_setstackaddr(
        arg1: *mut crate::pthread_attr_t,
        arg2: *mut c_void,
    ) ->c_int;
    #[link_name = "@@PT@ASK"]
    pub fn pthread_attr_setstack(
        arg1: *mut crate::pthread_attr_t,
        arg2: *mut c_void,
        arg3: usize,
    ) ->c_int;
    #[link_name = "@@PT@CPO"]
    pub fn pthread_cleanup_pop(arg1:c_int);
    #[link_name = "@@PT@CPU"]
    pub fn pthread_cleanup_push(
        arg1: Option<
            unsafe extern "C" fn(arg1: *mut c_void),
        >,
        arg2: *mut c_void,
    );
    #[link_name = "@@PT@GC"]
    pub fn pthread_getconcurrency() ->c_int;
    #[link_name = "@@PT@SCS"]
    pub fn pthread_setcancelstate(
        arg1:c_int,
        arg2: *mut c_int,
    ) ->c_int;
    #[link_name = "@@PT@SCT"]
    pub fn pthread_setcanceltype(
        arg1:c_int,
        arg2: *mut c_int,
    ) ->c_int;
    #[link_name = "@@PT@SC"]
    pub fn pthread_setconcurrency(
        arg1:c_int,
    ) ->c_int;
    #[link_name = "@@PT@TC"]
    pub fn pthread_testcancel();
    #[link_name = "@@PT3AGD"]
    pub fn pthread_attr_getdetachstate(
        attr: *const crate::pthread_attr_t,
        detachstate: *mut c_int,
    ) -> c_int;
    #[link_name = "@@PT3CT6"]
    pub fn pthread_cond_timedwait64(
        arg1: *mut crate::pthread_cond_t,
        arg2: *mut crate::pthread_mutex_t,
        arg3: *const crate::timespec,
    ) ->c_int;
    #[link_name = "@@PT3TG"]
    pub fn pthread_mutexattr_gettype(
        arg1: *const pthread_mutexattr_t,
        arg2: *mut c_int,
    ) ->c_int;
    #[link_name = "@@PT3O"]
    pub fn pthread_once(
        arg1: *mut pthread_once_t,
        arg2: Option<unsafe extern "C" fn()>,
    ) ->c_int;
}
extern "C" {
    pub fn iconv(
        cd: iconv_t,
        inbuf: *mut *mut c_char,
        inbytesleft: *mut size_t,
        outbuf: *mut *mut c_char,
        outbytesleft: *mut size_t,
    ) -> size_t;
    #[link_name = "@@ICONVC"]
    pub fn iconv_close(cd: iconv_t) -> c_int;
    #[link_name = "@@A00119"]
    pub fn iconv_open(tocode: *const c_char, fromcode: *const c_char) -> iconv_t;
}

extern "C" {
    #[link_name = "@@A00584"]
    pub fn accept4(
        sockfd: c_int,
        addr: *mut sockaddr,
        addrlen: *mut socklen_t,
        flags: c_int,
    ) -> c_int;
    pub fn aio_cancel(fildes: c_int, aiocbp: *mut crate::aiocb) -> c_int;
    pub fn aio_error(aiocbp: *const crate::aiocb) -> c_int;
    pub fn aio_read(aiocbp: *mut crate::aiocb) -> c_int;
    pub fn aio_return(aiocbp: *const crate::aiocb) -> c_int;
    pub fn aio_suspend(
        aiocb_list: *const *const crate::aiocb,
        nitems: c_int,
        timeout: *const crate::timespec,
    ) -> c_int;
    pub fn aio_write(aiocbp: *mut crate::aiocb) -> c_int;
    #[link_name = "@@A00376"]
    pub fn basename(path: *mut c_char) -> *mut c_char;
    #[link_name = "@@A00406"]
    pub fn bind(
        socket: c_int,
        address: *const crate::sockaddr,
        address_len: crate::socklen_t,
    ) -> c_int;
    #[link_name = "@@CGTIME"]
    pub fn clock_gettime(clk_id: crate::clockid_t, tp: *mut crate::timespec) -> c_int;
    pub fn clearenv() -> c_int;
    #[link_name = "@@DIRFD"]
    pub fn dirfd(dirp: *mut crate::DIR) -> c_int;
    #[link_name = "@@A00377"]
    pub fn dirname(path: *mut c_char) -> *mut c_char;
    pub fn drand48() -> c_double;
    pub fn endgrent();
    pub fn endpwent();
    #[link_name = "@@EUTNT"]
    pub fn endutxent();
    #[link_name = "@@EPLCT"]
    pub fn epoll_create(size: c_int) -> c_int;
    #[link_name = "@@EPLCT1"]
    pub fn epoll_create1(flags: c_int) -> c_int;
    #[link_name = "@@EPLCTL"]
    pub fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut crate::epoll_event) -> c_int;
    #[link_name = "@@EPLWT"]
    pub fn epoll_wait(
        epfd: c_int,
        events: *mut crate::epoll_event,
        maxevents: c_int,
        timeout: c_int,
    ) -> c_int;
    pub fn erand48(xseed: *mut c_ushort) -> c_double;
    #[link_name = "@@EVNTFD"]
    pub fn eventfd(initval: c_uint, flags: c_int) -> c_int;
    #[link_name = "@@A00605"]
    pub fn faccessat(dirfd: c_int, pathname: *const c_char, mode: c_int, flags: c_int) -> c_int;
    pub fn fattach(fildes: c_int, path: *const c_char) -> c_int;
    #[link_name = "@@FDSYNC"]
    pub fn fdatasync(fd: c_int) -> c_int;
    pub fn ffs(value: c_int) -> c_int;
    #[link_name = "@@FSTA64"]
    pub fn fstat64(fildes: c_int, buf: *mut stat64) -> c_int;
    #[link_name = "@@FSTAFS"]
    pub fn fstatfs(fd: c_int, buf: *mut statfs) -> c_int;
    #[link_name = "@@A00239"]
    pub fn ftok(path: *const c_char, id: c_int) -> crate::key_t;
    #[link_name = "@@GETCNT"]
    pub fn getcontext(ucp: *mut ucontext_t) -> c_int;
    pub fn getdtablesize() -> c_int;
    #[link_name = "@@A00253"]
    pub fn getgrent() -> *mut crate::group;
    #[link_name = "@@A00254"]
    pub fn getgrgid(gid: crate::gid_t) -> *mut crate::group;
    #[link_name = "@@A00016"]
    pub fn getgrgid_r(
        gid: crate::gid_t,
        grp: *mut crate::group,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut crate::group,
    ) -> c_int;
    #[link_name = "@@A00255"]
    pub fn getgrnam(name: *const c_char) -> *mut crate::group;
    #[link_name = "@@A00028"]
    pub fn getgrnam_r(
        name: *const c_char,
        grp: *mut crate::group,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut crate::group,
    ) -> c_int;
    #[link_name = "@@GHTID"]
    pub fn gethostid() -> c_long;
    #[link_name = "@@A00087"]
    pub fn getnameinfo(
        sa: *const crate::sockaddr,
        salen: socklen_t,
        host: *mut c_char,
        hostlen: socklen_t,
        serv: *mut c_char,
        servlen: socklen_t,
        flags: c_int,
    ) -> c_int;
    #[link_name = "@@GPAGE"]
    pub fn getpagesize() -> c_int;
    #[link_name = "@@GPRIOR"]
    pub fn getpriority(which: c_int, who: crate::id_t) -> c_int;
    #[link_name = "@@A00264"]
    pub fn getpwent() -> *mut crate::passwd;
    #[link_name = "@@A00640"]
    pub fn getpwent_r(
        pwd: *mut passwd,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut passwd,
    ) -> c_int;
    #[link_name = "@@A00031"]
    pub fn getpwnam_r(
        name: *const c_char,
        pwd: *mut passwd,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut passwd,
    ) -> c_int;
    #[link_name = "@@A00032"]
    pub fn getpwuid_r(
        uid: crate::uid_t,
        pwd: *mut passwd,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut passwd,
    ) -> c_int;
    #[link_name = "@@GRLMT"]
    pub fn getrlimit(resource: c_int, rlim: *mut crate::rlimit) -> c_int;
    #[link_name = "@@GTODY"]  // _ALL_SOURCE is NOT defined version
    pub fn gettimeofday(tp: *mut crate::timeval, tz: *mut c_void) -> c_int;
    #[link_name = "@@GITMR"]
    pub fn getitimer(which: c_int, curr_value: *mut crate::itimerval) -> c_int;
    #[link_name = "@@GTUTE"]
    pub fn getutxent() -> *mut utmpx;
    pub fn getutxid(ut: *const utmpx) -> *mut utmpx;
    #[link_name = "@@GUTLN"]
    pub fn getutxline(ut: *const utmpx) -> *mut utmpx;
    #[link_name = "@@A00378"]
    pub fn glob(
        pattern: *const c_char,
        flags: c_int,
        errfunc: Option<extern "C" fn(epath: *const c_char, errno: c_int) -> c_int>,
        pglob: *mut crate::glob_t,
    ) -> c_int;
    pub fn globfree(pglob: *mut crate::glob_t);
    pub fn hcreate(nelt: size_t) -> c_int;
    pub fn hdestroy();
    pub fn hsearch(entry: entry, action: c_int) -> *mut entry;
    #[link_name = "@@IFFREE"]
    pub fn if_freenameindex(ptr: *mut if_nameindex);
    #[link_name = "@@A00201"]
    pub fn if_nameindex() -> *mut if_nameindex;
    #[link_name = "@@A00277"]
    pub fn initgroups(name: *const c_char, basegid: crate::gid_t) -> c_int;
    pub fn ioctl(fildes: c_int, request: c_int, ...) -> c_int;
    pub fn jrand48(xseed: *mut c_ushort) -> c_long;
    pub fn lcong48(p: *mut c_ushort);
    pub fn lfind(
        key: *const c_void,
        base: *const c_void,
        nelp: *mut size_t,
        width: size_t,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    ) -> *mut c_void;
    pub fn lrand48() -> c_long;
    pub fn lsearch(
        key: *const c_void,
        base: *mut c_void,
        nelp: *mut size_t,
        width: size_t,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    ) -> *mut c_void;
    #[link_name = "@@A00568"]
    pub fn lstat64(path: *const c_char, buf: *mut stat64) -> c_int;
    #[link_name = "@@MAKCNT"]
    pub fn makecontext(ucp: *mut crate::ucontext_t, func: extern "C" fn(), argc: c_int, ...);
    #[link_name = "@@A00611"]
    pub fn mkfifoat(dirfd: c_int, pathname: *const c_char, mode: crate::mode_t) -> c_int;
    #[link_name = "@@A00612"]
    pub fn mknodat(dirfd: c_int, pathname: *const c_char, mode: crate::mode_t, dev: dev_t)
        -> c_int;
    #[link_name = "@@A00138"]  // Format 1: when only _OPEN_SYS is defined
    pub fn mount(
        path: *const c_char,
        filesystem: *mut c_char,
        filesystype: *mut c_char,
        mtm: mtm_t,
        parmlen: c_int,
        parm: *mut c_char
    ) -> c_int;
    pub fn mprotect(addr: *mut c_void, len: size_t, prot: c_int) -> c_int;
    pub fn mrand48() -> c_long;
    pub fn msgctl(msqid: c_int, cmd: c_int, buf: *mut msqid_ds) -> c_int;
    pub fn msgget(key: crate::key_t, msgflg: c_int) -> c_int;
    #[link_name = "@@A00280"]
    pub fn msgrcv(
        msqid: c_int,
        msgp: *mut c_void,
        msgsz: size_t,
        msgtyp: c_long,
        msgflg: c_int,
    ) -> ssize_t;
    #[link_name = "@@A00281"]
    pub fn msgsnd(msqid: c_int, msgp: *const c_void, msgsz: size_t, msgflg: c_int) -> c_int;
    pub fn msync(addr: *mut c_void, len: size_t, flags: c_int) -> c_int;
    #[link_name = "@@A00077"]
    pub fn nl_langinfo(item: crate::nl_item) -> *mut c_char;
    pub fn nrand48(xseed: *mut c_ushort) -> c_long;
    #[link_name = "@@PIPE2"]
    pub fn pipe2(pipefd: *mut c_int, flags: c_int) -> c_int;
    #[link_name = "@@DUP3"]
    pub fn dup3(oldfd: c_int, newfd: c_int, flags: c_int) -> c_int;
    #[link_name = "@@A00249"]
    pub fn popen(command: *const c_char, mode: *const c_char) -> *mut crate::FILE;
    #[link_name = "@@PUTUL"]
    pub fn pututxline(ut: *const utmpx) -> *mut utmpx;
    pub fn rand() -> c_int;
    pub fn readv(fd: c_int, iov: *const crate::iovec, iovcnt: c_int) -> ssize_t;
    #[link_name = "@@A00410"]
    pub fn recvfrom(
        socket: c_int,
        buf: *mut c_void,
        len: size_t,
        flags: c_int,
        addr: *mut crate::sockaddr,
        addrlen: *mut crate::socklen_t,
    ) -> ssize_t;
    #[link_name = "@@A00413"]  // X/Open version
    pub fn recvmsg(sockfd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
    #[link_name = "@@A00041"]
    pub fn regcomp(preg: *mut regex_t, pattern: *const c_char, cflags: c_int) -> c_int;
    #[link_name = "@@A00043"]
    pub fn regerror(
        errcode: c_int,
        preg: *const crate::regex_t,
        errbuf: *mut c_char,
        errbuf_size: size_t,
    ) -> size_t;
    #[link_name = "@@A00045"]
    pub fn regexec(
        preg: *const regex_t,
        input: *const c_char,
        nmatch: size_t,
        pmatch: *mut regmatch_t,
        eflags: c_int,
    ) -> c_int;
    #[link_name = "@@A00047"]
    pub fn regfree(preg: *mut regex_t);
    pub fn seed48(xseed: *mut c_ushort) -> *mut c_ushort;
    pub fn seekdir(dirp: *mut crate::DIR, loc: c_long);
    pub fn semctl(semid: c_int, semnum: c_int, cmd: c_int, ...) -> c_int;
    pub fn semget(key: crate::key_t, nsems: c_int, semflag: c_int) -> c_int;
    pub fn semop(semid: c_int, sops: *mut sembuf, nsops: size_t) -> c_int;
    pub fn send_file(socket: *mut c_int, iobuf: *mut sf_parms, flags: c_uint) -> c_int;
    #[link_name = "@@A00412"]
    pub fn sendmsg(sockfd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;
    #[link_name = "@@SETCNT"]
    pub fn setcontext(ucp: *const ucontext_t) -> c_int;
    #[link_name = "@@SETGRP"]
    pub fn setgroups(ngroups: c_int, ptr: *const crate::gid_t) -> c_int;
    pub fn setgrent();
    #[link_name = "@@A00589"]
    pub fn sethostname(name: *const c_char, len: size_t) -> c_int;
    #[link_name = "@@SPRIOR"]
    pub fn setpriority(which: c_int, who: id_t, priority: c_int) -> c_int;
    pub fn setpwent();
    #[link_name = "@@SRLIM"]
    pub fn setrlimit(resource: c_int, rlim: *const crate::rlimit) -> c_int;
    #[link_name = "@@SITMR"]
    pub fn setitimer(
        which: c_int,
        new_value: *const crate::itimerval,
        old_value: *mut crate::itimerval,
    ) -> c_int;
    #[link_name = "@@SUTNT"]
    pub fn setutxent();
    #[link_name = "@@SIGAL"]
    pub fn sigaltstack(ss: *const stack_t, oss: *mut stack_t) -> c_int;
    #[link_name = "@@SIGSUS"]
    pub fn sigsuspend(mask: *const crate::sigset_t) -> c_int;
    #[link_name = "@@SIGTWA"]
    pub fn sigtimedwait(
        set: *const sigset_t,
        info: *mut siginfo_t,
        timeout: *const crate::timespec,
    ) -> c_int;
    #[link_name = "@@SIG3WT"]
    pub fn sigwait(set: *const sigset_t, sig: *mut c_int) -> c_int;
    #[link_name = "@@SIGWIF"]
    pub fn sigwaitinfo(set: *const sigset_t, info: *mut siginfo_t) -> c_int;
    pub fn shmat(shmid: c_int, shmaddr: *const c_void, shmflg: c_int) -> *mut c_void;
    pub fn shmdt(shmaddr: *const c_void) -> c_int;
    pub fn shmctl(shmid: c_int, cmd: c_int, buf: *mut crate::shmid_ds) -> c_int;
    pub fn shmget(key: key_t, size: size_t, shmflg: c_int) -> c_int;
    pub fn srand(seed: c_uint);
    pub fn srand48(seed: c_long);
    #[link_name = "@@A00569"]
    pub fn stat64(path: *const c_char, buf: *mut stat64) -> c_int;
    #[link_name = "@@A00586"]
    pub fn statfs(path: *const c_char, buf: *mut statfs) -> c_int;
    #[link_name = "@@A00470"]
    pub fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> c_int;
    #[link_name = "@@A00095"]
    pub fn strftime(
        arg1: *mut c_char,
        arg2: size_t,
        arg3: *const c_char,
        arg4: *const tm,
    ) -> size_t;
    #[link_name = "@@A00097"]
    pub fn strptime(s: *const c_char, format: *const c_char, tm: *mut crate::tm) -> *mut c_char;
    #[link_name = "@@SWPCNT"]
    pub fn swapcontext(uocp: *mut ucontext_t, ucp: *const ucontext_t) -> c_int;
    pub fn sync();
    pub fn telldir(dirp: *mut crate::DIR) -> c_long;
    #[link_name = "@@A00296"]
    pub fn uname(buf: *mut crate::utsname) -> c_int;
    #[link_name = "@@A00629"]
    pub fn utimensat(
        dirfd: c_int,
        path: *const c_char,
        times: *const crate::timespec,
        flag: c_int,
    ) -> c_int;
    #[link_name = "@@WAIT4"]
    pub fn wait4(
        pid: crate::pid_t,
        status: *mut c_int,
        options: c_int,
        rusage: *mut crate::rusage,
    ) -> crate::pid_t;
    pub fn waitid(
        idtype: idtype_t,
        id: id_t,
        infop: *mut crate::siginfo_t,
        options: c_int,
    ) -> c_int;
    pub fn writev(fd: c_int, iov: *const crate::iovec, iovcnt: c_int) -> ssize_t;

    pub fn __errno() -> *mut c_int;
    pub fn __fchattr(fd: c_int, attributes: *mut crate::f_attributes, attributes_len: c_int) -> c_int;
    #[link_name = "@@ENVNA"]
    pub fn __EnvnA() -> *mut *mut *mut c_char;
    pub fn __Envn() -> *mut *mut *mut c_char;
    #[link_name = "@@FUTIME"]
    pub fn futimes(fd: c_int, times: *const crate::timeval) -> c_int;
    #[link_name = "@@A00603"]
    pub fn lutimes(file: *const c_char, times: *const crate::timeval) -> c_int;
}

cfg_if! {
    if #[cfg(target_arch = "s390x")] {
        mod s390x;
        pub use self::s390x::*;
    }
}
