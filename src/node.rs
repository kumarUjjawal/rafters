#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Role {
    Candidate,
    Follower,
    Leader,
}

pub struct RafNode {
    id: u64,
    current_term: u64,
    voted_for: Option<u64>,
    role: Role,
}

impl RafNode {
    pub fn new(id: u64) -> Self {
        RafNode {
            id,
            current_term: 0,
            voted_for: None,
            role: Role::Follower,
        }
    }

    pub fn role(&self) -> Role {
        self.role
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_starts_with_follower() {
        let node = RafNode::new(1);
        assert_eq!(node.role(), Role::Follower);
        assert_eq!(node.current_term, 0);
        assert_eq!(node.voted_for, None);
    }
}
