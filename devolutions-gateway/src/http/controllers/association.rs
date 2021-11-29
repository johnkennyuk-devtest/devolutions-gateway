use crate::config::Config;
use crate::http::guards::access::{AccessGuard, JetTokenType};
use crate::jet::association::{Association, AssociationResponse};
use crate::jet::candidate::Candidate;
use crate::jet_client::JetAssociationsMap;
use crate::token::{ConnectionMode, JetAccessScope, JetAccessTokenClaims};
use crate::utils::association::{remove_jet_association, ACCEPT_REQUEST_TIMEOUT};
use jet_proto::JET_VERSION_V2;
use saphir::controller::Controller;
use saphir::http::{Method, StatusCode};
use saphir::macros::controller;
use saphir::request::Request;
use slog_scope::info;
use std::sync::Arc;
use tokio::runtime::Handle;
use uuid::Uuid;

pub struct AssociationController {
    config: Arc<Config>,
    jet_associations: JetAssociationsMap,
}

impl AssociationController {
    pub fn new(config: Arc<Config>, jet_associations: JetAssociationsMap) -> Self {
        Self {
            config,
            jet_associations,
        }
    }
}

#[controller(name = "jet/association")]
impl AssociationController {
    #[get("/")]
    #[guard(
        AccessGuard,
        init_expr = r#"JetTokenType::Scope(JetAccessScope::GatewayAssociationsRead)"#
    )]
    async fn get_associations(&self, detail: Option<bool>) -> (StatusCode, Option<String>) {
        let with_detail = detail.unwrap_or(false);
        let associations_response: Vec<AssociationResponse>;
        let associations = self.jet_associations.lock().await;

        associations_response = associations
            .values()
            .map(|association| AssociationResponse::from(association, with_detail))
            .collect();

        if let Ok(body) = serde_json::to_string(&associations_response) {
            return (StatusCode::OK, Some(body));
        }

        (StatusCode::BAD_REQUEST, None)
    }

    #[post("/<association_id>")]
    #[guard(AccessGuard, init_expr = r#"JetTokenType::Association"#)]
    async fn create_association(&self, mut req: Request) -> StatusCode {
        if let Some(JetAccessTokenClaims::Association(association_claims)) =
            req.extensions_mut().remove::<JetAccessTokenClaims>()
        {
            let association_id = match req
                .captures()
                .get("association_id")
                .and_then(|id| Uuid::parse_str(id).ok())
            {
                Some(id) => id,
                None => {
                    return StatusCode::BAD_REQUEST;
                }
            };

            match association_claims.jet_cm {
                ConnectionMode::Rdv if association_claims.jet_aid == association_id => {}
                _ => {
                    slog_scope::error!(
                        "Invalid session token: expected rendezvous token for {}",
                        association_id
                    );
                    return StatusCode::FORBIDDEN;
                }
            }

            let mut jet_associations = self.jet_associations.lock().await;

            jet_associations.insert(
                association_id,
                Association::new(association_id, JET_VERSION_V2, association_claims),
            );
            start_remove_association_future(self.jet_associations.clone(), association_id);

            StatusCode::OK
        } else {
            StatusCode::UNAUTHORIZED
        }
    }

    #[post("/<association_id>/candidates")]
    #[guard(AccessGuard, init_expr = r#"JetTokenType::Association"#)]
    async fn gather_association_candidates(&self, mut req: Request) -> (StatusCode, Option<String>) {
        if let Some(JetAccessTokenClaims::Association(association_claims)) =
            req.extensions_mut().remove::<JetAccessTokenClaims>()
        {
            let association_id = match req
                .captures()
                .get("association_id")
                .and_then(|id| Uuid::parse_str(id).ok())
            {
                Some(id) => id,
                None => return (StatusCode::BAD_REQUEST, None),
            };

            match association_claims.jet_cm {
                ConnectionMode::Rdv if association_claims.jet_aid == association_id => {}
                _ => {
                    slog_scope::error!(
                        "Invalid session token: expected rendezvous token for {}",
                        association_id
                    );
                    return (StatusCode::FORBIDDEN, None);
                }
            }

            // create association if needed

            let mut jet_associations = self.jet_associations.lock().await;

            let association = if let Some(association) = jet_associations.get_mut(&association_id) {
                association
            } else {
                slog_scope::error!("Association {} not found", association_id);
                return (StatusCode::INTERNAL_SERVER_ERROR, None);
            };

            if association.get_candidates().is_empty() {
                for listener in &self.config.listeners {
                    if let Some(candidate) = Candidate::new(listener.external_url.to_string().trim_end_matches('/')) {
                        association.add_candidate(candidate);
                    }
                }
            }

            (StatusCode::OK, Some(association.gather_candidate().to_string()))
        } else {
            (StatusCode::UNAUTHORIZED, None)
        }
    }
}

pub fn start_remove_association_future(jet_associations: JetAssociationsMap, uuid: Uuid) {
    if let Ok(runtime_handle) = Handle::try_current() {
        runtime_handle.spawn(async move {
            tokio::time::sleep(ACCEPT_REQUEST_TIMEOUT).await;
            if remove_jet_association(jet_associations, uuid, None).await {
                info!(
                    "No connect request received with association {}. Association removed!",
                    uuid
                );
            }
        });
    }
}