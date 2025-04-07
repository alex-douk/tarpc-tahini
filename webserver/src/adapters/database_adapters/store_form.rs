use alohomora::{bbox::BBox, tarpc::TahiniTransformInto};
use core_tahini_utils::{
    policies::MessagePolicy,
    types::Message,
};
use database_tahini_utils::{
    policies::UserIdDBPolicy,
    types::{DatabaseRetrieveForm, DatabaseStoreForm},
};

pub struct StoreFormAdapter<
    T1: TahiniTransformInto<BBox<String, UserIdDBPolicy>>,
    T2: TahiniTransformInto<BBox<Option<String>, UserIdDBPolicy>>,
    T3: TahiniTransformInto<BBox<Message, MessagePolicy>>,
> {
    uuid: T1,
    conv_id: T2,
    message: T3,
}

pub struct RetrieveFormAdapter<
    T1: TahiniTransformInto<BBox<String, UserIdDBPolicy>>,
    T2: TahiniTransformInto<BBox<String, UserIdDBPolicy>>,
> {
    uuid: T1,
    conv_id: T2,
}

impl<
    T1: TahiniTransformInto<BBox<String, UserIdDBPolicy>>,
    T2: TahiniTransformInto<BBox<String, UserIdDBPolicy>>,
> RetrieveFormAdapter<T1, T2>
{
    pub(crate) fn new(uuid: T1, conv_id: T2) -> Self {
        RetrieveFormAdapter { uuid, conv_id }
    }
}

impl<
    T1: TahiniTransformInto<BBox<String, UserIdDBPolicy>>,
    T2: TahiniTransformInto<BBox<Option<String>, UserIdDBPolicy>>,
    T3: TahiniTransformInto<BBox<Message, MessagePolicy>>,
> StoreFormAdapter<T1, T2, T3>
{
    pub(crate) fn new(uuid: T1, conv_id: T2, message: T3) -> Self {
        StoreFormAdapter {
            uuid,
            conv_id,
            message,
        }
    }
}

impl<
    T1: TahiniTransformInto<BBox<String, UserIdDBPolicy>>,
    T2: TahiniTransformInto<BBox<Option<String>, UserIdDBPolicy>>,
    T3: TahiniTransformInto<BBox<Message, MessagePolicy>>,
> TahiniTransformInto<DatabaseStoreForm> for StoreFormAdapter<T1, T2, T3>
{
    fn transform_into(
        self,
        context: &alohomora::tarpc::context::TahiniContext,
    ) -> Result<DatabaseStoreForm, String> {
        let err = Err("Could not transform into DatabaseStoreForm".to_string());
        match context.service {
            "Database" => match context.rpc {
                "store_prompt" => Ok(DatabaseStoreForm {
                    uuid: self.uuid.transform_into(context)?,
                    conv_id: self.conv_id.transform_into(context)?,
                    message: self.message.transform_into(context)?,
                }),
                _ => err,
            },
            _ => err,
        }
    }
}

impl<
    T1: TahiniTransformInto<BBox<String, UserIdDBPolicy>>,
    T2: TahiniTransformInto<BBox<String, UserIdDBPolicy>>,
> TahiniTransformInto<DatabaseRetrieveForm> for RetrieveFormAdapter<T1, T2>
{
    fn transform_into(
        self,
        context: &alohomora::tarpc::context::TahiniContext,
    ) -> Result<DatabaseRetrieveForm, String> {
        let err = Err(format!("Could not transform into DatabaseRetrieveForm with context {}.{}", context.service, context.rpc));
        match context.service {
            "Database" => match context.rpc {
                "retrieve_prompt" => Ok(DatabaseRetrieveForm {
                    uuid: self.uuid.transform_into(context)?,
                    conv_id: self.conv_id.transform_into(context)?,
                }),
                _ => err,
            },
            _ => err,
        }
    }
}
