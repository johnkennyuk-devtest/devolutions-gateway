use crate::utils::TargetAddr;
use uuid::Uuid;
use zeroize::Zeroize;

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
pub enum JetAccessTokenClaims {
    Association(JetAssociationTokenClaims),
    Scope(JetScopeTokenClaims),
    Bridge(JetBridgeTokenClaims),
    Jmux(JetJmuxTokenClaims),
}

#[derive(Deserialize, Clone)]
pub struct JetAssociationTokenClaims {
    /// Jet Association ID (= Session ID)
    #[serde(default = "Uuid::new_v4")] // legacy: DVLS up to 2021.2.10 do not generate this claim.
    pub jet_aid: Uuid,

    /// Jet Application protocol
    pub jet_ap: ApplicationProtocol,

    /// Jet Connection Mode
    #[serde(flatten)]
    pub jet_cm: ConnectionMode,

    /// Jet Recording Policy
    #[serde(default)]
    pub jet_rec: bool,

    /// Jet Filtering Policy
    #[serde(default)]
    pub jet_flt: bool,
}

impl JetAssociationTokenClaims {
    pub fn contains_secrets(&self) -> bool {
        matches!(&self.jet_cm, ConnectionMode::Fwd { creds: Some(_), .. })
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ApplicationProtocol {
    Wayk,
    Pwsh,
    Rdp,
    Ard,
    Ssh,
    Sftp,
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "jet_cm")]
#[allow(clippy::large_enum_variant)]
pub enum ConnectionMode {
    /// Connection should be processed following the rendez-vous protocol
    Rdv,

    /// Connection should be forwared to a given destination host
    Fwd {
        /// Destination Host
        dst_hst: TargetAddr,

        /// Alternate Destination Hosts
        #[serde(default)]
        dst_alt: Vec<TargetAddr>,

        /// Credentials to use if protocol is wrapped by the Gateway (e.g. RDP TLS)
        #[serde(flatten)]
        creds: Option<CredsClaims>,
    },
}

#[derive(Deserialize, Zeroize, Clone)]
#[zeroize(drop)]
pub struct CredsClaims {
    // Proxy credentials (client ↔ jet)
    pub prx_usr: String,
    pub prx_pwd: String,

    // Target credentials (jet ↔ server)
    pub dst_usr: String,
    pub dst_pwd: String,
}

#[derive(Clone, Deserialize)]
pub struct JetScopeTokenClaims {
    pub scope: JetAccessScope,
}

#[derive(Clone, Deserialize, PartialEq)]
pub enum JetAccessScope {
    #[serde(rename = "gateway.sessions.read")]
    GatewaySessionsRead,
    #[serde(rename = "gateway.associations.read")]
    GatewayAssociationsRead,
    #[serde(rename = "gateway.diagnostics.read")]
    GatewayDiagnosticsRead,
}

#[derive(Clone, Deserialize)]
pub struct JetBridgeTokenClaims {
    pub target_host: TargetAddr,
}

#[derive(Clone, Deserialize)]
pub struct JetJmuxTokenClaims {
    filtering: Option<()>, // TODO
}