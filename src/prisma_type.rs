use crate::prisma::channel;

channel::select!(channel_select_without_id {
    key
    name
    weight
});
