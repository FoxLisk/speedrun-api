use std::{borrow::Cow, collections::HashMap};

use http::Method;
use serde::Serialize;

use super::{endpoint::Endpoint, Direction, Pageable};

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Runs<'a> {
    user: Option<Cow<'a, str>>,
    guest: Option<Cow<'a, str>>,
    examiner: Option<Cow<'a, str>>,
    game: Option<Cow<'a, str>>,
    level: Option<Cow<'a, str>>,
    category: Option<Cow<'a, str>>,
    platform: Option<Cow<'a, str>>,
    region: Option<Cow<'a, str>>,
    emulated: Option<bool>,
    status: Option<RunStatus>,
    orderby: Option<RunsSorting>,
    direction: Option<Direction>,
}

impl<'a> Runs<'a> {
    pub fn builder() -> RunsBuilder<'a> {
        RunsBuilder::default()
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum RunStatus {
    New,
    Verified,
    Rejected,
}

/// Sorting options for runs
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum RunsSorting {
    /// Sorts by the game the run was done in (default)
    Game,
    /// Sorts by the run category
    Category,
    /// Sorts by the run level
    Level,
    /// Sorts by the platform used for the run
    Platform,
    /// Sorts by the console region used for the run
    Region,
    /// Sorts by whether an emulator was used for the run
    Emulated,
    /// Sorts by the date of the run
    Date,
    /// Sorts by the date when the run was submitted to speedrun.com
    Submitted,
    /// Sorts by verification status
    Status,
    /// Sorts by the date the run was verified
    VerifyDate,
}

impl Default for RunsSorting {
    fn default() -> Self {
        Self::Game
    }
}

impl Endpoint for Runs<'_> {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "/runs".into()
    }

    fn query_parameters(&self) -> Result<Cow<'static, str>, super::error::BodyError> {
        Ok(serde_urlencoded::to_string(self)?.into())
    }
}

impl Pageable for Runs<'_> {}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct Run<'a> {
    id: Cow<'a, str>,
}

impl<'a> Run<'a> {
    pub fn builder() -> RunBuilder<'a> {
        RunBuilder::default()
    }
}

impl Endpoint for Run<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/runs/{}", self.id).into()
    }
}

#[derive(Default, Debug, Builder, Serialize, Clone)]
#[builder(default, setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct CreateRun<'a> {
    category: Cow<'a, str>,
    level: Option<Cow<'a, str>>,
    date: Option<Cow<'a, str>>,
    region: Option<Cow<'a, str>>,
    platform: Option<Cow<'a, str>>,
    verified: Option<bool>,
    times: Times,
    players: Option<Vec<Player>>,
    emulated: Option<bool>,
    video: Option<url::Url>,
    comment: Option<String>,
    splitsio: Option<SplitsIo>,
    variables: HashMap<String, VariableType>,
}

impl<'a> CreateRun<'a> {
    pub fn buider() -> CreateRunBuilder<'a> {
        CreateRunBuilder::default()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "rel")]
pub enum Player {
    User { id: String },
    Guest { name: String },
}

#[derive(Default, Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Times {
    realtime: Option<f64>,
    realtime_noloads: Option<f64>,
    ingame: Option<f64>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(untagged)]
pub enum SplitsIo {
    Id(String),
    Url(url::Url),
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum VariableType {
    PreDefined { value: String },
    UserDefined { value: String },
}

impl Endpoint for CreateRun<'_> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "/runs".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, super::error::BodyError> {
        Ok(serde_json::to_vec(self).map(|body| Some(("application/json", body)))?)
    }
}

#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct UpdateRunStatus<'a> {
    #[serde(skip)]
    id: Cow<'a, str>,
    status: NewStatus,
}

impl<'a> UpdateRunStatus<'a> {
    pub fn builder() -> UpdateRunStatusBuilder<'a> {
        UpdateRunStatusBuilder::default()
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "status")]
pub enum NewStatus {
    Verified,
    Rejected { reason: String },
}

impl Endpoint for UpdateRunStatus<'_> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/runs/{}/status", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, super::error::BodyError> {
        Ok(serde_json::to_vec(self).map(|body| Some(("application/json", body)))?)
    }
}

#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct UpdateRunPlayers<'a> {
    #[serde(skip)]
    id: Cow<'a, str>,
    players: Vec<Player>,
}

impl<'a> UpdateRunPlayers<'a> {
    pub fn builder() -> UpdateRunPlayersBuilder<'a> {
        UpdateRunPlayersBuilder::default()
    }
}

impl Endpoint for UpdateRunPlayers<'_> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/runs/{}/players", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, super::error::BodyError> {
        Ok(serde_json::to_vec(self).map(|body| Some(("application/json", body)))?)
    }
}

#[derive(Debug, Builder, Serialize, Clone)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "kebab-case")]
pub struct DeleteRun<'a> {
    id: Cow<'a, str>,
}

impl<'a> DeleteRun<'a> {
    pub fn builder() -> DeleteRunBuilder<'a> {
        DeleteRunBuilder::default()
    }
}

impl Endpoint for DeleteRun<'_> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/runs/{}", self.id).into()
    }
}
