//! Kad 网络中的节点服务模块

use crate::rpc::RpcHandler;

/// 节点服务，启动后监听端口，响应请求
pub struct NodeService {
    rpc_handler: RpcHandler,
}

impl NodeService {
    /// 构造函数
    pub fn new() -> Self {
        todo!()
    }

    /// 服务启动
    pub fn start(&self) {
        todo!()
    }
}
