
pub enum CodeMsg {
    SUCCESS,
    PARAMETER_ERROR,
    NOT_LOGIN,
    LOGIN_EXPIRED,
    SYSTEM_REPAIR,
    FAIL,
}

impl CodeMsg{
    pub fn get_enum_by_code<'a>(code: i32) -> Option<&'a CodeMsg> {
        match code{
            200 => Some(CodeMsg::SUCCESS),
            400 => Some(CodeMsg::PARAMETER_ERROR),
            401 => Some(CodeMsg::NOT_LOGIN),
            402 => Some(CodeMsg::LOGIN_EXPIRED),
            503 => Some(CodeMsg::SYSTEM_REPAIR),
            500 => Some(CodeMsg::FAIL),
        }
        None
    }

    pub fn get_code(&self) -> i32 {
        match self{
            CodeMsg::SUCCESS => 200,
            CodeMsg::PARAMETER_ERROR => 400,
            CodeMsg::NOT_LOGIN => 401,
            CodeMsg::LOGIN_EXPIRED => 402,
            CodeMsg::SYSTEM_REPAIR => 503,
            CodeMsg::FAIL => 500,
        }
    }

    pub fn get_desc(&self) -> &str {
        match self{
            CodeMsg::SUCCESS => "成功！",
            CodeMsg::PARAMETER_ERROR => "参数异常！",
            CodeMsg::NOT_LOGIN => "未登陆，请登陆后再进行操作！",
            CodeMsg::LOGIN_EXPIRED => "登录已过期，请重新登陆！",
            CodeMsg::SYSTEM_REPAIR => "系统维护中，敬请期待！",
            CodeMsg::FAIL => "服务异常！",
        }
    }
}