use alohomora::policy::{Policy, Reason};
use tarpc::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct PromptPolicy {
    pub no_storage: bool,
    pub marketing_consent: bool,
    pub unprotected_image_gen: bool
}

impl Policy for PromptPolicy {
    fn name(&self) -> String {
        "PromptPolicy".to_string()
    }

    fn check(&self, context: &alohomora::context::UnprotectedContext, reason: alohomora::policy::Reason<'_>) -> bool {
        match reason {
            Reason::DB(_, _) => !self.no_storage,
            Reason::Response => true,
            //If we have a custom  reason, it needs to be an inference reason
            Reason::Custom(reason) => match reason.cast().downcast_ref::<InferenceReason>() {
                None => false,
                Some(reason) => match reason {
                    //If it is, we check the inference reason
                    InferenceReason::SendToMarketing => self.marketing_consent,
                    InferenceReason::SendToImageGen => self.unprotected_image_gen,
                    InferenceReason::SendToDB => !self.no_storage
                }
            }
            //If it is not as a direct query response, a DB request, or an inference specific
            //purpose, we deny
            _ => false
        }
    }

    fn join(&self, other: alohomora::policy::AnyPolicy) -> Result<alohomora::policy::AnyPolicy, ()> {
        Ok(other)
    }

    fn join_logic(&self, other: Self) -> Result<Self, ()> where Self: Sized {
        Ok(self.clone())
    }
}

#[derive(Clone)]
pub enum InferenceReason {
    SendToMarketing,
    SendToImageGen,
    SendToDB
}

