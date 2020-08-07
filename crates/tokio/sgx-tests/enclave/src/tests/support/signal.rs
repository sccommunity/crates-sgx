use sgx_libc as libc;

use sgx_signal::raise_signal;

pub fn send_signal(signal: libc::c_int) {
 
        //assert_eq!(kill(getpid(), signal), 0);
    raise_signal(signal);
 
}
