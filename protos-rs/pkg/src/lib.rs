pub mod gateway {
    pub mod assettransfer {
        include!(concat!("./generated", "/gateway.assettransfer.rs"));
    }
}

pub mod common {
    pub mod ack {
        include!(concat!("./generated", "/common.ack.rs"));
    }
}
