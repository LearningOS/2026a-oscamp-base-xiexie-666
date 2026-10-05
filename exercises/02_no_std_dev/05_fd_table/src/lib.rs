use std::sync::Arc;
pub trait File: Send + Sync { fn read(&self, buf:&mut [u8])->isize; fn write(&self, buf:&[u8])->isize; }
pub struct FdTable { entries: Vec<Option<Arc<dyn File>>> }
impl FdTable {
 pub fn new()->Self { Self{entries:Vec::new()} }
 pub fn alloc(&mut self,file:Arc<dyn File>)->usize { for (i,s) in self.entries.iter_mut().enumerate(){ if s.is_none(){*s=Some(file);return i;} } self.entries.push(Some(file)); self.entries.len()-1 }
 pub fn get(&self,fd:usize)->Option<Arc<dyn File>> { self.entries.get(fd).and_then(|x|x.as_ref().cloned()) }
 pub fn close(&mut self,fd:usize)->bool { match self.entries.get_mut(fd){Some(s@Some(_))=>{*s=None;true},_=>false} }
 pub fn count(&self)->usize { self.entries.iter().filter(|x|x.is_some()).count() }
}
impl Default for FdTable { fn default()->Self{Self::new()} }
