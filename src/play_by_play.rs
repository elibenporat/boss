//! API Bindings for the MLB Play by Play API hosted at http://statsapi.mlb.com/api/v1/game/{game_pk}/playByPlay.<br/>
//! All data are subject to MLB Advanced Media copyright
//! 
//! There are differences with respect to MLB-level data as compared to Minor League data. This means that for fields that are MLB specific
//! we'll have to wrap them in Option<T> to signify they won't always be there. We want a unified data set, so we'll create one set of structs
//! 
//! From impls are built in a modular fashion, allowing us to flatten out the deserialization into simpler structs. Additionally, we'll take great care to
//! convert all text into enums so that every single type will implement Copy, allowing us to efficiently flatten the entire file. This means we will
//! ignore all free-form "description" fields. All the relevent data in the description are in other parts of the data, including whether the ball was hit hard.
//! This leads to an incredibly efficient data structure.
//! 
//! Creating enums for all the text fields will also work as a defacto dictionary compression method, which will allow for super-fast queries, as well as very efficient storage.
//! 
//! 
//! 
//! 
//! 



use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub (crate) struct Game {
  pub (crate) all_plays: Vec<AllPlays>,
}

#[derive(Debug, Deserialize)]
pub (crate) struct AllPlays {
  pub (crate) result: PlateAppearanceData,
  pub (crate) about: About,
  pub (crate) matchup: MatchupData,
  pub (crate) runners: Vec<RunnerData>,
  //contains pitches, subs, pickoffs, etc.
  #[serde(rename="playEvents")]
  pub (crate) play_events: Vec<PlayEvent>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub (crate) enum PlayEventType  {
    Action,
    Pitch,
    Pickoff,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub (crate) struct PitchDataParse {
    pub (crate) strike_zone_top: f32,
    pub (crate) strike_zone_bottom: f32,
    pub (crate) coordinates: PitchCoordinates,
    pub (crate) start_speed: Option<f32>,
    pub (crate) end_speed: Option<f32>,
    pub (crate) breaks: Option<PitchBreaks>,
    pub (crate) plate_time: Option<f32>,
    pub (crate) extension: Option<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub (crate) struct PitchBreaks {
    pub (crate) break_length: Option<f32>,
    pub (crate) break_y: Option<f32>,
    pub (crate) spin_rate: Option<f32>,
    pub (crate) spin_direction: Option<f32>,
}

#[serde(rename_all="camelCase")]
#[derive(Debug, Deserialize)]
pub (crate) struct PitchCoordinates {
    // x,y are the pixel coordinates on the screen. Mildly useful for minor league strike zone data
    pub (crate) x: Option<f32>,
    pub (crate) y: Option<f32>,
    // The rest of the fields relate to Pitch/Fx or StatCast data
    pub (crate) a_x: Option<f32>,
    pub (crate) a_y: Option<f32>,
    pub (crate) a_z: Option<f32>,
    pub (crate) pfx_x: Option<f32>,
    pub (crate) pfx_z: Option<f32>,
    pub (crate) p_x: Option<f32>,
    pub (crate) p_z: Option<f32>,
    pub (crate) v_x0: Option<f32>,
    pub (crate) v_y0: Option<f32>,
    pub (crate) v_z0: Option<f32>,
    pub (crate) x0: Option<f32>,
    pub (crate) y0: Option<f32>,
    pub (crate) z0: Option<f32>,

}

#[derive(Debug, Deserialize)]
pub (crate) struct HitCoordinates {
    #[serde(alias="coordX")]
    pub (crate) x: Option<f32>,
    #[serde(alias="coordY")]
    pub (crate) y: Option<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub (crate) struct HitData {
    pub (crate) trajectory: Option<Trajectory>,
    pub (crate) hardness: Option<Hardness>,
    pub (crate) coordinates: Option<HitCoordinates>,
    pub (crate) launch_speed: Option<f32>,
    pub (crate) launch_angle: Option<f32>,
    pub (crate) total_distance: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Trajectory {
    #[serde(alias = "line_drive", alias = "bunt_line_drive")]
    LineDrive,
    #[serde(alias = "fly_ball")]
    FlyBall,
    #[serde(alias = "popup", alias = "bunt_popup")]
    PopUp,
    #[serde(alias = "ground_ball", alias="bunt_grounder")]
    GroundBall,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub enum Hardness {
    Soft,
    Medium,
    Hard,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct PitchType {
  pub code: PitchTypeCode,
  pub description: PitchTypeDescription,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PitchTypeCode {
    CH,
    CU,
    EP,
    FA,
    FC,
    FF,
    FO,
    FS,
    FT,
    IN,
    KC,
    KN,
    PO,
    SC,
    SI,
    SL,
    SU,
    FL,
    GY,
    #[serde(other)]
    UN,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="PascalCase")]
pub enum PitchTypeDescription {
    Changeup,
    Curveball,
    Eephus,
    Fastball,
    Cutter,
    #[serde(alias="Four-Seam Fastball")]
    FourSeamFastball,
    Forkball,
    Splitter,
    #[serde(alias="Two-Seam Fastball")]
    TwoSeamFastball,
    #[serde(alias="Int. Ball")]
    IntentionalBall,
    KnuckleCurve,
    Knuckleball,
    Pitchout,
    Screwball,
    Sinker,
    Slider,
    Slurve,
    Slutter,
    Gyroball,
    #[serde(other)]
    Unknown,
}

// The PlayEvent can be either an event or a pitch. This is because non-pitch related things can happen between pitches,
// such as subs, stolen bases, balks, etc. Unfortunately, this creates a less-than ideal initial struct, as we'll need to wrap
// a bunch of fields in Options.
#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub (crate) struct PlayEvent {
    pub (crate) details: PlayEventDetails,
    pub (crate) is_pitch: bool,
    // This index is used to find the matching runner events
    pub (crate) index: u8,
    pub (crate) event: Option<Event>,
    #[serde(rename="type")]
    pub (crate) play_event_type: PlayEventType,
    pub (crate) pitch_data: Option<PitchDataParse>,
    pub (crate) hit_data: Option<HitData>,
    pub (crate) player: Option<PlayerID>,
    pub (crate) position: Option<crate::boxscore::Position>,
    pub (crate) start_time: Option<String>,
}

#[derive(Debug, Deserialize)]
pub (crate) struct PlayerID {
  pub (crate) id: u32,
}

#[derive(Debug, Deserialize)]
pub enum Code {
  ///Pickoff Attempt
  #[serde(rename="1")]
  PO,
  /// Ball in dirt
  #[serde(rename="*B")]
  BD,
  /// Ball
  B,
  /// Called Strike
  C,
  /// Hit Into Play (no outs)
  D,
  /// Hit Into Play (runs)
  E,
  /// Foul Ball
  F,
  /// Strike Swinging
  S,
  /// Strike Foul Bunt
  L,
  /// In Play - Outs
  X,
  #[serde(other)]
  Other,
}

// We ignore the "count" as we'll be computing the state manually
#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub (crate) struct PlayEventDetails {
  pub (crate) event: Option<Event>,
  pub (crate) event_type: Option<EventType>,
  pub (crate) has_review: bool,
  pub (crate) is_in_play: Option<bool>,
  #[serde(rename="type")]
  pub (crate) pitch_type: Option<PitchType>,
  pub (crate) code: Option<Code>,
}

///Result captures plate appearance level details. We ignore the "rbi", "awayscore" and "homescore" fields, as we'll be manually tracking game state,
///including RE24/288, Win Probability and other metadata such as previous pitch. We are ignoring the plate appearance description for performance reasons. 
/// All the relevant data are captured in other data fields.
#[derive(Debug, Deserialize)]
pub(crate) struct PlateAppearanceData {
    #[serde(rename="type")]
    result_type: Option<ResultType>,
    #[serde(rename="event")]
    plate_appearance_result: Option<Event>,
    #[serde(rename="eventType")]
    plate_appearance_result_type: Option<EventType>,
    // #[serde(rename="description")]
    // plate_appearance_result_description: String,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all="camelCase")]
pub enum HalfInning {
    Top,
    Bottom,
}

#[derive(Debug, Deserialize)]
pub(crate) struct About {
    #[serde(rename="atBatIndex")]
    // The record for plate appearances in an extra inning game is 12, since 12x18 is less than u8::max(), we can safely use a u8 here
    pub (crate) plate_appearance_index: u8,
    #[serde(rename="halfInning")]
    pub (crate) half_inning: HalfInning,
    #[serde(rename="inning")]
    pub (crate) inning_num: u8
}

#[derive(Debug, Deserialize)]
#[serde(field_identifier, rename_all="camelCase")]
pub (crate) enum ResultType {
    AtBat,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub(crate) struct Matchup {
    pub(crate) batter: Player,
    pub(crate) pitcher: Player,
    pub(crate) bat_side: Side,
    pub(crate) pitch_hand: Side,
}

#[derive(Debug, Deserialize)]
pub (crate) struct Player {
    id: u32,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub(crate) struct Side {
    code: SideCode,
    description: SideDescription,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum SideCode {
    L,
    R,
    S,
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum SideDescription {
    Left,
    Right,
    Switch,
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Runner {
    movement: RunnerMovement,
    details: RunnerDetails
}

#[derive(Debug, Deserialize)]
#[serde(from = "Base")]
pub(crate) struct BaseValue {
    value: u8,
    runs: u8,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RunnerMovement {
    start: BaseValue,
    end: BaseValue,
    #[serde(rename="isOut")]
    is_out: Out,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub(crate) struct RunnerDetails {
    runner: Player,
    event: Event,
    event_type: EventType,
    // movement_reason: RunnerMovementReason,
    rbi: bool,
    earned: bool,
    //This is sometimes -1, need to handle in game.rs
    play_index: i8,
}



#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Base {
    BaseState (String),
    Null,
}

impl From<Out> for u8 {
    fn from (out: Out) -> u8 {
        match out {
            Out::IsOut(true) => 1,
            _ => 0,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Out {
    IsOut (bool),
    Null,
}


/// Converts the runner base value to a base state value to use to calculate RE288.
/// The binary representation of the base translates to the base value. Bases loaded
/// is 111, thus a 7. Bases empty corresponds to 000 = 0.
impl From<Base> for BaseValue {
    fn from(base_state: Base) -> BaseValue {
        match base_state {
            Base::BaseState (base) => match base.as_str() {
                "1B" => BaseValue {value: 0b001, runs: 0}, // a Runner of first hase a base value of 1 for RE288,
                "2B" => BaseValue {value: 0b010, runs: 0}, // a Runner of second hase a base value of 2 for RE288,
                "3B" => BaseValue {value: 0b100, runs: 0}, // a Runner of third hase a base value of 4 for RE288,
                "score" => BaseValue {value: 0b000, runs: 1},
                _ => BaseValue {value: 0b000, runs: 0},
            },
            Base::Null => BaseValue {value: 0b000, runs: 0},
        }
    }
}

/// RunnerData captures all the runner movement for any pitch or action. We ignore the "movementReason" field since
/// we can get better info from the Event/EventType fields. At this point, the base states are flattened and converted
/// to base values.
#[derive(Debug, Deserialize)]
#[serde(from="Runner")]
pub (crate) struct RunnerData {
    pub (crate) runner_id: u32,
    pub (crate) start_base_value: u8,
    pub (crate) end_base_value: u8,
    pub (crate) runs: u8,
    pub (crate) event: Event,
    pub (crate) event_type: EventType,
    pub (crate) rbi: bool,
    pub (crate) earned: bool,
    pub (crate) play_index: i8,
    pub (crate) outs: u8,
}

//TODO We can remove this from and do it at a later stage. TBD
#[derive(Debug, Deserialize)]
#[serde(from="Matchup")]
pub (crate) struct MatchupData {
  pub (crate) batter_id: u32,
  pub (crate) batter_bat_side_code: SideCode,
  pub (crate) batter_bat_side_desc: SideDescription,
  pub (crate) pitcher_id: u32,
  pub (crate) pitcher_pitch_hand_code: SideCode,
  pub (crate) pitcher_pitch_hand_desc: SideDescription,
}

/// Flatten the runner data out into a more efficient and readable structe without all the nesting. 
/// This will be further flattened later in the process. Having intermediate flattening steps is
/// perhaps less efficient than one big one, but it is IMO more readable and makes the logic easier
/// to reason about and troubleshoot.
impl From <Runner> for RunnerData {
    fn from (runner: Runner) -> RunnerData {
        RunnerData {
            runner_id: runner.details.runner.id,
            start_base_value: runner.movement.start.value,
            end_base_value: runner.movement.end.value,
            runs: runner.movement.end.runs,
            event: runner.details.event,
            event_type: runner.details.event_type,
            rbi: runner.details.rbi,
            earned: runner.details.earned,
            play_index: runner.details.play_index,
            outs: runner.movement.is_out.into(),
        }
    }
}

/// Flatten the batter-pitcher matchup data
impl From <Matchup> for MatchupData {
    fn from (matchup: Matchup) -> MatchupData {
        MatchupData {
            batter_id: matchup.batter.id,
            batter_bat_side_code: matchup.bat_side.code,
            batter_bat_side_desc: matchup.bat_side.description,
            pitcher_id: matchup.pitcher.id,
            pitcher_pitch_hand_code: matchup.pitch_hand.code,
            pitcher_pitch_hand_desc: matchup.pitch_hand.description,
        }
    }
}



/// EventType is a slightly more grouped Event. We probably don't need both,
/// but should be fairly cheap since we're storing both as enums. TODO: Clean
/// this up to use proper formatting or merge with Event.
#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize)]
#[serde(field_identifier)]
pub (crate) enum EventType {
  balk,
  fan_interference,
  other_advance,
  pickoff_error_2b,
  stolen_base_3b,
  batter_interference,
  field_error,
  other_out,
  pickoff_error_3b,
  stolen_base_home,
  catcher_interf,
  field_out,
  passed_ball,
  runner_double_play,
  strikeout,
  caught_stealing_2b,
  fielders_choice,
  pickoff_1b,
  runner_interference,
  strikeout_double_play,
  caught_stealing_3b,
  fielders_choice_out,
  pickoff_2b,
  sac_bunt,
  triple,
  caught_stealing_home,
  force_out,
  pickoff_3b,
  sac_bunt_double_play,
  triple_play,
  defensive_indiff,
  grounded_into_double_play,
  pickoff_caught_stealing_2b,
  sac_fly,
  walk,
  double,
  hit_by_pitch,
  pickoff_caught_stealing_3b,
  sac_fly_double_play,
  wild_pitch,
  double_play,
  home_run,
  pickoff_caught_stealing_home,
  single,
  error,
  intent_walk,
  pickoff_error_1b,
  stolen_base_2b,
  defensive_substitution,
  offensive_substitution,
  pitching_substitution,
  defensive_switch,
  ejection,
  game_advisory,
  #[serde(other)]
  other,
}


/// Event stores all the possible events. Wherever possible, we'll convert text
/// into enums, avoiding lifetime issues and increasing memory efficiency. Serde does all the heavy lifting in the
/// background. TODO: use this for both the "event" and "eventType" fields to see where there are differences
#[derive(Debug, Deserialize)]
#[serde(field_identifier)]
pub (crate) enum Event {
    #[serde(alias = "Game Advisory")]
    GameAdvisory,
    #[serde(alias = "Ejection")]
    Ejection,
    #[serde(alias = "Batter Interference")]
    BatterInterference,
    #[serde(alias = "Bunt Ground Out", alias = "Bunt Groundout")]
    BuntGroundOut,
    #[serde(alias = "Bunt Pop Out", alias = "Bunt Popout")]
    BuntPopOut,
    #[serde(alias = "Catcher Interference")]
    CatcherInterference,
    Double,
    #[serde(alias = "Double Play", alias = "Grounded Into DP", alias = "Lined Into DP")]
    DoublePlay,
    #[serde(alias = "Fan Interference")]
    FanInterference,
    #[serde(alias = "Field Error", alias = "Error")]
    FieldError,
    #[serde(alias = "Field Out", alias="Fieldout")]
    FieldOut,
    #[serde(alias = "Fielders Choice", alias = "Fielders Choice Out")]
    FieldersChoice,
    #[serde(alias = "Fly Out", alias = "Flyout")]
    FlyOut,
    #[serde(alias = "Force Out", alias = "Forceout")]
    ForceOut,
    #[serde(alias = "Ground Out", alias = "Groundout")]
    GroundOut,
    #[serde(alias = "Grounded into DP")]
    GroundedIntoDoublePlay,
    #[serde(alias = "Hit By Pitch")]
    HitByPitch,
    #[serde(alias = "Home Run")]
    HomeRun,
    #[serde(alias = "Intent Walk", alias = "Intentional Walk")]
    IntentionalWalk,
    #[serde(alias = "Line Out", alias="Lineout")]
    LineOut,
    #[serde(alias = "Pop Out", alias = "Popout")]
    PopOut,
    #[serde(alias = "Runner Out")]
    RunnerOut,
    #[serde(alias = "Sac Bunt")]
    SacBunt,
    #[serde(alias = "Sac Fly")]
    SacFly,
    #[serde(alias = "Sac Fly DP", alias = "Sac Fly Double Play")]
    SacFlyDoublePlay,
    #[serde(alias = "Sacrifice Bunt DP")]
    SacrificeBuntDoublePlay,
    Single,
    #[serde(alias = "Strikeout", alias = "Strikeout Double Play", alias = "Strikeout - DP", alias = "Strikeout - TP")]
    StrikeOut,
    Triple,
    #[serde(alias = "Triple Play", alias = "triple_play")]
    TriplePlay,
    #[serde(alias = "Walk")]
    Walk,
    #[serde(alias = "Balk")]
    Balk,
    #[serde(alias = "Stolen Base 2B", alias = "Stolen Base 3B", alias = "Stolen Base Home", alias = "stolen_base_2B", alias = "stolen_base_3B", alias = "stolen_base_home")]
    StolenBase,
    #[serde(alias = "Pickoff Error 1B", alias = "Pickoff Error 2B", alias = "Pickoff Error 3B")]
    PickOffError,
    #[serde(alias = "Pickoff 1B", alias = "Pickoff 2B", alias = "Pickoff 3B")]
    PickOff,
    #[serde(alias = "Caught Stealing 1B", alias = "Caught Stealing 2B", alias = "Caught Stealing 3B", alias = "Caught Stealing Home")]
    CaughtStealing,
    #[serde(alias = "Pickoff Caught Stealing 1B", alias = "Pickoff Caught Stealing 2B", alias = "Pickoff Caught Stealing 3B", alias = "Pickoff Caught Stealing Home")]
    PickoffCaughtStealing,
    #[serde(alias = "Wild Pitch", alias = "wild_pitch")]
    WildPitch,
    #[serde(alias = "Passed Ball", alias = "passed_ball")]
    PassedBall,
    #[serde(alias="Pitching Substitution", alias="pitching_substitution")]
    PitchingSubstitution,
    #[serde(alias="Defensive Sub", alias="defensive_substitution")]
    DefensiveSubstitution,
    #[serde(alias="Defensive Switch", alias="defensive_switch")]
    DefensiveSwitch,
    #[serde(alias="Offensive Substitution", alias="offensive_substitution")]
    OffensiveSubstitution,
    #[serde(alias="Defensive Indiff", alias="Defensive Indifference")]
    DefensiveIndifference,
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub (crate) struct PlateAppearance {
    #[serde(rename="type")]
    result_type: ResultType,
    event: Event,
    event_type: EventType,
}

