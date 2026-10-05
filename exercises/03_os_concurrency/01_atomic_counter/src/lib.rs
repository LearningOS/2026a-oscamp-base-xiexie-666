use std::sync::atomic::{AtomicU64, Ordering};
pub struct AtomicCounter { value: AtomicU64 }
impl AtomicCounter {
 pub const fn new(init:u64)->Self{Self{value:AtomicU64::new(init)}}
 pub fn increment(&self)->u64{self.value.fetch_add(1,Ordering::Relaxed)}
 pub fn decrement(&self)->u64{self.value.fetch_sub(1,Ordering::Relaxed)}
 pub fn get(&self)->u64{self.value.load(Ordering::Relaxed)}
 pub fn compare_and_swap(&self,e:u64,n:u64)->Result<u64,u64>{self.value.compare_exchange(e,n,Ordering::AcqRel,Ordering::Acquire)}
 pub fn fetch_multiply(&self,m:u64)->u64{loop{let c=self.get();if let Ok(v)=self.compare_and_swap(c,c*m){return v;}}}
}
