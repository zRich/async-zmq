pub struct ZmqSocketReq {
    ctx: Option<ZmqContext>,
    sock_type: ZmqSocketType,
}

impl ZmqSocketReq {
    pub fn new(ctx: ZmqContext) -> Self {
        Self {
            ctx,
            sock_type = ZmqSocketType::REP,
        }
    }

    pub fn send()
}


