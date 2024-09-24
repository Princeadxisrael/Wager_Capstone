use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Event {
    pub creator: Pubkey,
    pub id: u64, //unique identifier for an event
    #[max_len(6,50)]
    pub outcomes: Vec<String>, 
    #[max_len(10, 6)]
    pub odds: Vec<u64>,
    pub is_active: bool,
    #[max_len(10, 32)]
    pub total_bets: Vec<u64>,
    #[max_len(50)]
    pub winner: Option<String>,
    pub bump: u8,
}

impl Event {
    pub fn set_data(
        &mut self,
        event_id: u64,
        outcomes: Vec<String>,
        odds: Vec<u64>,
        creator: Pubkey
    ) -> Result<()> {
        //check if number of outcome matches number of odds
        if outcomes.len() != odds.len() {
            return Err(ErrorCode::OutcomesOddsMismatch.into());
        }
        if outcomes.len() > 6 || outcomes.is_empty() {
            return Err(ErrorCode::InvalidOutcomesCount.into());
        }
        
        self.creator = creator;
        self.id = event_id;
        self.outcomes = outcomes;
        self.odds = odds;
        self.is_active = true;
        self.total_bets = vec![0; self.outcomes.len()]; //initialized with zeros per outcome
        self.winner = None; //None as the event is just being created
        
        Ok(())
    }
}


#[derive(InitSpace,AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
//This enum represents different types of events in the prediction contexts
pub enum EventType {
    Match,
    Tournament,
    SeasonOutcome,
    PlayerTransfer,
    CoachChange,
    AwardWinner,
    Custom( #[max_len(50)]String), //custom variant for a string value for any custom event types
}

impl EventType {
    pub fn to_string(&self) -> String {
        match self {
            EventType::Match => "Match".to_string(),
            EventType::Tournament => "Tournament".to_string(),
            EventType::SeasonOutcome => "Season Outcome".to_string(),
            EventType::PlayerTransfer => "Player Transfer".to_string(),
            EventType::CoachChange => "Coach Change".to_string(),
            EventType::AwardWinner => "Award Winner".to_string(),
            EventType::Custom(s) => s.clone(),
        }
    }
}

#[error_code]
pub enum ErrorCode {
    OutcomesOddsMismatch,
    InvalidOutcomesCount,
}