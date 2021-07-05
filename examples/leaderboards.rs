use speedrun_api::{
    api::{
        AsyncQuery,
        leaderboards::{FullGameLeaderboard, IndividualLevelLeaderboard},
    },
    error::SpeedrunApiResult,
    SpeedrunApiBuilder,
    types,
    types::TimingMethod,
};

#[tokio::main]
pub async fn main() -> SpeedrunApiResult<()> {
    env_logger::init();

    let client = SpeedrunApiBuilder::new().build_async().await?;

    let endpoint = FullGameLeaderboard::builder()
        .game("xldev513")
        .category("rklg3rdn")
        .build()
        .unwrap();
    let leaderboard: types::Leaderboard = endpoint.query_async(&client).await?;
    println!("{:#?}", leaderboard);

    let endpoint = FullGameLeaderboard::builder()
        .game("n4d7jzd7")
        .category("w20p7jkn")
        .timing(TimingMethod::Realtime)
        .build()
        .unwrap();
    let leaderboard: types::Leaderboard = endpoint.query_async(&client).await?;
    println!("{:#?}", leaderboard);

    // This game/level/category combonation does not exist, however this is the
    // example used by the API documentation.
    let endpoint = IndividualLevelLeaderboard::builder()
        .game("xldev513")
        .level("rdqz4kdx")
        .category("xk9le4k0")
        .build()
        .unwrap();
    let leaderboard: types::Leaderboard = endpoint.query_async(&client).await?;
    println!("{:#?}", leaderboard);

    Ok(())
}
