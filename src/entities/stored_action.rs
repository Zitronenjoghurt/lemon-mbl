use crate::battle_logic::battle_event_type::BattleEventType;
use crate::entities::action_data::ActionData;
use crate::enums::action_target::ActionTarget;
use crate::enums::team_side::TeamSide;
use crate::get_game_data;
use crate::serialization::arc_ref;
use crate::traits::action_data_access::ActionDataAccess;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoredAction {
    #[serde(with = "arc_ref")]
    data: Arc<ActionData>,
    total_use_count: u32,
}

impl StoredAction {
    pub fn create(id: u16) -> Option<Self> {
        get_game_data().actions.get(id)
            .map(|data| Self::from_data(Arc::clone(data)))
    }

    pub fn from_data(data: Arc<ActionData>) -> Self {
        Self {
            data,
            total_use_count: 0,
        }
    }

    pub fn get_total_use_count(&self) -> u32 {
        self.total_use_count
    }

    pub fn on_use(&mut self) {
        self.total_use_count += 1;
    }

    pub fn validate_target(&self, source_team: &TeamSide, target_team: &TeamSide, source_monster_index: usize, target_monster_index: usize) -> bool {
        for potential_target in self.get_potential_targets() {
            match potential_target {
                ActionTarget::None => return true,
                ActionTarget::OneSelf => {
                    if source_team == target_team && source_monster_index == target_monster_index {
                        return true;
                    }
                }
                ActionTarget::SpecificAlly | ActionTarget::EveryAllyIncludingSelf => {
                    if source_team == target_team {
                        return true;
                    }
                }
                ActionTarget::SpecificOpponent | ActionTarget::EveryOpponent => {
                    if source_team != target_team {
                        return true;
                    }
                }
                ActionTarget::EveryAllyExceptSelf => {
                    if target_team == source_team && source_monster_index != target_monster_index {
                        return true;
                    }
                }
            }
        }
        false
    }
}

impl ActionDataAccess for StoredAction {
    fn get_id(&self) -> u16 {
        self.data.get_id()
    }

    fn get_internal_name(&self) -> &str {
        self.data.get_internal_name()
    }

    fn get_priority(&self) -> u8 {
        self.data.get_priority()
    }

    fn get_event_types(&self) -> &[BattleEventType] {
        self.data.get_event_types()
    }

    fn has_event_type(&self, event_type: &BattleEventType) -> bool {
        self.data.has_event_type(event_type)
    }

    fn get_potential_targets(&self) -> &[ActionTarget] {
        self.data.get_potential_targets()
    }

    fn has_potential_target(&self, action_target: &ActionTarget) -> bool {
        self.data.has_potential_target(action_target)
    }
}