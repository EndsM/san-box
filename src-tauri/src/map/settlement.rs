use crate::map::id::SettlementId;

pub struct Settlement {
    id: SettlementId,
    name: String,
}

impl Settlement {
    pub fn id(&self) -> SettlementId {
        self.id
    }
}
