use fluvio_connector_common::connector;

#[connector(config)]
#[derive(Debug)]
pub(crate) struct ElasticSearchConfig {
    pub index: String,
    pub username: String,
    pub password: String,
    pub url: String,
}
