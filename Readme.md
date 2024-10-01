
# Wager Markets

Wager Markets seeks to provide a smooth and easy sport prediction experience for web3 sport ethusaists. Predict outcomes on all sort of sport events, from matches to tournaments, player transfers, coach change, award winners and so much more.

### Program Id:
```bash 
Program Id: 86Uce9bTCk2uo8DfjMyvj1R3f9njWhSfWVySDS8epP8S
```


[Pitch Deck](https://drive.google.com/file/d/1LedrUd8oNfdg_vJxgW0FCP9jTR8ZCs63/view?usp=sharing)

## Acknowledgements

 - [Turbin3](https://turbin3.com)
 - [WBA](https://https://solana.web3builders.dev/)
 - [Solana cookbook](https://solanacookbook.com)
 - [Anchor](https://www.anchor-lang.com/)
 


## Installation
Install Rust, Rustup and Cargo

Open up a terminal window (for MacOS) or command prompt (for Windows) and paste this command.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Install Solana Tool Suite using brew 
```bash
  brew install solana
```
verify the installation by checking the version:
```bash
  solana --version
  ```

Anchor CLI: Install the Anchor CLI, which includes tools for building and deploying Solana programs

```bash
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked

```
Verify the installation by checking the Anchor version:
```bash
anchor --version
```
initialize a new anchor project using:
```bash
anchor init "program_name"
```
## Documentation

The architecture of the program consist of:

```bash
contexts
states
errors.rs 
lib.rs
```
It is important to understand the main configuration of the program. The main configuration state of the program is the event configuration stored in the event.rs file. it consist of:
```
#[account]
#[derive(InitSpace)]
pub struct Event {
    pub creator: Pubkey,
    pub id: u64, 
    #[max_len(6,50)]
    pub outcomes: Vec<String>,
    pub winning_outcome:u8,
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
        self.total_bets = vec![0; self.outcomes.len()];
        self.winner = None
        
        Ok(())
    }
}


#[derive(InitSpace,AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]

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
```

In this event.rs , we store the following structure onchain:

### The ```Event``` struct

#### Fields:
- creator (Pubkey): The public key of the user who created the event.

- id (u64): A unique identifier for the event.

- outcomes (Vec<String>): A vector of possible outcomes (e.g., "Team A wins", "Team B wins"). The #[max_len(6,50)] attribute indicates it can have between 6 and 50 elements, where each element is a string.

- winning_outcome (u8): Stores the index of the winning outcome.

- odds (Vec<u64>): A vector storing the odds associated with each outcome. The length is constrained by #[max_len(10, 6)] to a maximum of 10 elements, where each number is an unsigned 64-bit integer.

- is_active (bool): Boolean flag indicating whether the event is active or not.

- total_bets (Vec<u64>): A vector storing the total amount of bets placed on each outcome. The #[max_len(10, 32)] attribute indicates it can store up to 10 outcomes with a maximum of 32 total bets.

- winner (Option<String>): Stores the winning outcome once the event is resolved. It’s wrapped in Option since it may not be decided when the event is created. The #[max_len(50)] attribute constrains the string length.

- bump (u8): A bump seed used in Anchor’s PDA (Program Derived Address) to ensure uniqueness and security when generating addresses for the account.

The space for the account is calculated using the InitSpace derive macro (we take anchor discriminator into cossideratiion when initializing the account)

#### ```set_data``` method 

This method is designed to initialize and set the data for an event. The key functionality is:

- #### Checks for data consistency: It verifies that the number of outcomes matches the number of odds. If there’s a mismatch, it returns an error (ErrorCode::OutcomesOddsMismatch).

- #### Validates outcome length: It ensures the number of outcomes is within an acceptable range (1 to 6). If it's out of range, it returns an error (ErrorCode::InvalidOutcomesCount).

- #### Initializes the event's data:
    - The event creator’s public key (creator), ID (event_id), outcomes, and odds are set.

    - The event is marked as active (is_active).
    - The total_bets vector is initialized to zero for each outcome.
    - The winner is set to None since the event has just been created.


### The ```EventType``` Enum

The ```EventType``` enum defines different types of events that can exist in this prediction system. It allows the event to be categorized by type, such as a sports match, tournament, or player transfer. It uses the #[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)] attributes, which auto-derive implementations for several traits needed in Solana programs and serialization.

#### Variants:

- Match: Represents a match event.

- Tournament: Represents a tournament.

- SeasonOutcome: Represents a season's outcome.

- PlayerTransfer: Represents a player transfer event.

- CoachChange: Represents a coach change event.

- AwardWinner: Represents an award winner event.

- Custom(String): Allows a custom event type, where the string can represent any user-defined type of event. The string length is constrained to a maximum of 50 characters with #[max_len(50)].

#### to_string Method:

This method converts each ```EventType``` enum variant into its corresponding string representation. For example:

- EventType::Match becomes "Match".
- EventType::Custom("Special Event") becomes "Special Event".
This is very be useful for converting the enum into a human-readable format or for use in UIs.




## Deployment
To deploy to devnet, Set the cluster by runing:
```bash
solana config set --url https://api.devnet.solana.com
```
#### build:
build the program by running
```bash
  anchor build
```
#### deploy:
deploy to devnet by executing:
```bash
anchor deploy
```


### Program Id:
```bash 
Program Id: 86Uce9bTCk2uo8DfjMyvj1R3f9njWhSfWVySDS8epP8S
```


## Authors

- [@princeadxisrael](https://www.github.com/princeadxisrael)
