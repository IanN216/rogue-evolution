#[derive(Clone, Debug, PartialEq)]
pub struct KingdomState {
    pub name: String,
    pub resources: f32,
    pub corruption: f32,
    pub order: f32,
    pub is_active: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KingdomRole {
    Citizen,
    Soldier,
    Leader,
    Refugee,
    Slave,
    ExperimentSubject,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KingdomMember {
    pub kingdom_id: u32,
    pub role: KingdomRole,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HordeLeader;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HordeMember {
    pub leader_entity: hecs::Entity,
}
