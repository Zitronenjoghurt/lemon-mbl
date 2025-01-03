#[derive(Debug, thiserror::Error)]
pub enum BattleError {
    #[error("Invalid source monster index")]
    InvalidSourceMonsterIndex,
    #[error("Invalid target monster index")]
    InvalidTargetMonsterIndex,
    #[error("Invalid monster index")]
    InvalidMonsterIndex,
    #[error("Invalid action index")]
    InvalidActionIndex,
    #[error("Invalid action target")]
    InvalidActionTarget,
    #[error("Insufficient momentum")]
    InsufficientMomentum,
    #[error("Insufficient energy")]
    InsufficientEnergy,
    #[error("Insufficient hp")]
    InsufficientHp,
    #[error("Already moved")]
    AlreadyMoved,
}