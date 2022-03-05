use crate::config::{ USER_STACK_SIZE };
use crate::utils::{ get_used_stack_range, range_contains, r_sscratch };
use crate::task::{ get_current_addr_range };

const FD_STDOUT: usize = 1;

// YOUR JOB: 修改 sys_write 使之通过测试
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            if len == 0 {
                return 0;
            }
            let start_addr = buf as usize;
            let addr_range_to_access = (start_addr, start_addr + len - 1);
            // sscratch stores the user stack sp
            let stack_range = get_used_stack_range(r_sscratch(), USER_STACK_SIZE);
            if range_contains(stack_range, addr_range_to_access) ||
               range_contains(get_current_addr_range(), addr_range_to_access) {
                let slice = unsafe { core::slice::from_raw_parts(buf, len) };
                let str = core::str::from_utf8(slice).unwrap();
                print!("{}", str);
                len as isize
            }
            else {
                -1
            }
        }
        _ => {
            // panic!("Unsupported fd in sys_write!");
            -1
        }
    }
}
