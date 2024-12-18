mod store;
mod abi;
mod async_io;
use std::sync::Arc;


use store::DictionaryStore;
pub struct Dicfuse{
    pub store: Arc<DictionaryStore>,
}
#[allow(unused)]
impl Dicfuse{
    pub async fn new() -> Self {
        Self {
            store: DictionaryStore::new().await.into(), // Assuming DictionaryStore has a new() method
        }
    }
}


// #[allow(unused)]
// impl FileSystem for Dicfuse{
//     type Inode = u64;

//     type Handle = u64;
    
//     fn init(&self, capable:FsOptions) -> Result<FsOptions> {
//         println!("Dicfuse init....");
//         let s = self.store.clone();
//          s.import();
        
//         //let mut ops = FsOptions::DO_READDIRPLUS | FsOptions::READDIRPLUS_AUTO;
//         Ok(fuse_backend_rs::abi::fuse_abi::FsOptions::empty())
//     }
    
//     fn destroy(&self) {}
    
//     fn lookup(&self, ctx: &Context, parent: Self::Inode, name: &std::ffi::CStr) -> Result<Entry> {
//         println!("[lookup]: ctx:{}, parnet inode:{},name :{:?}",ctx.pid,parent,name);
//         let store = self.store.clone();
//         let mut ppath  = store.find_path(parent).ok_or_else(|| std::io::Error::from_raw_os_error(libc::ENODATA))?;
//         let pitem  = store.get_inode(parent)?;
//         ppath.push(name.to_string_lossy().into_owned());
//         let chil = store.get_by_path(&ppath.to_string())?;
//         let ree = chil.into_entry();
//         println!("[lookup-out]: entry:{:?}",ree);
//         Ok(ree)
//     }
    

//     fn forget(&self, ctx: &Context, inode: Self::Inode, count: u64) {
//         println!("[forget]: ctx:{}, inode:{},count :{}",ctx.pid,inode,count);
//     }
    
//     fn batch_forget(&self, ctx: &Context, requests: Vec<(Self::Inode, u64)>) {
//         println!("[batch-forget]: ctx:{}",ctx.pid);
//         for (inode, count) in requests {
//             self.forget(ctx, inode, count)
//         }
//     }
    
//     fn getattr(
//         &self,
//         ctx: &Context,
//         inode: Self::Inode,
//         handle: Option<Self::Handle>,
//     ) -> std::io::Result<(libc::stat64, std::time::Duration)> {
//         println!("[getattr]: ctx:{},  inode:{},handle :{:?}",ctx.pid,inode ,handle);
//         let store = self.store.clone();
//         let i = store.find_path(inode).ok_or_else(|| std::io::Error::from_raw_os_error(libc::ENODATA))?;
//         let item = store.get_inode(inode)?;
//         let mut entry  = item.get_stat();
        
//         println!("[getattr-out]:entry ->{:?}",entry.attr);
//         Ok((entry.attr,Duration::from_secs(0)))
//     }
    
//     fn setattr(
//         &self,
//         ctx: &Context,
//         inode: Self::Inode,
//         attr: libc::stat64,
//         handle: Option<Self::Handle>,
//         valid: fuse_backend_rs::abi::fuse_abi::SetattrValid,
//     ) -> std::io::Result<(libc::stat64, std::time::Duration)> {
//         println!("[setattr]: not implement.");
//         Err(std::io::Error::from_raw_os_error(libc::ENOSYS))
//     }
    
    
//     fn mknod(
//         &self,
//         ctx: &Context,
//         inode: Self::Inode,
//         name: &std::ffi::CStr,
//         mode: u32,
//         rdev: u32,
//         umask: u32,
//     ) -> std::io::Result<Entry> {
//         println!("[mknod]: not implement.");
//         Err(std::io::Error::from_raw_os_error(libc::ENOSYS))
//     }
    
//     fn mkdir(
//         &self,
//         ctx: &Context,
//         parent: Self::Inode,
//         name: &std::ffi::CStr,
//         mode: u32,
//         umask: u32,
//     ) -> std::io::Result<Entry> {
//         println!("[mkdir]: not implement.");
//         Err(std::io::Error::from_raw_os_error(libc::ENOSYS))
//     }

//     fn unlink(&self, ctx: &Context, parent: Self::Inode, name: &std::ffi::CStr) -> std::io::Result<()> {
//         println!("[unlink]: not implement.");
//         Err(std::io::Error::from_raw_os_error(libc::ENOSYS))
//     }
    
//     fn rmdir(&self, ctx: &Context, parent: Self::Inode, name: &std::ffi::CStr) -> std::io::Result<()> {
//         println!("[rmdir]: not implement.");
//         Err(std::io::Error::from_raw_os_error(libc::ENOSYS))
//     }
    
//     fn rename(
//         &self,
//         ctx: &Context,
//         olddir: Self::Inode,
//         oldname: &std::ffi::CStr,
//         newdir: Self::Inode,
//         newname: &std::ffi::CStr,
//         flags: u32,
//     ) -> std::io::Result<()> {
//         println!("[rename]: not implement.");
//         Err(std::io::Error::from_raw_os_error(libc::ENOSYS))
//     }
    
//     fn link(
//         &self,
//         ctx: &Context,
//         inode: Self::Inode,
//         newparent: Self::Inode,
//         newname: &std::ffi::CStr,
//     ) -> std::io::Result<Entry> {
//         println!("[link]: not implement.");
//         Err(std::io::Error::from_raw_os_error(libc::ENOSYS))
//     }
    
//     fn open(
//         &self,
//         ctx: &Context,
//         inode: Self::Inode,
//         flags: u32,
//         fuse_flags: u32,
//     ) -> std::io::Result<(Option<Self::Handle>, fuse_backend_rs::abi::fuse_abi::OpenOptions, Option<u32>)> {
//         println!("[open]: not implement.");
//         // Matches the behavior of libfuse.
//         Ok((None, fuse_backend_rs::abi::fuse_abi::OpenOptions::empty(), None))
//     }
    
//     fn create(
//         &self,
//         ctx: &Context,
//         parent: Self::Inode,
//         name: &std::ffi::CStr,
//         args: fuse_backend_rs::abi::fuse_abi::CreateIn,
//     ) -> std::io::Result<(Entry, Option<Self::Handle>, fuse_backend_rs::abi::fuse_abi::OpenOptions, Option<u32>)> {
//         println!("[create]: not implement.");
//         Err(std::io::Error::from_raw_os_error(libc::ENOSYS))
//     }
    
//     fn flush(
//         &self,
//         ctx: &Context,
//         inode: Self::Inode,
//         handle: Self::Handle,
//         lock_owner: u64,
//     ) -> std::io::Result<()> {
//         println!("[flush]: not implement.");
//         Err(std::io::Error::from_raw_os_error(libc::ENOSYS))
//     }
    
//     fn fsync(
//         &self,
//         ctx: &Context,
//         inode: Self::Inode,
//         datasync: bool,
//         handle: Self::Handle,
//     ) -> std::io::Result<()> {
//         println!("[fsync]: not implement.");
//         Err(std::io::Error::from_raw_os_error(libc::ENOSYS))
//     }
    
//     fn fallocate(
//         &self,
//         ctx: &Context,
//         inode: Self::Inode,
//         handle: Self::Handle,
//         mode: u32,
//         offset: u64,
//         length: u64,
//     ) -> std::io::Result<()> {
//         println!("[fallocate]: not implement.");
//         Err(std::io::Error::from_raw_os_error(libc::ENOSYS))
//     }
    
//     fn release(
//         &self,
//         ctx: &Context,
//         inode: Self::Inode,
//         flags: u32,
//         handle: Self::Handle,
//         flush: bool,
//         flock_release: bool,
//         lock_owner: Option<u64>,
//     ) -> std::io::Result<()> {
//         println!("[release]: not implement.");
//         Err(std::io::Error::from_raw_os_error(libc::ENOSYS))
//     }
    
//     fn statfs(&self, ctx: &Context, inode: Self::Inode) -> std::io::Result<libc::statvfs64> {
//         println!("[statfs]: not implement.");
//         // Safe because we are zero-initializing a struct with only POD fields.
//         let mut st: libc::statvfs64 = unsafe { std::mem::zeroed() };
//         // This matches the behavior of libfuse as it returns these values if the
//         // filesystem doesn't implement this method.
//         st.f_namemax = 255;
//         st.f_bsize = 512;
//         Ok(st)
//     }
    
//     fn setxattr(
//         &self,
//         ctx: &Context,
//         inode: Self::Inode,
//         name: &std::ffi::CStr,
//         value: &[u8],
//         flags: u32,
//     ) -> std::io::Result<()> {
//         println!("[setxattr]: not implement.");
//         Err(std::io::Error::from_raw_os_error(libc::ENOSYS))
//     }
    
//     fn getxattr(
//         &self,
//         ctx: &Context,
//         inode: Self::Inode,
//         name: &std::ffi::CStr,
//         size: u32,
//     ) -> std::io::Result<fuse_backend_rs::api::filesystem::GetxattrReply> {
//         println!("[getxattr]: not implement.");
//         Err(std::io::Error::from_raw_os_error(libc::ENOSYS))
//     }
    
//     fn listxattr(
//         &self,
//         ctx: &Context,
//         inode: Self::Inode,
//         size: u32,
//     ) -> std::io::Result<fuse_backend_rs::api::filesystem::ListxattrReply> {
//         println!("[listxattr]: not implement.");
//         Err(std::io::Error::from_raw_os_error(libc::ENOSYS))
//     }
    
//     fn opendir(
//         &self,
//         ctx: &Context,
//         inode: Self::Inode,
//         flags: u32,
//     ) -> std::io::Result<(Option<Self::Handle>, fuse_backend_rs::abi::fuse_abi::OpenOptions)> {
//         // Matches the behavior of libfuse.
//         println!("[opendir]: not implement.");
//         Ok((Some(inode), fuse_backend_rs::abi::fuse_abi::OpenOptions::empty()))
//     }
    
//     fn readdir(
//         &self,
//         ctx: &Context,
//         inode: Self::Inode,
//         handle: Self::Handle,
//         size: u32,
//         offset: u64,
//         add_entry: &mut dyn FnMut(fuse_backend_rs::api::filesystem::DirEntry) -> std::io::Result<usize>,
//     ) -> std::io::Result<()> {
//         println!("[readdir]: inode:{},handle:{},size:{},offset:{}",inode,handle,size,offset);
//         self.store.do_readdir(ctx, inode, handle, size, offset, add_entry)
//     }
    
//     fn readdirplus(
//         &self,
//         ctx: &Context,
//         inode: Self::Inode,
//         handle: Self::Handle,
//         size: u32,
//         offset: u64,
//         add_entry: &mut dyn FnMut(fuse_backend_rs::api::filesystem::DirEntry, Entry) -> std::io::Result<usize>,
//     ) -> std::io::Result<()> {
//         println!("[readdirplus]: not implement.");
//         Err(std::io::Error::from_raw_os_error(libc::ENOSYS))
//     }
    
//     fn access(&self, ctx: &Context, inode: Self::Inode, mask: u32) -> std::io::Result<()> {
//         println!("[access]: not implement.");
//         Ok(())
//     }
    

// }

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;

    use tokio::signal;

    use crate::dicfuse::Dicfuse;


    #[tokio::test]
    async fn test_mount_dic(){
        let fs = Dicfuse::new().await;
        let mountpoint =OsStr::new("/home/luxian/dic") ;
        let mut mount_handle =  crate::server::mount_filesystem(fs, mountpoint).await;
        let handle = &mut mount_handle;
        tokio::select! {
            res = handle => res.unwrap(),
            _ = signal::ctrl_c() => {
                mount_handle.unmount().await.unwrap()
            }
        }
    
    }
    
}
