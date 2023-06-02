use serde::{Serialize, Deserialize};

// 自定义错误类型状态
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Rt {
  AuthFail,
  AuthSuccess,
  Success,
  Fail,
  Error,
}
