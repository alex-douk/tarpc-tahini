use alohomora::policy::{FrontendPolicy, Policy};
use services_utils::policies::PromptPolicy;
use services_utils::policies::shared_policies::UsernamePolicy;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct LocalInferencePolicy {
    pub no_storage: bool,
    pub marketing_consent: bool,
    pub unprotected_image_gen: bool,
}

#[derive(Debug, Clone)]
pub struct LocalUserNamePolicy {}

impl Policy for LocalInferencePolicy {
    fn name(&self) -> String {
        "LocalInferencePolicy".to_string()
    }

    fn check(
        &self,
        context: &alohomora::context::UnprotectedContext,
        reason: alohomora::policy::Reason<'_>,
    ) -> bool {
        false
    }

    fn join(
        &self,
        other: alohomora::policy::AnyPolicy,
    ) -> Result<alohomora::policy::AnyPolicy, ()> {
        Ok(other)
    }

    fn join_logic(&self, other: Self) -> Result<Self, ()>
    where
        Self: Sized,
    {
        Ok(other)
    }
}

impl FrontendPolicy for LocalInferencePolicy {
    fn from_request<'a, 'r>(request: &'a rocket::Request<'r>) -> Self
    where
        Self: Sized,
    {
        let no_storage =
            bool::from_str(request.cookies().get("no_storage").unwrap().value()).unwrap();
        let marketing_consent =
            bool::from_str(request.cookies().get("ads").unwrap().value()).unwrap();
        let unprotected_image_gen =
            bool::from_str(request.cookies().get("image_gen").unwrap().value()).unwrap();
        LocalInferencePolicy {
            no_storage,
            marketing_consent,
            unprotected_image_gen,
        }
    }

    fn from_cookie<'a, 'r>(
        _name: &str,
        _cookie: &'a rocket::http::Cookie<'static>,
        request: &'a rocket::Request<'r>,
    ) -> Self
    where
        Self: Sized,
    {
        let no_storage =
            bool::from_str(request.cookies().get("no_storage").unwrap().value()).unwrap();
        let marketing_consent =
            bool::from_str(request.cookies().get("ads").unwrap().value()).unwrap();
        let unprotected_image_gen =
            bool::from_str(request.cookies().get("image_gen").unwrap().value()).unwrap();
        LocalInferencePolicy {
            no_storage,
            marketing_consent,
            unprotected_image_gen,
        }
    }
}

impl Into<PromptPolicy> for LocalInferencePolicy {
    fn into(self) -> PromptPolicy {
        PromptPolicy {
            no_storage: self.no_storage,
            marketing_consent: self.marketing_consent,
            unprotected_image_gen: self.unprotected_image_gen,
        }
    }
}

impl Policy for LocalUserNamePolicy {
    fn name(&self) -> String {
        "LocalUserNamePolicy".to_string()
    }
    fn check(
        &self,
        context: &alohomora::context::UnprotectedContext,
        reason: alohomora::policy::Reason<'_>,
    ) -> bool {
        false
    }

    fn join(
        &self,
        other: alohomora::policy::AnyPolicy,
    ) -> Result<alohomora::policy::AnyPolicy, ()> {
        Ok(self.clone().into_any())
    }

    fn join_logic(&self, other: Self) -> Result<Self, ()>
    where
        Self: Sized,
    {
        Ok(other)
    }
}

impl Into<UsernamePolicy> for LocalUserNamePolicy {
    fn into(self) -> UsernamePolicy {
        UsernamePolicy {}
    }
}

impl FrontendPolicy for LocalUserNamePolicy {
    fn from_cookie<'a, 'r>(
        name: &str,
        cookie: &'a rocket::http::Cookie<'static>,
        request: &'a rocket::Request<'r>,
    ) -> Self
    where
        Self: Sized,
    {
        Self {}
    }
    fn from_request<'a, 'r>(request: &'a rocket::Request<'r>) -> Self
    where
        Self: Sized,
    {
        Self {}
    }
}
