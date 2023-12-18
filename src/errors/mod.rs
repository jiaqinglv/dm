use std::fmt;

/// 错误类型
#[derive(Debug, Clone, Copy)]
pub enum WebErrorType {
    /// 配置错误
    ConfigError,
    /// 数据层错误
    DataError,
    /// Biz层错误
    BizError,
    /// 服务层错误
    ServiceError,
    /// API层错误
    ApiError,
}

pub struct WebError {
    /// 错误类型
    kind: WebErrorType,
    /// 错误信息
    message: String,
}

impl WebError {
    pub fn new(kind: WebErrorType, message:&'static str) -> WebError {
        WebError{
            kind,
            message: message.to_string()
        }
    }
}



impl fmt::Display for WebError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind as WebErrorType {
            WebErrorType::ConfigError => write!(f, "配置错误: {}", self.message),
            WebErrorType::DataError => write!(f, "数据层错误: {}", self.message),
            WebErrorType::BizError => write!(f, "Biz层错误: {}", self.message),
            WebErrorType::ServiceError => write!(f, "服务层错误: {}", self.message),
            WebErrorType::ApiError => write!(f, "API层错误: {}", self.message),
        }
    }
}