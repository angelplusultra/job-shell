use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::{
    error::AppResult,
    handlers::scrape_options::{
        ANDURIL_SCRAPE_OPTIONS, DISCORD_SCRAPE_OPTIONS, GITHUB_SCRAPE_OPTIONS,
        GITLAB_SCRAPE_OPTIONS, ONEPASSWORD_SCRAPE_OPTIONS, PALANTIR_DEFAULT_SCRAPE_OPTIONS,
        THE_BROWSER_COMPANY_DEFAULT_SCRAPE_OPTIONS, WEEDMAPS_SCRAPE_OPTIONS,
    },
    models::{data::Data, scraper::JobsPayload},
    scrapers::{
        airbnb::scraper::scrape_airbnb, atlassian::scraper::scrape_atlassian,
        blizzard::scraper::scrape_blizzard, chase::scraper::scrape_chase,
        cisco::scraper::scrape_cisco, cloudflare::scraper::scrape_cloudflare,
        coinbase::scraper::scrape_coinbase, costar_group::scraper::scrape_costar_group,
        default::default_scrape_jobs_handler, disney::scraper::scrape_disney,
        doordash::scraper::scrape_doordash, experian::scraper::scrape_experian,
        gen::scraper::scrape_gen, ibm::scraper::scrape_ibm, meta::scraper::scrape_meta,
        netflix::scraper::scrape_netflix, nike::scraper::scrape_nike,
        panasonic::scraper::scrape_panasonic, paypal::scraper::scrape_paypal,
        reddit::scraper::scrape_reddit, robinhood::scraper::scrape_robinhood,
        salesforce::scraper::scrape_salesforce, servicenow::scraper::scrape_servicenow,
        square::scraper::scrape_square, stripe::scraper::scrape_stripe,
        toast::scraper::scrape_toast, uber::scraper::scrape_uber,
    },
};

#[derive(EnumIter, Debug, Display)]
pub enum CompanyOption {
    #[strum(to_string = "AirBnB")]
    AirBnb,

    #[strum(to_string = "Atlassian")]
    Atlassian,

    #[strum(to_string = "Anduril")]
    Anduril,

    #[strum(to_string = "Blizzard")]
    Blizzard,

    #[strum(to_string = "The Browser Company")]
    TheBrowserCompany,

    #[strum(to_string = "Cisco")]
    Cisco,

    #[strum(to_string = "Cloudflare")]
    Cloudflare,

    #[strum(to_string = "Costar Group")]
    CostarGroup,

    #[strum(to_string = "Experian")]
    Experian,

    #[strum(to_string = "GitHub")]
    GitHub,

    #[strum(to_string = "GitLab")]
    GitLab,

    #[strum(to_string = "Discord")]
    Discord,

    #[strum(to_string = "1Password")]
    OnePassword,

    #[strum(to_string = "Salesforce")]
    Salesforce,

    #[strum(to_string = "Stripe")]
    Stripe,

    #[strum(to_string = "IBM")]
    IBM,

    #[strum(to_string = "Nike")]
    Nike,

    #[strum(to_string = "Toast")]
    Toast,

    #[strum(to_string = "Uber")]
    Uber,

    #[strum(to_string = "DoorDash")]
    DoorDash,

    #[strum(to_string = "PayPal")]
    PayPal,

    #[strum(to_string = "Panasonic")]
    Panasonic,

    #[strum(to_string = "Palantir")]
    Palantir,

    #[strum(to_string = "ServiceNow")]
    ServiceNow,

    #[strum(to_string = "Robinhood")]
    Robinhood,

    #[strum(to_string = "Square")]
    Square,

    #[strum(to_string = "Reddit")]
    Reddit,

    #[strum(to_string = "Meta")]
    Meta,

    #[strum(to_string = "Netflix")]
    Netflix,

    #[strum(to_string = "Chase")]
    Chase,

    #[strum(to_string = "Coinbase")]
    Coinbase,

    #[strum(to_string = "Disney")]
    Disney,

    #[strum(to_string = "Gen")]
    Gen,

    #[strum(to_string = "Weedmaps")]
    Weedmaps,
}

impl CompanyOption {
    pub fn keys() -> Vec<String> {
        let mut company_keys: Vec<String> = CompanyOption::iter().map(|x| x.to_string()).collect();

        company_keys.sort();

        company_keys
    }
}
pub trait ScrapeJobs {
    async fn scrape_jobs(&self, data: &mut Data) -> AppResult<JobsPayload>;
}
impl ScrapeJobs for CompanyOption {
    async fn scrape_jobs(&self, data: &mut Data) -> AppResult<JobsPayload> {
        match self {
            Self::AirBnb => scrape_airbnb(data).await,
            Self::Atlassian => scrape_atlassian(data).await,
            Self::Blizzard => scrape_blizzard(data).await,
            Self::Cisco => scrape_cisco(data).await,
            Self::Cloudflare => scrape_cloudflare(data).await,
            Self::CostarGroup => scrape_costar_group(data).await,
            Self::Chase => scrape_chase(data).await,
            Self::Coinbase => scrape_coinbase(data).await,
            Self::Disney => scrape_disney(data).await,
            Self::Gen => scrape_gen(data).await,
            Self::Meta => scrape_meta(data).await,
            Self::Netflix => scrape_netflix(data).await,
            Self::Stripe => scrape_stripe(data).await,
            Self::Salesforce => scrape_salesforce(data).await,
            Self::Experian => scrape_experian(data).await,
            Self::Nike => scrape_nike(data).await,
            Self::ServiceNow => scrape_servicenow(data).await,
            Self::Robinhood => scrape_robinhood(data).await,
            Self::Square => scrape_square(data).await,
            Self::IBM => scrape_ibm(data).await,
            Self::Reddit => scrape_reddit(data).await,
            Self::Uber => scrape_uber(data).await,
            Self::DoorDash => scrape_doordash(data).await,
            Self::PayPal => scrape_paypal(data).await,
            Self::Toast => scrape_toast(data).await,
            Self::Weedmaps => default_scrape_jobs_handler(data, WEEDMAPS_SCRAPE_OPTIONS).await,
            Self::TheBrowserCompany => {
                default_scrape_jobs_handler(data, THE_BROWSER_COMPANY_DEFAULT_SCRAPE_OPTIONS).await
            }
            Self::OnePassword => {
                default_scrape_jobs_handler(data, ONEPASSWORD_SCRAPE_OPTIONS).await
            }
            Self::GitHub => default_scrape_jobs_handler(data, GITHUB_SCRAPE_OPTIONS).await,
            Self::GitLab => default_scrape_jobs_handler(data, GITLAB_SCRAPE_OPTIONS).await,
            Self::Discord => default_scrape_jobs_handler(data, DISCORD_SCRAPE_OPTIONS).await,
            Self::Palantir => {
                default_scrape_jobs_handler(data, PALANTIR_DEFAULT_SCRAPE_OPTIONS).await
            }
            Self::Panasonic => scrape_panasonic(data).await,
            Self::Anduril => default_scrape_jobs_handler(data, ANDURIL_SCRAPE_OPTIONS).await,
        }
    }
}
