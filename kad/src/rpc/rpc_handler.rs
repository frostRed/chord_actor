//! 根据 RPC 请求数据类型，返回正确的响应

use super::route_table::RouteTable;

pub struct RpcHandler {
    route_table: RouteTable,
}
