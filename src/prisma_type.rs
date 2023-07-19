use crate::prisma::channel;

channel::select!(channel_without_id {
    key
    name
    weight
});
