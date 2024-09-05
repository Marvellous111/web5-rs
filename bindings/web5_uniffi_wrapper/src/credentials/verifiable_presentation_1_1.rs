use crate::{dids::bearer_did::BearerDid, errors::Result};
use std::{sync::Arc, time::SystemTime};
use web5::credentials::VerifiablePresentation as InnerVerifiablePresentation;
use web5::credentials::VerifiablePresentationCreateOptions as InnerVerifiablePresentationCreateOptions;

#[derive(Default)]
pub struct VerifiablePresentationCreateOptions {
    pub id: Option<String>,
    pub context: Option<Vec<String>>,
    pub r#type: Option<Vec<String>>,
    pub issuance_date: Option<SystemTime>,
    pub expiration_date: Option<SystemTime>,
}

pub struct VerifiablePresentation {
    pub inner_vp: InnerVerifiablePresentation,
}

impl VerifiablePresentation {
    pub fn create(
        holder: String,
        vc_jwts: Vec<String>,
        options: Option<VerifiablePresentationCreateOptions>,
    ) -> Result<Self> {
        let options = options.unwrap_or_default();
        let inner_options = InnerVerifiablePresentationCreateOptions {
            id: options.id,
            context: options.context,
            r#type: options.r#type,
            issuance_date: options.issuance_date,
            expiration_date: options.expiration_date,
        };

        let inner_vp = InnerVerifiablePresentation::create(holder, vc_jwts, Some(inner_options))?;

        Ok(Self { inner_vp })
    }

    pub fn get_data(&self) -> Result<VerifiablePresentationData> {
        Ok(VerifiablePresentationData {
            context: self.inner_vp.context.clone(),
            id: self.inner_vp.id.clone(),
            r#type: self.inner_vp.r#type.clone(),
            holder: self.inner_vp.holder.clone(),
            verifiable_credential: self.inner_vp.verifiable_credential.clone(),
            issuance_date: self.inner_vp.issuance_date,
            expiration_date: self.inner_vp.expiration_date,
        })
    }

    pub fn from_vp_jwt(vp_jwt: String, verify: bool) -> Result<Self> {
        let inner_vp = InnerVerifiablePresentation::from_vp_jwt(&vp_jwt, verify)?;

        Ok(Self { inner_vp })
    }

    pub fn sign(
        &self,
        bearer_did: Arc<BearerDid>,
        verification_method_id: Option<String>,
    ) -> Result<String> {
        let vp_jwt = self.inner_vp.sign(&bearer_did.0, verification_method_id)?;
        Ok(vp_jwt)
    }
}

#[derive(Clone)]
pub struct VerifiablePresentationData {
    pub context: Vec<String>,
    pub id: String,
    pub r#type: Vec<String>,
    pub holder: String,
    pub issuance_date: SystemTime,
    pub expiration_date: Option<SystemTime>,
    pub verifiable_credential: Vec<String>,
}