use core::any::Any;
/// Trait for block devices
/// which reads and writes data in the unit of blocks
/// send :表示类型可以安全地在线程间传递所有权
/// 允许将块设备实例移动到其他线程
/// 对于可能在不同线程处理I/O请求的文件系统必不可少
/// sync: 表示类型可以安全地在多个线程间共享引用
/// 允许多个线程同时通过&BlockDevice访问同一设备
///支持并发读取操作
/// Any: 允许在运行时检查和转换类型
/// 使文件系统能够处理多种具体设备类型（如虚拟设备、物理磁盘等)
pub trait BlockDevice: Send + Sync + Any {
    ///Read data form block to buffer
    fn read_block(&self, block_id: usize, buf: &mut [u8]);
    ///Write data from buffer to block
    fn write_block(&self, block_id: usize, buf: &[u8]);
}
