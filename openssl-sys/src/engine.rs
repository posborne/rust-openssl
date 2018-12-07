use libc::*;

use *;

extern "C" {
    pub fn ENGINE_get_first() -> *mut ENGINE;
    pub fn ENGINE_get_last() -> *mut ENGINE;
    pub fn ENGINE_get_next(e: *mut ENGINE) -> *mut ENGINE;
    pub fn ENGINE_get_prev(e: *mut ENGINE) -> *mut ENGINE;

    pub fn ENGINE_add(e: *mut ENGINE) -> c_int;
    pub fn ENGINE_remove(e: *mut ENGINE) -> c_int;

    pub fn ENGINE_by_id(id: *const c_char) -> *mut ENGINE;
    pub fn ENGINE_init(e: *mut ENGINE) -> c_int;
    pub fn ENGINE_finish(e: *mut ENGINE) -> c_int;

    pub fn ENGINE_load_builtin_engines();

    pub fn ENGINE_get_default_RSA() -> *mut ENGINE;
    pub fn ENGINE_get_default_DSA() -> *mut ENGINE;
    pub fn ENGINE_get_default_ECDH() -> *mut ENGINE;
    pub fn ENGINE_get_default_ECDSA() -> *mut ENGINE;
    pub fn ENGINE_get_default_DH() -> *mut ENGINE;
    pub fn ENGINE_get_default_RAND() -> *mut ENGINE;
    pub fn ENGINE_get_cipher_engine(nid: c_int) -> *mut ENGINE;
    pub fn ENGINE_get_digest_engine(nid: c_int) -> *mut ENGINE;

    pub fn ENGINE_set_default_RSA(e: *mut ENGINE) -> c_int;
    pub fn ENGINE_set_default_DSA(e: *mut ENGINE) -> c_int;
    pub fn ENGINE_set_default_ECDH(e: *mut ENGINE) -> c_int;
    pub fn ENGINE_set_default_ECDSA(e: *mut ENGINE) -> c_int;
    pub fn ENGINE_set_default_DH(e: *mut ENGINE) -> c_int;
    pub fn ENGINE_set_default_RAND(e: *mut ENGINE) -> c_int;
    pub fn ENGINE_set_default_ciphers(e: *mut ENGINE) -> c_int;
    pub fn ENGINE_set_default_digests(e: *mut ENGINE) -> c_int;
    pub fn ENGINE_set_default_string(e: *mut ENGINE, list: *const c_char) -> c_int;

    pub fn ENGINE_set_default(e: *mut ENGINE, flags: c_uint) -> c_int;

    pub fn ENGINE_get_table_flags() -> c_uint;
    pub fn ENGINE_set_table_flags(flags: c_uint);

    pub fn ENGINE_register_RSA(e: *mut ENGINE) -> c_int;
    pub fn ENGINE_unregister_RSA(e: *mut ENGINE);
    pub fn ENGINE_register_all_RSA();
    pub fn ENGINE_register_DSA(e: *mut ENGINE) -> c_int;
    pub fn ENGINE_unregister_DSA(e: *mut ENGINE);
    pub fn ENGINE_register_all_DSA();
    pub fn ENGINE_register_ECDH(e: *mut ENGINE) -> c_int;
    pub fn ENGINE_unregister_ECDH(e: *mut ENGINE);
    pub fn ENGINE_register_all_ECDH();
    pub fn ENGINE_register_ECDSA(e: *mut ENGINE) -> c_int;
    pub fn ENGINE_unregister_ECDSA(e: *mut ENGINE);
    pub fn ENGINE_register_all_ECDSA();
    pub fn ENGINE_register_DH(e: *mut ENGINE) -> c_int;
    pub fn ENGINE_unregister_DH(e: *mut ENGINE);
    pub fn ENGINE_register_all_DH();
    pub fn ENGINE_register_RAND(e: *mut ENGINE) -> c_int;
    pub fn ENGINE_unregister_RAND(e: *mut ENGINE);
    pub fn ENGINE_register_all_RAND();
    pub fn ENGINE_register_ciphers(e: *mut ENGINE) -> c_int;
    pub fn ENGINE_unregister_ciphers(e: *mut ENGINE);
    pub fn ENGINE_register_all_ciphers();
    pub fn ENGINE_register_digests(e: *mut ENGINE) -> c_int;
    pub fn ENGINE_unregister_digests(e: *mut ENGINE);
    pub fn ENGINE_register_all_digests();
    pub fn ENGINE_register_complete(e: *mut ENGINE) -> c_int;
    pub fn ENGINE_register_all_complete() -> c_int;
}
