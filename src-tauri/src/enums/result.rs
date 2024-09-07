


pub enum Result{
    /// 已存在
    Existed,
    /// 创建成功
    Created,
    /// 创建失败
    Failed(String),
}