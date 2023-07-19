use crate::prisma::channel;

channel::select!(channel_select_without_id {
    ckey
    name
    weight
});

channel::select!(channel_select_weight { weight });
