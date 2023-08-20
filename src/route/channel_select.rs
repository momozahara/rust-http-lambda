use crate::prisma::channel;

channel::select!(channel_select_without_id {
    v_key
    name
    weight
});

channel::select!(channel_select_weight { weight });
