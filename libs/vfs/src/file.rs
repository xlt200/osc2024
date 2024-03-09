use library::time::Time;

#[repr(u16)]
pub enum Umode {
    OwnerRead = 0b100000000,
    OwnerWrite = 0b010000000,
    OwnerExecute = 0b001000000,
    GroupRead = 0b000100000,
    GroupWrite = 0b000010000,
    GroupExecute = 0b000001000,
    OtherRead = 0b100,
    OtherWrite = 0b010,
    OtherExecute = 0b001,
}

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub umode: u16,
    pub uid: u32,
    pub gid: u32,
    /**
     * File last access time
     */
    pub atime: Time,
    /**
     * File content change time
     */
    pub mtime: Time,
    /**
     * File struct change time
     */
    pub ctime: Time,
}
