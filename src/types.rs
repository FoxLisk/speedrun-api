mod category;
mod common;
mod games;
mod guests;
mod leaderboards;
mod levels;
mod platforms;
mod regions;
mod runs;
mod series;
mod variables;

// TODO: Deserialize to URI/URL type
// TODO: Deserialize dates to chrono types

pub use category::{Category, CategoryType, Players};
pub use common::{Asset, Assets, Link, ModeratorRole, Pagination, TimingMethod};
pub use games::{Game, Names, Ruleset};
pub use guests::Guest;
pub use leaderboards::{Leaderboard, RankedRun};
pub use levels::Level;
pub use platforms::Platform;
pub use regions::Region;
pub use runs::{Player, Run, Status, System, Times, VideoLink, Videos};
pub use series::Series;
pub use variables::{Flags, Scope, Value, Values, Variable};
