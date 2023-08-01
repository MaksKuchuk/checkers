use std::net::Ipv4Addr;

#[derive(PartialEq)]
pub enum PlayerKind {
    First,
    Second,
}

impl Clone for PlayerKind {
    fn clone(&self) -> Self {
        match self {
            PlayerKind::First => PlayerKind::First,
            PlayerKind::Second => PlayerKind::Second,
        }
    }
}

pub struct Player {
    name: Option<String>,
    result: i32,
    order: Option<PlayerKind>,
    ip: Option<Ipv4Addr>,
    port: Option<i32>,
}

impl Player {
    pub fn new(name: String, order: PlayerKind) -> Player {
        Player {
            name: Some(name),
            result: 0,
            order: Some(order),
            ip: None,
            port: None,
        }
    }

    pub const fn default() -> Player {
        Player {
            name: None,
            result: 0,
            order: None,
            ip: None,
            port: None,
        }
    }

    pub fn fill(&mut self, name: String, order: PlayerKind, ip: Ipv4Addr, port: i32) {
        self.name = Some(name);
        self.order = Some(order);
        self.result = 0;
        self.ip = Some(ip);
        self.port = Some(port);
    }

    pub fn order(&self) -> PlayerKind {
        self.order.as_ref().unwrap().clone()
    }

    pub fn result(&self) -> i32 {
        self.result
    }

    pub fn result_add_one(&mut self) {
        self.result += 1;
    }
}
