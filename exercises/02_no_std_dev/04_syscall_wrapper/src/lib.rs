#![cfg_attr(not(test), no_std)]
pub struct SyscallABI { pub arch: &'static str, pub instruction: &'static str, pub id_reg: &'static str, pub ret_reg: &'static str, pub arg_regs: &'static [&'static str], pub clobbered: &'static [&'static str], pub sys_write: usize, pub sys_read: usize, pub sys_close: usize, pub sys_exit: usize }
fn abi(arch:&'static str,instruction:&'static str,id_reg:&'static str,ret_reg:&'static str,args:&'static [&'static str],clob:&'static [&'static str],w:usize,r:usize,c:usize,e:usize)->SyscallABI { SyscallABI{arch,instruction,id_reg,ret_reg,arg_regs:args,clobbered:clob,sys_write:w,sys_read:r,sys_close:c,sys_exit:e} }
pub fn x86_64_abi()->SyscallABI { abi("x86_64","syscall","rax","rax", &["rdi","rsi","rdx","r10","r8","r9"], &["rcx","r11"],1,0,3,60) }
pub fn aarch64_abi()->SyscallABI { abi("aarch64","svc #0","x8","x0", &["x0","x1","x2","x3","x4","x5"], &[],64,63,57,93) }
pub fn riscv64_abi()->SyscallABI { abi("riscv64","ecall","a7","a0", &["a0","a1","a2","a3","a4","a5"], &[],64,63,57,93) }
#[cfg(all(target_arch="x86_64",target_os="linux"))]
pub unsafe fn syscall3(id:usize,a0:usize,a1:usize,a2:usize)->isize { let mut ret=id; core::arch::asm!("syscall",inlateout("rax") ret,in("rdi") a0,in("rsi") a1,in("rdx") a2,lateout("rcx") _,lateout("r11") _); ret as isize }
#[cfg(all(target_arch="aarch64",target_os="linux"))]
pub unsafe fn syscall3(id:usize,a0:usize,a1:usize,a2:usize)->isize { let mut ret=a0; core::arch::asm!("svc #0",in("x8") id,inlateout("x0") ret,in("x1") a1,in("x2") a2); ret as isize }
#[cfg(not(target_os="linux"))] pub unsafe fn syscall3(_:usize,_:usize,_:usize,_:usize)->isize { -1 }
#[cfg(target_arch="x86_64")] const W:usize=1; #[cfg(target_arch="x86_64")] const R:usize=0; #[cfg(target_arch="x86_64")] const C:usize=3; #[cfg(target_arch="x86_64")] const E:usize=60;
#[cfg(target_arch="aarch64")] const W:usize=64; #[cfg(target_arch="aarch64")] const R:usize=63; #[cfg(target_arch="aarch64")] const C:usize=57; #[cfg(target_arch="aarch64")] const E:usize=93;
#[cfg(not(any(target_arch="x86_64",target_arch="aarch64")))] const W:usize=0; #[cfg(not(any(target_arch="x86_64",target_arch="aarch64")))] const R:usize=0; #[cfg(not(any(target_arch="x86_64",target_arch="aarch64")))] const C:usize=0; #[cfg(not(any(target_arch="x86_64",target_arch="aarch64")))] const E:usize=0;
pub fn sys_write(fd:usize,b:&[u8])->isize { unsafe{syscall3(W,fd,b.as_ptr() as usize,b.len())} }
pub fn sys_read(fd:usize,b:&mut [u8])->isize { unsafe{syscall3(R,fd,b.as_mut_ptr() as usize,b.len())} }
pub fn sys_close(fd:usize)->isize { unsafe{syscall3(C,fd,0,0)} }
pub fn sys_exit(code:i32)->! { unsafe{syscall3(E,code as usize,0,0)}; loop{core::hint::spin_loop()} }
