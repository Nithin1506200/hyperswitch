//! Errors specific to this custom redis interface

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum RedisError {
    #[error("Invalid Redis configuration: {0}")]
    InvalidConfiguration(String),
    #[error("Failed to set key value in Redis")]
    SetFailed,
    #[error("Failed to set key value in Redis. Duplicate value")]
    SetNxFailed,
    #[error("Failed to set key value with expiry in Redis")]
    SetExFailed,
    #[error("Failed to set expiry for key value in Redis")]
    SetExpiryFailed,
    #[error("Failed to get key value in Redis")]
    GetFailed,
    #[error("Failed to delete key value in Redis")]
    DeleteFailed,
    #[error("Failed to append entry to Redis stream")]
    StreamAppendFailed,
    #[error("Failed to read entries from Redis stream")]
    StreamReadFailed,
    #[error("Failed to get stream length")]
    GetLengthFailed,
    #[error("Failed to delete entries from Redis stream")]
    StreamDeleteFailed,
    #[error("Failed to trim entries from Redis stream")]
    StreamTrimFailed,
    #[error("Failed to acknowledge Redis stream entry")]
    StreamAcknowledgeFailed,
    #[error("Stream is either empty or not available")]
    StreamEmptyOrNotAvailable,
    #[error("Failed to create Redis consumer group")]
    ConsumerGroupCreateFailed,
    #[error("Failed to destroy Redis consumer group")]
    ConsumerGroupDestroyFailed,
    #[error("Failed to delete consumer from consumer group")]
    ConsumerGroupRemoveConsumerFailed,
    #[error("Failed to set last ID on consumer group")]
    ConsumerGroupSetIdFailed,
    #[error("Failed to set Redis stream message owner")]
    ConsumerGroupClaimFailed,
    #[error("Failed to serialize application type to JSON")]
    JsonSerializationFailed,
    #[error("Failed to deserialize application type from JSON")]
    JsonDeserializationFailed,
    #[error("Failed to set hash in Redis")]
    SetHashFailed,
    #[error("Failed to set hash field in Redis")]
    SetHashFieldFailed,
    #[error("Failed to add members to set in Redis")]
    SetAddMembersFailed,
    #[error("Failed to get hash field in Redis")]
    GetHashFieldFailed,
    #[error("The requested value was not found in Redis")]
    NotFound,
    #[error("Invalid RedisEntryId provided")]
    InvalidRedisEntryId,
    #[error("Failed to establish Redis connection")]
    RedisConnectionError,
    #[error("Failed to subscribe to a channel")]
    SubscribeError,
    #[error("Failed to publish to a channel")]
    PublishError,
    #[error("Failed while receiving message from publisher")]
    OnMessageError,
    #[error("Got an unknown result from redis")]
    UnknownResult,
    #[error("Failed to append elements to list in Redis")]
    AppendElementsToListFailed,
    #[error("Failed to get list elements in Redis")]
    GetListElementsFailed,
    #[error("Failed to get length of list")]
    GetListLengthFailed,
    #[error("Failed to pop list elements in Redis")]
    PopListElementsFailed,
    #[error("Failed to increment hash field in Redis")]
    IncrementHashFieldFailed,
}
