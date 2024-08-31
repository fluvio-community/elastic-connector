use fluvio_connector_common::connector;
use fluvio_connector_common::secret::SecretString;

#[connector(config)]
#[derive(Debug)]
pub(crate) struct ElasticSearchConfig {
    pub index: String,
    pub url: String,
    pub(crate) username: SecretString,
    pub(crate) password: SecretString,
    pub(crate) cloud_id: Option<SecretString>,
    
}
