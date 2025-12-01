const SESSION_KEY: &str = "SESSION";
const USERAGENT_KEY: &str = "USERAGENT";
const YEAR_KEY: &str = "YEAR";

pub fn get_user_agent() -> String {
    dotenv::var(USERAGENT_KEY)
    .expect(
        format!(
            "Please provide a .env with the {} variable, preferably the public URL to your git repository",
            USERAGENT_KEY
        ).as_str(),
    )
    .to_owned()
}

pub fn get_session_token() -> String {
    dotenv::var(SESSION_KEY)
    .expect(
        format!(
            "Please provide a .env with the {} variable (obtained from your cookie's 'session' value)",
            SESSION_KEY
        ).as_str(),
    )
    .to_owned()
}

pub fn get_year() -> String {
    dotenv::var(YEAR_KEY)
        .expect(
            format!(
                "Please provide a .env with the {} variable set to the current AOC year",
                YEAR_KEY
            )
            .as_str(),
        )
        .to_owned()
}
