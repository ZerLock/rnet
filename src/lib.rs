pub mod client {
    pub mod client;
    pub mod udp;
    pub mod tcp;
}

pub mod server {
    pub mod server;
    pub mod udp;
    pub mod tcp;
}

pub mod logic {
    pub mod client;
    pub mod server;
}

pub mod shared {
    pub mod packet;
}
