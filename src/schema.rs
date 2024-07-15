// @generated automatically by Diesel CLI.

diesel::table! {
    article (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        sort_id -> Nullable<Int4>,
        label_id -> Nullable<Int4>,
        #[max_length = 256]
        article_cover -> Nullable<Varchar>,
        #[max_length = 32]
        article_title -> Varchar,
        article_content -> Text,
        #[max_length = 1024]
        video_url -> Nullable<Varchar>,
        view_count -> Nullable<Int4>,
        like_count -> Nullable<Int4>,
        view_status -> Nullable<Int2>,
        #[max_length = 128]
        password -> Nullable<Varchar>,
        #[max_length = 128]
        tips -> Nullable<Varchar>,
        recommend_status -> Nullable<Int2>,
        comment_status -> Nullable<Int2>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
        #[max_length = 32]
        update_by -> Nullable<Varchar>,
        deleted -> Nullable<Int2>,
    }
}

diesel::table! {
    comment (id) {
        id -> Int4,
        source -> Int4,
        #[sql_name = "type"]
        #[max_length = 32]
        type_ -> Varchar,
        parent_comment_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        floor_comment_id -> Nullable<Int4>,
        parent_user_id -> Nullable<Int4>,
        like_count -> Nullable<Int4>,
        #[max_length = 1024]
        comment_content -> Varchar,
        #[max_length = 256]
        comment_info -> Nullable<Varchar>,
        create_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    family (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        #[max_length = 256]
        bg_cover -> Varchar,
        #[max_length = 256]
        man_cover -> Varchar,
        #[max_length = 256]
        woman_cover -> Varchar,
        #[max_length = 32]
        man_name -> Varchar,
        #[max_length = 32]
        woman_name -> Varchar,
        #[max_length = 32]
        timing -> Varchar,
        #[max_length = 32]
        countdown_title -> Nullable<Varchar>,
        #[max_length = 32]
        countdown_time -> Nullable<Varchar>,
        status -> Nullable<Int2>,
        #[max_length = 1024]
        family_info -> Nullable<Varchar>,
        like_count -> Nullable<Int4>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    history_info (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        #[max_length = 128]
        ip -> Varchar,
        #[max_length = 64]
        nation -> Nullable<Varchar>,
        #[max_length = 64]
        province -> Nullable<Varchar>,
        #[max_length = 64]
        city -> Nullable<Varchar>,
        create_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    im_chat_group (id) {
        id -> Int4,
        #[max_length = 32]
        group_name -> Varchar,
        master_user_id -> Nullable<Int4>,
        #[max_length = 256]
        avatar -> Nullable<Varchar>,
        #[max_length = 128]
        introduction -> Nullable<Varchar>,
        #[max_length = 1024]
        notice -> Nullable<Varchar>,
        in_type -> Nullable<Int2>,
        group_type -> Nullable<Int2>,
        create_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    im_chat_group_user (id) {
        id -> Int4,
        group_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        verify_user_id -> Nullable<Int4>,
        #[max_length = 1024]
        remark -> Nullable<Varchar>,
        admin_flag -> Nullable<Int2>,
        user_status -> Nullable<Int2>,
        create_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    im_chat_user_friend (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        friend_id -> Nullable<Int4>,
        friend_status -> Nullable<Int2>,
        #[max_length = 32]
        remark -> Nullable<Varchar>,
        create_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    im_chat_user_group_message (id) {
        id -> Int8,
        group_id -> Nullable<Int4>,
        from_id -> Nullable<Int4>,
        to_id -> Nullable<Int4>,
        #[max_length = 1024]
        content -> Varchar,
        create_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    im_chat_user_message (id) {
        id -> Int8,
        from_id -> Nullable<Int4>,
        to_id -> Nullable<Int4>,
        #[max_length = 1024]
        content -> Varchar,
        message_status -> Nullable<Int2>,
        create_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    label (id) {
        id -> Int4,
        sort_id -> Nullable<Int4>,
        #[max_length = 32]
        label_name -> Varchar,
        #[max_length = 256]
        label_description -> Nullable<Varchar>,
    }
}

diesel::table! {
    resource (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        #[sql_name = "type"]
        #[max_length = 32]
        type_ -> Varchar,
        #[max_length = 256]
        path -> Varchar,
        size -> Nullable<Int4>,
        #[max_length = 512]
        original_name -> Nullable<Varchar>,
        #[max_length = 256]
        mime_type -> Nullable<Varchar>,
        status -> Nullable<Int2>,
        #[max_length = 16]
        store_type -> Nullable<Varchar>,
        create_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    resource_path (id) {
        id -> Int4,
        #[max_length = 64]
        title -> Varchar,
        #[max_length = 32]
        classify -> Nullable<Varchar>,
        #[max_length = 256]
        cover -> Nullable<Varchar>,
        #[max_length = 256]
        url -> Nullable<Varchar>,
        #[max_length = 1024]
        introduction -> Nullable<Varchar>,
        #[sql_name = "type"]
        #[max_length = 32]
        type_ -> Varchar,
        status -> Nullable<Int2>,
        remark -> Nullable<Text>,
        create_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    sort (id) {
        id -> Int4,
        #[max_length = 32]
        sort_name -> Varchar,
        #[max_length = 256]
        sort_description -> Varchar,
        sort_type -> Nullable<Int2>,
        priority -> Nullable<Int4>,
    }
}

diesel::table! {
    sys_config (id) {
        id -> Int4,
        #[max_length = 128]
        config_name -> Varchar,
        #[max_length = 64]
        config_key -> Varchar,
        #[max_length = 256]
        config_value -> Nullable<Varchar>,
        #[max_length = 1]
        config_type -> Bpchar,
    }
}

diesel::table! {
    tree_hole (id) {
        id -> Int4,
        #[max_length = 256]
        avatar -> Nullable<Varchar>,
        #[max_length = 64]
        message -> Varchar,
        create_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user (id) {
        id -> Int4,
        #[max_length = 32]
        username -> Nullable<Varchar>,
        #[max_length = 128]
        password -> Nullable<Varchar>,
        #[max_length = 16]
        phone_number -> Nullable<Varchar>,
        #[max_length = 32]
        email -> Nullable<Varchar>,
        user_status -> Nullable<Int2>,
        gender -> Nullable<Int2>,
        #[max_length = 128]
        open_id -> Nullable<Varchar>,
        #[max_length = 256]
        avatar -> Nullable<Varchar>,
        #[max_length = 32]
        admire -> Nullable<Varchar>,
        subscribe -> Nullable<Text>,
        #[max_length = 4096]
        introduction -> Nullable<Varchar>,
        user_type -> Nullable<Int2>,
        create_time -> Nullable<Timestamp>,
        update_time -> Nullable<Timestamp>,
        #[max_length = 32]
        update_by -> Nullable<Varchar>,
        deleted -> Nullable<Int2>,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 511]
        password -> Varchar,
    }
}

diesel::table! {
    web_info (id) {
        id -> Int4,
        #[max_length = 16]
        web_name -> Varchar,
        #[max_length = 512]
        web_title -> Varchar,
        #[max_length = 512]
        notices -> Nullable<Varchar>,
        #[max_length = 256]
        footer -> Varchar,
        #[max_length = 256]
        background_image -> Nullable<Varchar>,
        #[max_length = 256]
        avatar -> Varchar,
        random_avatar -> Nullable<Text>,
        #[max_length = 4096]
        random_name -> Nullable<Varchar>,
        random_cover -> Nullable<Text>,
        waifu_json -> Nullable<Text>,
        status -> Nullable<Int2>,
    }
}

diesel::table! {
    wei_yan (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        like_count -> Nullable<Int4>,
        #[max_length = 1024]
        content -> Varchar,
        #[sql_name = "type"]
        #[max_length = 32]
        type_ -> Varchar,
        source -> Nullable<Int4>,
        is_public -> Nullable<Int2>,
        create_time -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    article,
    comment,
    family,
    history_info,
    im_chat_group,
    im_chat_group_user,
    im_chat_user_friend,
    im_chat_user_group_message,
    im_chat_user_message,
    label,
    resource,
    resource_path,
    sort,
    sys_config,
    tree_hole,
    user,
    users,
    web_info,
    wei_yan,
);