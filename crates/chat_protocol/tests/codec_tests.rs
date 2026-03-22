//! Unit and property-based tests for chat_protocol codec.

use bytes::BytesMut;
use chat_protocol::codec::*;
use chat_protocol::error::CodecError;
use chat_protocol::types::*;
use chat_protocol::{FRAME_HEADER_SIZE, MAX_TIMESTAMP, MIN_TIMESTAMP};

// ===========================================================================
// Unit tests
// ===========================================================================

#[cfg(test)]
mod unit {
    use super::*;

    // -- FrameKind roundtrip --

    #[test]
    fn frame_kind_from_u8_roundtrip() {
        for &kind in FrameKind::all() {
            let byte = kind as u8;
            let decoded = FrameKind::from_u8(byte).expect("valid frame kind");
            assert_eq!(decoded, kind, "roundtrip failed for {kind:?} (0x{byte:02x})");
        }
    }

    #[test]
    fn frame_kind_from_u8_unknown() {
        // 0x00, 0x05, 0xFF should all return None
        for byte in [0x00, 0x06, 0x0F, 0x2E, 0x32, 0x4C, 0x56, 0xFF] {
            assert!(FrameKind::from_u8(byte).is_none(), "expected None for 0x{byte:02x}");
        }
    }

    #[test]
    fn frame_kind_all_count() {
        // Ensure all() returns the expected number of variants
        assert_eq!(FrameKind::all().len(), 53);
    }

    // -- ErrorCode --

    #[test]
    fn error_code_slug_stability() {
        // Slug values must never change — this test catches regressions
        assert_eq!(ErrorCode::Unauthorized.slug(), "unauthorized");
        assert_eq!(ErrorCode::TokenExpired.slug(), "token_expired");
        assert_eq!(ErrorCode::Forbidden.slug(), "forbidden");
        assert_eq!(ErrorCode::ChatNotFound.slug(), "chat_not_found");
        assert_eq!(ErrorCode::RateLimited.slug(), "rate_limited");
        assert_eq!(ErrorCode::InternalError.slug(), "internal_error");
        assert_eq!(ErrorCode::MalformedFrame.slug(), "malformed_frame");
        assert_eq!(ErrorCode::FrameTooLarge.slug(), "frame_too_large");
    }

    #[test]
    fn error_code_from_u16_roundtrip() {
        for &code in ErrorCode::all() {
            let wire = code as u16;
            let decoded = ErrorCode::from_u16(wire).expect("valid error code");
            assert_eq!(decoded, code);
        }
    }

    #[test]
    fn error_code_permanent_transient_classification() {
        // Permanent errors
        assert!(ErrorCode::Forbidden.is_permanent());
        assert!(ErrorCode::ChatNotFound.is_permanent());
        assert!(ErrorCode::NotChatMember.is_permanent());
        assert!(ErrorCode::MessageTooLarge.is_permanent());
        assert!(ErrorCode::ContentFiltered.is_permanent());

        // Transient errors
        assert!(ErrorCode::InternalError.is_transient());
        assert!(ErrorCode::ServiceUnavailable.is_transient());
        assert!(ErrorCode::DatabaseError.is_transient());
        assert!(ErrorCode::RateLimited.is_transient());

        // Neither
        assert!(!ErrorCode::Unauthorized.is_permanent());
        assert!(!ErrorCode::Unauthorized.is_transient());
    }

    // -- Permission --

    #[test]
    fn permission_bitflags_operations() {
        let member = Permission::SEND_MESSAGES | Permission::SEND_MEDIA;
        assert!(member.contains(Permission::SEND_MESSAGES));
        assert!(member.contains(Permission::SEND_MEDIA));
        assert!(!member.contains(Permission::BAN_MEMBERS));
    }

    #[test]
    fn default_permissions_member_group() {
        let perms = default_permissions(ChatRole::Member, ChatKind::Group);
        assert!(perms.contains(Permission::SEND_MESSAGES));
        assert!(perms.contains(Permission::EDIT_OWN_MESSAGES));
        assert!(!perms.contains(Permission::DELETE_OTHERS_MESSAGES));
        assert!(!perms.contains(Permission::BAN_MEMBERS));
    }

    #[test]
    fn default_permissions_member_channel_readonly() {
        let perms = default_permissions(ChatRole::Member, ChatKind::Channel);
        assert_eq!(perms, Permission::empty());
    }

    #[test]
    fn default_permissions_moderator() {
        let perms = default_permissions(ChatRole::Moderator, ChatKind::Group);
        assert!(perms.contains(Permission::SEND_MESSAGES));
        assert!(perms.contains(Permission::DELETE_OTHERS_MESSAGES));
        assert!(perms.contains(Permission::MUTE_MEMBERS));
        assert!(!perms.contains(Permission::BAN_MEMBERS));
    }

    #[test]
    fn default_permissions_admin() {
        let perms = default_permissions(ChatRole::Admin, ChatKind::Group);
        assert!(perms.contains(Permission::BAN_MEMBERS));
        assert!(perms.contains(Permission::INVITE_MEMBERS));
        assert!(perms.contains(Permission::MANAGE_ROLES));
        assert!(!perms.contains(Permission::TRANSFER_OWNERSHIP));
    }

    #[test]
    fn default_permissions_owner_all() {
        let perms = default_permissions(ChatRole::Owner, ChatKind::Group);
        assert_eq!(perms, Permission::all());
    }

    // -- DisconnectCode --

    #[test]
    fn disconnect_code_should_reconnect() {
        // Non-terminal (3000–3499) → reconnect
        assert!(DisconnectCode::ServerShutdown.should_reconnect());
        assert!(DisconnectCode::SessionExpired.should_reconnect());
        assert!(DisconnectCode::DuplicateSession.should_reconnect());
        assert!(DisconnectCode::ServerError.should_reconnect());
        assert!(DisconnectCode::BufferOverflow.should_reconnect());
        assert!(DisconnectCode::RateLimited.should_reconnect());

        // Terminal (3500–3999) → no reconnect
        assert!(!DisconnectCode::TokenInvalid.should_reconnect());
        assert!(!DisconnectCode::Banned.should_reconnect());
        assert!(!DisconnectCode::UnsupportedVersion.should_reconnect());
        assert!(!DisconnectCode::ConnectionLimit.should_reconnect());
    }

    #[test]
    fn disconnect_code_from_u16_roundtrip() {
        for &code in DisconnectCode::all() {
            let wire = code as u16;
            let decoded = DisconnectCode::from_u16(wire).expect("valid disconnect code");
            assert_eq!(decoded, code);
        }
    }

    // -- Frame header --

    #[test]
    fn header_encode_decode_roundtrip() {
        let header = FrameHeader {
            kind: FrameKind::SendMessage,
            seq: 42,
            event_seq: 0,
        };
        let mut buf = BytesMut::new();
        encode_header(&mut buf, &header);
        assert_eq!(buf.len(), FRAME_HEADER_SIZE);

        let decoded = decode_header(&mut buf.freeze()).unwrap();
        assert_eq!(decoded, header);
    }

    #[test]
    fn header_all_kinds_roundtrip() {
        for &kind in FrameKind::all() {
            let header = FrameHeader {
                kind,
                seq: 0xDEAD,
                event_seq: 0,
            };
            let mut buf = BytesMut::new();
            encode_header(&mut buf, &header);

            let decoded = decode_header(&mut buf.freeze()).unwrap();
            assert_eq!(decoded, header, "roundtrip failed for {kind:?}");
        }
    }

    // -- Timestamp validation --

    #[test]
    fn timestamp_valid_range() {
        let mut buf = BytesMut::new();

        // MIN valid
        write_timestamp(&mut buf, MIN_TIMESTAMP).unwrap();

        // MAX valid
        write_timestamp(&mut buf, MAX_TIMESTAMP).unwrap();

        // Typical current timestamp
        write_timestamp(&mut buf, 1_711_100_000).unwrap();
    }

    #[test]
    fn timestamp_negative_rejected() {
        let mut buf = BytesMut::new();
        let result = write_timestamp(&mut buf, -1);
        assert!(matches!(result, Err(CodecError::TimestampOutOfRange(-1))));
    }

    #[test]
    fn timestamp_too_large_rejected() {
        let mut buf = BytesMut::new();
        // Just over MAX (2^41)
        let result = write_timestamp(&mut buf, MAX_TIMESTAMP + 1);
        assert!(matches!(result, Err(CodecError::TimestampOutOfRange(_))));
    }

    #[test]
    fn timestamp_milliseconds_rejected() {
        // 2025+ in milliseconds > 2^41 (2_199_023_255_551)
        // Use a timestamp from 2040 in ms to be sure
        let millis = 2_208_988_800_000_i64; // year 2040 in ms
        assert!(
            millis > MAX_TIMESTAMP,
            "test precondition: millis must exceed MAX_TIMESTAMP"
        );
        let result = chat_protocol::codec::validate_timestamp(millis);
        assert!(matches!(result, Err(CodecError::TimestampOutOfRange(_))));
    }

    // -- Wire helpers --

    #[test]
    fn string_roundtrip_empty() {
        let mut buf = BytesMut::new();
        write_string(&mut buf, "");
        let result = read_string(&mut buf).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn string_roundtrip_unicode() {
        let mut buf = BytesMut::new();
        let input = "Привет мир 🌍 Héllo";
        write_string(&mut buf, input);
        let result = read_string(&mut buf).unwrap();
        assert_eq!(result, input);
    }

    #[test]
    fn optional_string_roundtrip_none() {
        let mut buf = BytesMut::new();
        write_optional_string(&mut buf, None);
        let result = read_optional_string(&mut buf).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn optional_string_roundtrip_some() {
        let mut buf = BytesMut::new();
        write_optional_string(&mut buf, Some("hello"));
        let result = read_optional_string(&mut buf).unwrap();
        assert_eq!(result, Some("hello".to_string()));
    }

    #[test]
    fn option_u32_roundtrip() {
        let mut buf = BytesMut::new();
        write_option_u32(&mut buf, None);
        write_option_u32(&mut buf, Some(42));

        assert_eq!(read_option_u32(&mut buf).unwrap(), None);
        assert_eq!(read_option_u32(&mut buf).unwrap(), Some(42));
    }

    #[test]
    fn uuid_roundtrip() {
        let mut buf = BytesMut::new();
        let id = uuid::Uuid::new_v4();
        write_uuid(&mut buf, &id);
        let result = read_uuid(&mut buf).unwrap();
        assert_eq!(result, id);
    }

    // -- Payload roundtrips --

    #[test]
    fn hello_roundtrip() {
        let payload = HelloPayload {
            protocol_version: 1,
            sdk_version: "1.0.0".into(),
            platform: "rust".into(),
            token: "jwt.token.here".into(),
            device_id: uuid::Uuid::new_v4(),
        };
        let mut buf = BytesMut::new();
        encode_hello(&mut buf, &payload).unwrap();

        let decoded = decode_hello(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn welcome_roundtrip() {
        let payload = WelcomePayload {
            session_id: 1,
            server_time: 1_711_100_000,
            user_id: 42,
            limits: ServerLimits::default(),
            capabilities: ServerCapabilities::MEDIA_UPLOAD | ServerCapabilities::SEARCH,
        };
        let mut buf = BytesMut::new();
        encode_welcome(&mut buf, &payload).unwrap();

        let decoded = decode_welcome(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn send_message_roundtrip() {
        let payload = SendMessagePayload {
            chat_id: 1,
            kind: MessageKind::Text,
            idempotency_key: uuid::Uuid::new_v4(),
            reply_to_id: Some(10),
            content: "Hello, world!".into(),
            rich_content: None,
            extra: Some(r#"{"key":"value"}"#.into()),
            mentioned_user_ids: vec![42, 99],
        };
        let mut buf = BytesMut::new();
        encode_send_message(&mut buf, &payload);

        let decoded = decode_send_message(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn edit_message_roundtrip() {
        let payload = EditMessagePayload {
            chat_id: 1,
            message_id: 5,
            content: "edited content".into(),
            rich_content: None,
            extra: None,
        };
        let mut buf = BytesMut::new();
        encode_edit_message(&mut buf, &payload);

        let decoded = decode_edit_message(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn delete_message_roundtrip() {
        let payload = DeleteMessagePayload {
            chat_id: 1,
            message_id: 5,
        };
        let mut buf = BytesMut::new();
        encode_delete_message(&mut buf, &payload);

        let decoded = decode_delete_message(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn load_messages_paginate_roundtrip() {
        let payload = LoadMessagesPayload::Paginate {
            chat_id: 1,
            direction: LoadDirection::Older,
            anchor_id: 100,
            limit: 50,
        };
        let mut buf = BytesMut::new();
        encode_load_messages(&mut buf, &payload).unwrap();

        let decoded = decode_load_messages(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn load_messages_range_check_roundtrip() {
        let payload = LoadMessagesPayload::RangeCheck {
            chat_id: 1,
            from_id: 50,
            to_id: 100,
            since_ts: 1_711_100_000,
        };
        let mut buf = BytesMut::new();
        encode_load_messages(&mut buf, &payload).unwrap();

        let decoded = decode_load_messages(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn error_payload_roundtrip() {
        let payload = ErrorPayload {
            code: ErrorCode::RateLimited,
            message: "Too many messages".into(),
            retry_after_ms: 5000,
            extra: Some(r#"{"detail":"slow down"}"#.into()),
        };
        let mut buf = BytesMut::new();
        encode_error(&mut buf, &payload);

        let decoded = decode_error(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn chat_entry_roundtrip() {
        let entry = ChatEntry {
            id: 1,
            kind: ChatKind::Group,
            parent_id: None,
            created_at: 1_711_100_000,
            updated_at: 1_711_100_100,
            title: Some("Test Group".into()),
            avatar_url: Some("https://example.com/avatar.png".into()),
            last_message: None,
            unread_count: 0,
            member_count: 0,
        };
        let mut buf = BytesMut::new();
        encode_chat_entry(&mut buf, &entry).unwrap();

        let decoded = decode_chat_entry(&mut buf).unwrap();
        assert_eq!(decoded, entry);
    }

    #[test]
    fn chat_entry_dm_no_strings() {
        let entry = ChatEntry {
            id: 1,
            kind: ChatKind::Direct,
            parent_id: None,
            created_at: 1_711_100_000,
            updated_at: 1_711_100_000,
            title: None,
            avatar_url: None,
            last_message: None,
            unread_count: 0,
            member_count: 0,
        };
        let mut buf = BytesMut::new();
        encode_chat_entry(&mut buf, &entry).unwrap();

        // Minimum size: 4+1+1+8+8+4+4+1(last_message flag)+4(unread)+4(member) = 39 bytes
        assert_eq!(buf.len(), 39);

        let decoded = decode_chat_entry(&mut buf).unwrap();
        assert_eq!(decoded, entry);
    }

    #[test]
    fn chat_entry_channel_with_parent() {
        let entry = ChatEntry {
            id: 10,
            kind: ChatKind::Channel,
            parent_id: Some(5),
            created_at: 1_711_100_000,
            updated_at: 1_711_100_000,
            title: Some("general".into()),
            avatar_url: None,
            last_message: None,
            unread_count: 0,
            member_count: 0,
        };
        let mut buf = BytesMut::new();
        encode_chat_entry(&mut buf, &entry).unwrap();

        let decoded = decode_chat_entry(&mut buf).unwrap();
        assert_eq!(decoded, entry);
    }

    #[test]
    fn chat_member_entry_no_override() {
        let entry = ChatMemberEntry {
            user_id: 42,
            role: ChatRole::Member,
            permissions: None,
        };
        let mut buf = BytesMut::new();
        encode_chat_member_entry(&mut buf, &entry);
        assert_eq!(buf.len(), 6); // 4+1+1

        let decoded = decode_chat_member_entry(&mut buf).unwrap();
        assert_eq!(decoded, entry);
    }

    #[test]
    fn chat_member_entry_with_override() {
        let entry = ChatMemberEntry {
            user_id: 42,
            role: ChatRole::Admin,
            permissions: Some(Permission::SEND_MESSAGES | Permission::BAN_MEMBERS),
        };
        let mut buf = BytesMut::new();
        encode_chat_member_entry(&mut buf, &entry);
        assert_eq!(buf.len(), 10); // 4+1+1+4

        let decoded = decode_chat_member_entry(&mut buf).unwrap();
        assert_eq!(decoded, entry);
    }

    #[test]
    fn user_entry_roundtrip() {
        let entry = UserEntry {
            id: 42,
            flags: UserFlags::BOT,
            created_at: 1_711_100_000,
            updated_at: 1_711_100_100,
            username: Some("testbot".into()),
            first_name: Some("Test".into()),
            last_name: None,
            avatar_url: None,
        };
        let mut buf = BytesMut::new();
        encode_user_entry(&mut buf, &entry).unwrap();

        let decoded = decode_user_entry(&mut buf).unwrap();
        assert_eq!(decoded, entry);
    }

    #[test]
    fn presence_result_roundtrip() {
        let entries = vec![
            PresenceEntry {
                user_id: 1,
                status: PresenceStatus::Online,
                last_seen: 0,
            },
            PresenceEntry {
                user_id: 2,
                status: PresenceStatus::Offline,
                last_seen: 1_711_100_000,
            },
        ];
        let mut buf = BytesMut::new();
        encode_presence_result(&mut buf, &entries).unwrap();

        let decoded = decode_presence_result(&mut buf).unwrap();
        assert_eq!(decoded, entries);
    }

    // -- Message & batch --

    fn sample_message(id: u32) -> Message {
        Message {
            id,
            chat_id: 1,
            sender_id: 42,
            created_at: 1_711_100_000,
            updated_at: 1_711_100_000,
            kind: MessageKind::Text,
            flags: MessageFlags::empty(),
            reply_to_id: None,
            content: format!("Message {id}"),
            rich_content: None,
            extra: None,
        }
    }

    #[test]
    fn message_roundtrip_plain() {
        let msg = sample_message(1);
        let mut buf = BytesMut::new();
        encode_message(&mut buf, &msg).unwrap();

        let decoded = decode_message(&mut buf).unwrap();
        assert_eq!(decoded, msg);
    }

    #[test]
    fn message_roundtrip_with_rich_and_extra() {
        let msg = Message {
            id: 1,
            chat_id: 1,
            sender_id: 42,
            created_at: 1_711_100_000,
            updated_at: 1_711_100_100,
            kind: MessageKind::Text,
            flags: MessageFlags::EDITED | MessageFlags::REPLY,
            reply_to_id: Some(5),
            content: "Hello bold world".into(),
            rich_content: Some(vec![
                RichSpan {
                    start: 6,
                    end: 10,
                    style: RichStyle::BOLD,
                    meta: None,
                },
                RichSpan {
                    start: 0,
                    end: 16,
                    style: RichStyle::LINK,
                    meta: Some(r#"{"url":"https://example.com"}"#.into()),
                },
            ]),
            extra: Some(r#"{"reply":{"chat_id":1,"msg_id":5}}"#.into()),
        };
        let mut buf = BytesMut::new();
        encode_message(&mut buf, &msg).unwrap();

        let decoded = decode_message(&mut buf).unwrap();
        assert_eq!(decoded, msg);
    }

    #[test]
    fn message_batch_roundtrip() {
        let batch = MessageBatch {
            messages: (1..=10).map(sample_message).collect(),
            has_more: true,
        };
        let mut buf = BytesMut::new();
        encode_message_batch(&mut buf, &batch).unwrap();

        let decoded = decode_message_batch(&mut buf).unwrap();
        assert_eq!(decoded, batch);
    }

    #[test]
    fn message_batch_empty() {
        let batch = MessageBatch {
            messages: vec![],
            has_more: false,
        };
        let mut buf = BytesMut::new();
        encode_message_batch(&mut buf, &batch).unwrap();

        let decoded = decode_message_batch(&mut buf).unwrap();
        assert_eq!(decoded, batch);
    }

    // -- Rich content --

    #[test]
    fn rich_content_overlapping_spans() {
        let spans = vec![
            RichSpan {
                start: 0,
                end: 10,
                style: RichStyle::BOLD,
                meta: None,
            },
            RichSpan {
                start: 5,
                end: 15,
                style: RichStyle::ITALIC,
                meta: None,
            },
            RichSpan {
                start: 0,
                end: 20,
                style: RichStyle::LINK,
                meta: Some(r#"{"url":"https://x.com"}"#.into()),
            },
        ];
        let mut buf = BytesMut::new();
        encode_rich_content(&mut buf, &spans);

        let decoded = decode_rich_content(&mut buf).unwrap();
        assert_eq!(decoded, spans);
    }

    #[test]
    fn rich_content_code_block_with_lang() {
        let spans = vec![RichSpan {
            start: 0,
            end: 100,
            style: RichStyle::CODE_BLOCK,
            meta: Some(r#"{"lang":"rust"}"#.into()),
        }];
        let mut buf = BytesMut::new();
        encode_rich_content(&mut buf, &spans);

        let decoded = decode_rich_content(&mut buf).unwrap();
        assert_eq!(decoded, spans);
    }

    #[test]
    fn rich_style_has_meta() {
        assert!(!RichStyle::BOLD.has_meta());
        assert!(!RichStyle::ITALIC.has_meta());
        assert!(!RichStyle::UNDERLINE.has_meta());
        assert!(!RichStyle::STRIKE.has_meta());
        assert!(!RichStyle::CODE.has_meta());
        assert!(!RichStyle::SPOILER.has_meta());
        assert!(!RichStyle::BLOCKQUOTE.has_meta());

        assert!(RichStyle::LINK.has_meta());
        assert!(RichStyle::MENTION.has_meta());
        assert!(RichStyle::COLOR.has_meta());
        assert!(RichStyle::CODE_BLOCK.has_meta());

        // Combined
        assert!((RichStyle::BOLD | RichStyle::LINK).has_meta());
    }

    // -- Error cases --

    #[test]
    fn truncated_header() {
        let buf = bytes::Bytes::from_static(&[0x10]); // only 1 byte, need 9
        let err = decode_header(&mut buf.clone()).unwrap_err();
        assert!(matches!(
            err,
            CodecError::Truncated {
                needed: 9,
                available: 1
            }
        ));
    }

    #[test]
    fn truncated_header_empty() {
        let buf = bytes::Bytes::new();
        let err = decode_header(&mut buf.clone()).unwrap_err();
        assert!(matches!(
            err,
            CodecError::Truncated {
                needed: 9,
                available: 0
            }
        ));
    }

    #[test]
    fn unknown_frame_kind() {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(&[0xFF, 0, 0, 0, 0, 0, 0, 0, 0]); // unknown kind 0xFF
        let err = decode_header(&mut buf.freeze()).unwrap_err();
        assert!(matches!(err, CodecError::UnknownFrameKind(0xFF)));
    }

    #[test]
    fn truncated_string() {
        let mut buf = BytesMut::new();
        write_u32(&mut buf, 100); // claims 100 bytes, but nothing follows
        let err = read_string(&mut buf).unwrap_err();
        assert!(matches!(err, CodecError::Truncated { needed: 100, .. }));
    }

    #[test]
    fn truncated_message() {
        // Write only the first 4 bytes of a message (id) — should fail on chat_id
        let mut buf = BytesMut::new();
        write_u32(&mut buf, 1);
        let err = decode_message(&mut buf).unwrap_err();
        assert!(matches!(err, CodecError::Truncated { .. }));
    }

    #[test]
    fn unknown_message_kind() {
        // Write a complete message header with an invalid kind byte
        let mut buf = BytesMut::new();
        write_u32(&mut buf, 1); // id
        write_u32(&mut buf, 1); // chat_id
        write_u32(&mut buf, 1); // sender_id
        write_i64(&mut buf, 1_711_100_000); // created_at
        write_i64(&mut buf, 1_711_100_000); // updated_at
        write_u8(&mut buf, 99); // invalid kind
        // flags + rest don't matter, it should fail on kind

        let err = decode_message(&mut buf).unwrap_err();
        assert!(matches!(
            err,
            CodecError::UnknownDiscriminant {
                type_name: "MessageKind",
                ..
            }
        ));
    }

    #[test]
    fn timestamp_out_of_range_in_message() {
        let msg = Message {
            id: 1,
            chat_id: 1,
            sender_id: 1,
            created_at: -1, // invalid
            updated_at: 1_711_100_000,
            kind: MessageKind::Text,
            flags: MessageFlags::empty(),
            reply_to_id: None,
            content: String::new(),
            rich_content: None,
            extra: None,
        };
        let mut buf = BytesMut::new();
        let err = encode_message(&mut buf, &msg).unwrap_err();
        assert!(matches!(err, CodecError::TimestampOutOfRange(-1)));
    }

    // -- CreateChat --

    #[test]
    fn create_chat_roundtrip() {
        let payload = CreateChatPayload {
            kind: ChatKind::Group,
            parent_id: None,
            title: Some("My Group".into()),
            avatar_url: None,
            member_ids: vec![1, 2, 3],
        };
        let mut buf = BytesMut::new();
        encode_create_chat(&mut buf, &payload);

        let decoded = decode_create_chat(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    // -- Event payloads --

    #[test]
    fn receipt_update_roundtrip() {
        let payload = ReceiptUpdatePayload {
            chat_id: 1,
            user_id: 42,
            message_id: 100,
        };
        let mut buf = BytesMut::new();
        encode_receipt_update(&mut buf, &payload);

        let decoded = decode_receipt_update(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn typing_update_roundtrip() {
        let payload = TypingUpdatePayload {
            chat_id: 1,
            user_id: 42,
            expires_in_ms: 5000,
        };
        let mut buf = BytesMut::new();
        encode_typing_update(&mut buf, &payload);

        let decoded = decode_typing_update(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn member_joined_left_roundtrip() {
        let joined = MemberJoinedPayload {
            chat_id: 1,
            user_id: 42,
            role: ChatRole::Member,
            invited_by: 10,
        };
        let mut buf = BytesMut::new();
        encode_member_joined(&mut buf, &joined);
        let decoded = decode_member_joined(&mut buf).unwrap();
        assert_eq!(decoded, joined);

        let left = MemberLeftPayload {
            chat_id: 1,
            user_id: 42,
        };
        let mut buf = BytesMut::new();
        encode_member_left(&mut buf, &left);
        let decoded = decode_member_left(&mut buf).unwrap();
        assert_eq!(decoded, left);
    }

    // -- UpdateMember --

    #[test]
    fn update_member_kick_roundtrip() {
        let payload = UpdateMemberPayload {
            chat_id: 1,
            user_id: 42,
            action: MemberAction::Kick,
        };
        let mut buf = BytesMut::new();
        encode_update_member(&mut buf, &payload);
        let decoded = decode_update_member(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn update_member_ban_roundtrip() {
        let payload = UpdateMemberPayload {
            chat_id: 1,
            user_id: 42,
            action: MemberAction::Ban,
        };
        let mut buf = BytesMut::new();
        encode_update_member(&mut buf, &payload);
        let decoded = decode_update_member(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn update_member_mute_roundtrip() {
        let payload = UpdateMemberPayload {
            chat_id: 1,
            user_id: 42,
            action: MemberAction::Mute { duration_secs: 3600 },
        };
        let mut buf = BytesMut::new();
        encode_update_member(&mut buf, &payload);
        let decoded = decode_update_member(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn update_member_unmute_roundtrip() {
        let payload = UpdateMemberPayload {
            chat_id: 1,
            user_id: 42,
            action: MemberAction::Mute { duration_secs: 0 },
        };
        let mut buf = BytesMut::new();
        encode_update_member(&mut buf, &payload);
        let decoded = decode_update_member(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn update_member_change_role_roundtrip() {
        let payload = UpdateMemberPayload {
            chat_id: 1,
            user_id: 42,
            action: MemberAction::ChangeRole(ChatRole::Admin),
        };
        let mut buf = BytesMut::new();
        encode_update_member(&mut buf, &payload);
        let decoded = decode_update_member(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn update_member_permissions_roundtrip() {
        let payload = UpdateMemberPayload {
            chat_id: 1,
            user_id: 42,
            action: MemberAction::UpdatePermissions(Permission::SEND_MESSAGES | Permission::SEND_MEDIA),
        };
        let mut buf = BytesMut::new();
        encode_update_member(&mut buf, &payload);
        let decoded = decode_update_member(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    // -- UpdateChat --

    #[test]
    fn update_chat_set_title_roundtrip() {
        let payload = UpdateChatPayload {
            chat_id: 1,
            title: Some("New Title".into()),
            avatar_url: None, // don't change
        };
        let mut buf = BytesMut::new();
        encode_update_chat(&mut buf, &payload);
        let decoded = decode_update_chat(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn update_chat_clear_avatar_roundtrip() {
        let payload = UpdateChatPayload {
            chat_id: 1,
            title: None,                     // don't change
            avatar_url: Some(String::new()), // clear
        };
        let mut buf = BytesMut::new();
        encode_update_chat(&mut buf, &payload);
        let decoded = decode_update_chat(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    // -- MessageDeleted event --

    #[test]
    fn message_deleted_roundtrip() {
        let payload = MessageDeletedPayload {
            chat_id: 1,
            message_id: 42,
        };
        let mut buf = BytesMut::new();
        encode_message_deleted(&mut buf, &payload);
        let decoded = decode_message_deleted(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    // -- Missing codec roundtrips --

    #[test]
    fn delete_chat_roundtrip() {
        let payload = DeleteChatPayload { chat_id: 1 };
        let mut buf = BytesMut::new();
        encode_delete_chat(&mut buf, &payload);
        let decoded = decode_delete_chat(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn get_chat_info_roundtrip() {
        let payload = GetChatInfoPayload { chat_id: 1 };
        let mut buf = BytesMut::new();
        encode_get_chat_info(&mut buf, &payload);
        let decoded = decode_get_chat_info(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn get_chat_members_roundtrip() {
        let payload = GetChatMembersPayload {
            chat_id: 1,
            cursor: 0,
            limit: 50,
        };
        let mut buf = BytesMut::new();
        encode_get_chat_members(&mut buf, &payload);
        let decoded = decode_get_chat_members(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn invite_members_roundtrip() {
        let payload = InviteMembersPayload {
            chat_id: 1,
            user_ids: vec![10, 20, 30],
        };
        let mut buf = BytesMut::new();
        encode_invite_members(&mut buf, &payload);
        let decoded = decode_invite_members(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn leave_chat_roundtrip() {
        let payload = LeaveChatPayload { chat_id: 1 };
        let mut buf = BytesMut::new();
        encode_leave_chat(&mut buf, &payload);
        let decoded = decode_leave_chat(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    // -- SendMessage with kind --

    #[test]
    fn send_message_image_kind_roundtrip() {
        let payload = SendMessagePayload {
            chat_id: 1,
            kind: MessageKind::Image,
            idempotency_key: uuid::Uuid::new_v4(),
            reply_to_id: None,
            content: "photo.jpg".into(),
            rich_content: None,
            extra: Some(r#"{"url":"https://cdn.example.com/photo.jpg"}"#.into()),
            mentioned_user_ids: vec![],
        };
        let mut buf = BytesMut::new();
        encode_send_message(&mut buf, &payload);
        let decoded = decode_send_message(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    // -- LoadChats enum --

    #[test]
    fn load_chats_first_page_roundtrip() {
        let payload = LoadChatsPayload::FirstPage { limit: 20 };
        let mut buf = BytesMut::new();
        encode_load_chats(&mut buf, &payload).unwrap();
        let decoded = decode_load_chats(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn load_chats_after_roundtrip() {
        let payload = LoadChatsPayload::After {
            cursor_ts: 1_711_100_000,
            limit: 20,
        };
        let mut buf = BytesMut::new();
        encode_load_chats(&mut buf, &payload).unwrap();
        let decoded = decode_load_chats(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    // -- Subscribe batch --

    #[test]
    fn subscribe_batch_roundtrip() {
        let payload = SubscribePayload {
            channels: vec!["general".into(), "chat#1".into(), "chat#100".into()],
        };
        let mut buf = BytesMut::new();
        encode_subscribe(&mut buf, &payload);
        let decoded = decode_subscribe(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn subscribe_single_roundtrip() {
        let payload = SubscribePayload {
            channels: vec!["chat#42".into()],
        };
        let mut buf = BytesMut::new();
        encode_subscribe(&mut buf, &payload);
        let decoded = decode_subscribe(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn subscribe_empty_roundtrip() {
        let payload = SubscribePayload { channels: vec![] };
        let mut buf = BytesMut::new();
        encode_subscribe(&mut buf, &payload);
        let decoded = decode_subscribe(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn unsubscribe_batch_roundtrip() {
        let payload = UnsubscribePayload {
            channels: vec!["chat#1".into(), "chat#2".into(), "chat#3".into()],
        };
        let mut buf = BytesMut::new();
        encode_unsubscribe(&mut buf, &payload);
        let decoded = decode_unsubscribe(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    // -- Frame encode/decode roundtrip --

    #[test]
    fn frame_ping_roundtrip() {
        let frame = Frame {
            seq: 0,
            event_seq: 0,
            payload: FramePayload::Ping,
        };
        let mut buf = BytesMut::new();
        encode_frame(&mut buf, &frame).unwrap();
        let decoded = decode_frame(&mut buf).unwrap();
        assert_eq!(decoded, frame);
    }

    #[test]
    fn frame_send_message_roundtrip() {
        let frame = Frame {
            seq: 7,
            event_seq: 0,
            payload: FramePayload::SendMessage(SendMessagePayload {
                chat_id: 1,
                kind: MessageKind::Text,
                idempotency_key: uuid::Uuid::new_v4(),
                reply_to_id: None,
                content: "hello".into(),
                rich_content: None,
                extra: None,
                mentioned_user_ids: vec![],
            }),
        };
        let mut buf = BytesMut::new();
        encode_frame(&mut buf, &frame).unwrap();
        let decoded = decode_frame(&mut buf).unwrap();
        assert_eq!(decoded, frame);
    }

    #[test]
    fn frame_error_roundtrip() {
        let frame = Frame {
            seq: 3,
            event_seq: 0,
            payload: FramePayload::Error(ErrorPayload {
                code: ErrorCode::RateLimited,
                message: "slow down".into(),
                retry_after_ms: 5000,
                extra: None,
            }),
        };
        let mut buf = BytesMut::new();
        encode_frame(&mut buf, &frame).unwrap();
        let decoded = decode_frame(&mut buf).unwrap();
        assert_eq!(decoded, frame);
    }

    #[test]
    fn frame_subscribe_batch_roundtrip() {
        let frame = Frame {
            seq: 1,
            event_seq: 0,
            payload: FramePayload::Subscribe(SubscribePayload {
                channels: vec!["chat#10".into(), "chat#20".into(), "chat#30".into()],
            }),
        };
        let mut buf = BytesMut::new();
        encode_frame(&mut buf, &frame).unwrap();
        let decoded = decode_frame(&mut buf).unwrap();
        assert_eq!(decoded, frame);
    }

    #[test]
    fn frame_load_chats_first_page_roundtrip() {
        let frame = Frame {
            seq: 2,
            event_seq: 0,
            payload: FramePayload::LoadChats(LoadChatsPayload::FirstPage { limit: 50 }),
        };
        let mut buf = BytesMut::new();
        encode_frame(&mut buf, &frame).unwrap();
        let decoded = decode_frame(&mut buf).unwrap();
        assert_eq!(decoded, frame);
    }

    #[test]
    fn frame_payload_kind_consistency() {
        assert_eq!(FramePayload::Ping.kind(), FrameKind::Ping);
        assert_eq!(FramePayload::Pong.kind(), FrameKind::Pong);
        assert_eq!(FramePayload::Ack(AckPayload::Empty).kind(), FrameKind::Ack);
    }

    // -- New frame roundtrips --

    #[test]
    fn add_reaction_roundtrip() {
        let payload = AddReactionPayload {
            chat_id: 1,
            message_id: 42,
            pack_id: 0,
            emoji_index: 5,
        };
        let mut buf = BytesMut::new();
        encode_add_reaction(&mut buf, &payload);
        let decoded = decode_add_reaction(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn remove_reaction_roundtrip() {
        let payload = RemoveReactionPayload {
            chat_id: 1,
            message_id: 42,
            pack_id: 100,
            emoji_index: 255,
        };
        let mut buf = BytesMut::new();
        encode_remove_reaction(&mut buf, &payload);
        let decoded = decode_remove_reaction(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn reaction_update_roundtrip() {
        let payload = ReactionUpdatePayload {
            chat_id: 1,
            message_id: 42,
            user_id: 7,
            pack_id: 0,
            emoji_index: 3,
            added: true,
        };
        let mut buf = BytesMut::new();
        encode_reaction_update(&mut buf, &payload);
        let decoded = decode_reaction_update(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn pin_unpin_roundtrip() {
        let pin = PinMessagePayload {
            chat_id: 1,
            message_id: 42,
        };
        let mut buf = BytesMut::new();
        encode_pin_message(&mut buf, &pin);
        assert_eq!(decode_pin_message(&mut buf).unwrap(), pin);

        let unpin = UnpinMessagePayload {
            chat_id: 1,
            message_id: 42,
        };
        let mut buf = BytesMut::new();
        encode_unpin_message(&mut buf, &unpin);
        assert_eq!(decode_unpin_message(&mut buf).unwrap(), unpin);
    }

    #[test]
    fn refresh_token_roundtrip() {
        let payload = RefreshTokenPayload {
            token: "new-jwt-token".into(),
        };
        let mut buf = BytesMut::new();
        encode_refresh_token(&mut buf, &payload);
        let decoded = decode_refresh_token(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn forward_message_roundtrip() {
        let payload = ForwardMessagePayload {
            from_chat_id: 1,
            message_id: 42,
            to_chat_id: 2,
            idempotency_key: uuid::Uuid::new_v4(),
        };
        let mut buf = BytesMut::new();
        encode_forward_message(&mut buf, &payload);
        let decoded = decode_forward_message(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn get_user_roundtrip() {
        let payload = GetUserPayload { user_id: 42 };
        let mut buf = BytesMut::new();
        encode_get_user(&mut buf, &payload);
        assert_eq!(decode_get_user(&mut buf).unwrap(), payload);
    }

    #[test]
    fn get_users_roundtrip() {
        let payload = GetUsersPayload {
            user_ids: vec![1, 2, 3],
        };
        let mut buf = BytesMut::new();
        encode_get_users(&mut buf, &payload);
        assert_eq!(decode_get_users(&mut buf).unwrap(), payload);
    }

    #[test]
    fn update_profile_roundtrip() {
        let payload = UpdateProfilePayload {
            username: Some("newname".into()),
            first_name: None,
            last_name: Some("".into()), // clear
            avatar_url: None,
        };
        let mut buf = BytesMut::new();
        encode_update_profile(&mut buf, &payload);
        assert_eq!(decode_update_profile(&mut buf).unwrap(), payload);
    }

    #[test]
    fn block_unblock_roundtrip() {
        let block = BlockUserPayload { user_id: 42 };
        let mut buf = BytesMut::new();
        encode_block_user(&mut buf, &block);
        assert_eq!(decode_block_user(&mut buf).unwrap(), block);

        let unblock = UnblockUserPayload { user_id: 42 };
        let mut buf = BytesMut::new();
        encode_unblock_user(&mut buf, &unblock);
        assert_eq!(decode_unblock_user(&mut buf).unwrap(), unblock);
    }

    #[test]
    fn get_block_list_roundtrip() {
        let payload = GetBlockListPayload { cursor: 0, limit: 50 };
        let mut buf = BytesMut::new();
        encode_get_block_list(&mut buf, &payload);
        assert_eq!(decode_get_block_list(&mut buf).unwrap(), payload);
    }

    #[test]
    fn mute_unmute_chat_roundtrip() {
        let mute = MuteChatPayload {
            chat_id: 1,
            duration_secs: 3600,
        };
        let mut buf = BytesMut::new();
        encode_mute_chat(&mut buf, &mute);
        assert_eq!(decode_mute_chat(&mut buf).unwrap(), mute);

        let unmute = UnmuteChatPayload { chat_id: 1 };
        let mut buf = BytesMut::new();
        encode_unmute_chat(&mut buf, &unmute);
        assert_eq!(decode_unmute_chat(&mut buf).unwrap(), unmute);
    }

    #[test]
    fn search_scope_roundtrip() {
        // Chat scope
        let payload = SearchPayload {
            scope: SearchScope::Chat { chat_id: 42 },
            query: "hello".into(),
            cursor: 0,
            limit: 20,
        };
        let mut buf = BytesMut::new();
        encode_search(&mut buf, &payload);
        assert_eq!(decode_search(&mut buf).unwrap(), payload);

        // Global scope
        let payload = SearchPayload {
            scope: SearchScope::Global,
            query: "test".into(),
            cursor: 5,
            limit: 10,
        };
        let mut buf = BytesMut::new();
        encode_search(&mut buf, &payload);
        assert_eq!(decode_search(&mut buf).unwrap(), payload);

        // User scope
        let payload = SearchPayload {
            scope: SearchScope::User { user_id: 7 },
            query: "from user".into(),
            cursor: 0,
            limit: 50,
        };
        let mut buf = BytesMut::new();
        encode_search(&mut buf, &payload);
        assert_eq!(decode_search(&mut buf).unwrap(), payload);
    }

    #[test]
    fn chat_entry_with_last_message_roundtrip() {
        let entry = ChatEntry {
            id: 1,
            kind: ChatKind::Group,
            parent_id: None,
            created_at: 1_711_100_000,
            updated_at: 1_711_100_100,
            title: Some("Test Group".into()),
            avatar_url: None,
            last_message: Some(LastMessagePreview {
                id: 42,
                sender_id: 7,
                created_at: 1_711_100_050,
                kind: MessageKind::Text,
                flags: MessageFlags::empty(),
                content_preview: "Hello world!".into(),
            }),
            unread_count: 5,
            member_count: 12,
        };
        let mut buf = BytesMut::new();
        encode_chat_entry(&mut buf, &entry).unwrap();
        let decoded = decode_chat_entry(&mut buf).unwrap();
        assert_eq!(decoded, entry);
    }

    #[test]
    fn send_message_with_mentions_roundtrip() {
        let payload = SendMessagePayload {
            chat_id: 1,
            kind: MessageKind::Text,
            idempotency_key: uuid::Uuid::new_v4(),
            reply_to_id: None,
            content: "Hey @alice @bob".into(),
            rich_content: None,
            extra: None,
            mentioned_user_ids: vec![42, 99, 7],
        };
        let mut buf = BytesMut::new();
        encode_send_message(&mut buf, &payload);
        let decoded = decode_send_message(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn unban_member_action_roundtrip() {
        let payload = UpdateMemberPayload {
            chat_id: 1,
            user_id: 42,
            action: MemberAction::Unban,
        };
        let mut buf = BytesMut::new();
        encode_update_member(&mut buf, &payload);
        let decoded = decode_update_member(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn event_seq_in_frame_roundtrip() {
        let frame = Frame {
            seq: 0,
            event_seq: 12345,
            payload: FramePayload::Ping,
        };
        let mut buf = BytesMut::new();
        encode_frame(&mut buf, &frame).unwrap();
        let decoded = decode_frame(&mut buf).unwrap();
        assert_eq!(decoded.event_seq, 12345);
        assert_eq!(decoded, frame);
    }

    // -- ChatDeleted & MemberUpdated --

    #[test]
    fn chat_deleted_roundtrip() {
        let payload = ChatDeletedPayload { chat_id: 42 };
        let mut buf = BytesMut::new();
        encode_chat_deleted(&mut buf, &payload);
        assert_eq!(decode_chat_deleted(&mut buf).unwrap(), payload);
    }

    #[test]
    fn member_updated_roundtrip() {
        let payload = MemberUpdatedPayload {
            chat_id: 1,
            user_id: 42,
            role: ChatRole::Moderator,
            permissions: Some(Permission::SEND_MESSAGES | Permission::MUTE_MEMBERS),
        };
        let mut buf = BytesMut::new();
        encode_member_updated(&mut buf, &payload);
        assert_eq!(decode_member_updated(&mut buf).unwrap(), payload);
    }

    #[test]
    fn member_updated_no_override_roundtrip() {
        let payload = MemberUpdatedPayload {
            chat_id: 1,
            user_id: 42,
            role: ChatRole::Admin,
            permissions: None,
        };
        let mut buf = BytesMut::new();
        encode_member_updated(&mut buf, &payload);
        assert_eq!(decode_member_updated(&mut buf).unwrap(), payload);
    }

    #[test]
    fn frame_chat_deleted_roundtrip() {
        let frame = Frame {
            seq: 0,
            event_seq: 100,
            payload: FramePayload::ChatDeleted(ChatDeletedPayload { chat_id: 5 }),
        };
        let mut buf = BytesMut::new();
        encode_frame(&mut buf, &frame).unwrap();
        let decoded = decode_frame(&mut buf).unwrap();
        assert_eq!(decoded, frame);
    }

    #[test]
    fn frame_member_updated_roundtrip() {
        let frame = Frame {
            seq: 0,
            event_seq: 200,
            payload: FramePayload::MemberUpdated(MemberUpdatedPayload {
                chat_id: 1,
                user_id: 42,
                role: ChatRole::Moderator,
                permissions: None,
            }),
        };
        let mut buf = BytesMut::new();
        encode_frame(&mut buf, &frame).unwrap();
        let decoded = decode_frame(&mut buf).unwrap();
        assert_eq!(decoded, frame);
    }

    // -- reply_to_id --

    #[test]
    fn send_message_reply_roundtrip() {
        let payload = SendMessagePayload {
            chat_id: 1,
            kind: MessageKind::Text,
            idempotency_key: uuid::Uuid::new_v4(),
            reply_to_id: Some(42),
            content: "replying".into(),
            rich_content: None,
            extra: None,
            mentioned_user_ids: vec![7],
        };
        let mut buf = BytesMut::new();
        encode_send_message(&mut buf, &payload);
        let decoded = decode_send_message(&mut buf).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn message_with_reply_roundtrip() {
        let msg = Message {
            id: 10,
            chat_id: 1,
            sender_id: 42,
            created_at: 1_711_100_000,
            updated_at: 1_711_100_000,
            kind: MessageKind::Text,
            flags: MessageFlags::REPLY,
            reply_to_id: Some(5),
            content: "replying to msg 5".into(),
            rich_content: None,
            extra: None,
        };
        let mut buf = BytesMut::new();
        encode_message(&mut buf, &msg).unwrap();
        let decoded = decode_message(&mut buf).unwrap();
        assert_eq!(decoded, msg);
    }

    // -- has_more --

    #[test]
    fn message_batch_has_more() {
        let batch = MessageBatch {
            messages: vec![sample_message(1)],
            has_more: true,
        };
        let mut buf = BytesMut::new();
        encode_message_batch(&mut buf, &batch).unwrap();
        let decoded = decode_message_batch(&mut buf).unwrap();
        assert!(decoded.has_more);
        assert_eq!(decoded, batch);
    }

    #[test]
    fn disconnect_code_event_seq_overflow() {
        let code = DisconnectCode::EventSeqOverflow;
        assert!(code.should_reconnect());
        assert_eq!(code as u16, 3006);
        assert_eq!(DisconnectCode::from_u16(3006), Some(code));
    }
}

// ===========================================================================
// Proptest
// ===========================================================================

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    // -- Strategies --

    fn arb_timestamp() -> impl Strategy<Value = i64> {
        MIN_TIMESTAMP..=MAX_TIMESTAMP
    }

    fn arb_frame_kind() -> impl Strategy<Value = FrameKind> {
        prop::sample::select(FrameKind::all())
    }

    fn arb_message_kind() -> impl Strategy<Value = MessageKind> {
        prop_oneof![
            Just(MessageKind::Text),
            Just(MessageKind::Image),
            Just(MessageKind::File),
            Just(MessageKind::System),
        ]
    }

    fn arb_message_flags() -> impl Strategy<Value = MessageFlags> {
        (0u16..=0x00FFu16).prop_map(MessageFlags::from_bits_truncate)
    }

    fn arb_rich_style() -> impl Strategy<Value = RichStyle> {
        (0u16..=0x07FFu16).prop_map(RichStyle::from_bits_truncate)
    }

    fn arb_short_string() -> impl Strategy<Value = String> {
        prop::string::string_regex("[a-zA-Z0-9_ ]{0,100}").unwrap()
    }

    fn arb_rich_span(max_offset: u32) -> impl Strategy<Value = RichSpan> {
        (
            0..=max_offset,
            0..=max_offset,
            arb_rich_style(),
            prop::option::of(arb_short_string()),
        )
            .prop_map(|(a, b, style, meta)| {
                let (start, end) = if a <= b { (a, b) } else { (b, a) };
                // Normalize: Some("") → None (matches wire format behavior)
                let meta = meta.filter(|s| !s.is_empty());
                RichSpan {
                    start,
                    end,
                    style,
                    meta,
                }
            })
    }

    fn arb_message() -> impl Strategy<Value = Message> {
        (
            any::<u32>(),
            any::<u32>(),
            any::<u32>(),
            arb_timestamp(),
            arb_timestamp(),
            arb_message_kind(),
            arb_message_flags(),
            prop::option::of(any::<u32>()),
            arb_short_string(),
            prop::option::of(prop::collection::vec(arb_rich_span(200), 0..10)),
            prop::option::of(arb_short_string()),
        )
            .prop_map(
                |(id, chat_id, sender_id, created_at, updated_at, kind, flags, reply_to_id, content, rich, extra)| {
                    Message {
                        id,
                        chat_id,
                        sender_id,
                        created_at,
                        updated_at,
                        kind,
                        flags,
                        reply_to_id,
                        content,
                        rich_content: rich,
                        // Normalize: Some("") → None (matches wire format behavior)
                        extra: extra.filter(|s| !s.is_empty()),
                    }
                },
            )
    }

    // -- Property tests --

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10_000))]

        #[test]
        fn header_roundtrip(kind in arb_frame_kind(), seq in any::<u32>()) {
            let header = FrameHeader { kind, seq, event_seq: 0 };
            let mut buf = BytesMut::new();
            encode_header(&mut buf, &header);
            let decoded = decode_header(&mut buf.freeze()).unwrap();
            prop_assert_eq!(decoded, header);
        }

        #[test]
        fn string_roundtrip(s in ".*") {
            let mut buf = BytesMut::new();
            write_string(&mut buf, &s);
            let decoded = read_string(&mut buf).unwrap();
            prop_assert_eq!(decoded, s);
        }

        #[test]
        fn optional_string_roundtrip(s in prop::option::of(".*")) {
            let mut buf = BytesMut::new();
            write_optional_string(&mut buf, s.as_deref());
            let decoded = read_optional_string(&mut buf).unwrap();
            // Note: Some("") encodes as len=0 which decodes as None
            if s.as_deref() == Some("") {
                prop_assert_eq!(decoded, None);
            } else {
                prop_assert_eq!(decoded, s);
            }
        }

        #[test]
        fn timestamp_roundtrip(ts in arb_timestamp()) {
            let mut buf = BytesMut::new();
            write_timestamp(&mut buf, ts).unwrap();
            let decoded = read_timestamp(&mut buf).unwrap();
            prop_assert_eq!(decoded, ts);
        }

        #[test]
        fn message_roundtrip(msg in arb_message()) {
            let mut buf = BytesMut::new();
            encode_message(&mut buf, &msg).unwrap();
            let decoded = decode_message(&mut buf).unwrap();
            prop_assert_eq!(decoded, msg);
        }

        #[test]
        fn message_batch_roundtrip(count in 0usize..100, has_more in any::<bool>()) {
            let batch = MessageBatch {
                messages: (0..count as u32).map(|i| Message {
                    id: i,
                    chat_id: 1,
                    sender_id: 1,
                    created_at: 1_711_100_000,
                    updated_at: 1_711_100_000,
                    kind: MessageKind::Text,
                    flags: MessageFlags::empty(),
                    reply_to_id: None,
                    content: format!("msg {i}"),
                    rich_content: None,
                    extra: None,
                }).collect(),
                has_more,
            };
            let mut buf = BytesMut::new();
            encode_message_batch(&mut buf, &batch).unwrap();
            let decoded = decode_message_batch(&mut buf).unwrap();
            prop_assert_eq!(decoded, batch);
        }

        #[test]
        fn rich_content_roundtrip(spans in prop::collection::vec(arb_rich_span(1000), 0..20)) {
            let mut buf = BytesMut::new();
            encode_rich_content(&mut buf, &spans);
            let decoded = decode_rich_content(&mut buf).unwrap();
            prop_assert_eq!(decoded, spans);
        }

        #[test]
        fn hello_roundtrip(
            version in any::<u8>(),
            sdk in arb_short_string(),
            platform in arb_short_string(),
            token in arb_short_string(),
        ) {
            let payload = HelloPayload {
                protocol_version: version,
                sdk_version: sdk,
                platform,
                token,
                device_id: uuid::Uuid::new_v4(),
            };
            let mut buf = BytesMut::new();
            encode_hello(&mut buf, &payload).unwrap();
            let decoded = decode_hello(&mut buf).unwrap();
            prop_assert_eq!(decoded, payload);
        }

        #[test]
        fn chat_entry_roundtrip(
            id in any::<u32>(),
            kind_idx in 0u8..3,
            parent in prop::option::of(any::<u32>()),
            created_at in arb_timestamp(),
            updated_at in arb_timestamp(),
            title in prop::option::of(arb_short_string()),
            avatar in prop::option::of(arb_short_string()),
        ) {
            let kind = ChatKind::from_u8(kind_idx).unwrap();
            let entry = ChatEntry {
                id,
                kind,
                parent_id: parent,
                created_at,
                updated_at,
                title: title.filter(|s| !s.is_empty()),
                avatar_url: avatar.filter(|s| !s.is_empty()),
                last_message: None,
                unread_count: 0,
                member_count: 0,
            };
            let mut buf = BytesMut::new();
            encode_chat_entry(&mut buf, &entry).unwrap();
            let decoded = decode_chat_entry(&mut buf).unwrap();
            prop_assert_eq!(decoded, entry);
        }

        #[test]
        fn user_entry_roundtrip(
            id in any::<u32>(),
            flags in (0u16..=0x0007u16).prop_map(UserFlags::from_bits_truncate),
            created_at in arb_timestamp(),
            updated_at in arb_timestamp(),
        ) {
            let entry = UserEntry {
                id,
                flags,
                created_at,
                updated_at,
                username: Some("testuser".into()),
                first_name: Some("Test".into()),
                last_name: None,
                avatar_url: None,
            };
            let mut buf = BytesMut::new();
            encode_user_entry(&mut buf, &entry).unwrap();
            let decoded = decode_user_entry(&mut buf).unwrap();
            prop_assert_eq!(decoded, entry);
        }

        #[test]
        fn send_message_roundtrip(
            chat_id in any::<u32>(),
            kind_idx in 0u8..3,
            reply_to_id in prop::option::of(any::<u32>()),
            content in arb_short_string(),
            extra in prop::option::of(arb_short_string()),
        ) {
            let kind = MessageKind::from_u8(kind_idx).unwrap();
            let payload = SendMessagePayload {
                chat_id,
                kind,
                idempotency_key: uuid::Uuid::new_v4(),
                reply_to_id,
                content,
                rich_content: None,
                extra: extra.filter(|s| !s.is_empty()),
                mentioned_user_ids: vec![],
            };
            let mut buf = BytesMut::new();
            encode_send_message(&mut buf, &payload);
            let decoded = decode_send_message(&mut buf).unwrap();
            prop_assert_eq!(decoded, payload);
        }

        #[test]
        fn load_chats_roundtrip(limit in any::<u16>(), cursor_ts in arb_timestamp()) {
            // FirstPage
            let p1 = LoadChatsPayload::FirstPage { limit };
            let mut buf = BytesMut::new();
            encode_load_chats(&mut buf, &p1).unwrap();
            let d1 = decode_load_chats(&mut buf).unwrap();
            prop_assert_eq!(d1, p1);

            // After
            let p2 = LoadChatsPayload::After { cursor_ts, limit };
            let mut buf = BytesMut::new();
            encode_load_chats(&mut buf, &p2).unwrap();
            let d2 = decode_load_chats(&mut buf).unwrap();
            prop_assert_eq!(d2, p2);
        }

        #[test]
        fn subscribe_roundtrip(channels in prop::collection::vec(arb_short_string(), 0..50)) {
            let payload = SubscribePayload { channels };
            let mut buf = BytesMut::new();
            encode_subscribe(&mut buf, &payload);
            let decoded = decode_subscribe(&mut buf).unwrap();
            prop_assert_eq!(decoded, payload);
        }

        #[test]
        fn error_payload_roundtrip(
            msg in arb_short_string(),
            retry in any::<u32>(),
            extra in prop::option::of(arb_short_string()),
        ) {
            let payload = ErrorPayload {
                code: ErrorCode::InternalError,
                message: msg,
                retry_after_ms: retry,
                extra: extra.filter(|s| !s.is_empty()),
            };
            let mut buf = BytesMut::new();
            encode_error(&mut buf, &payload);
            let decoded = decode_error(&mut buf).unwrap();
            prop_assert_eq!(decoded, payload);
        }

        #[test]
        fn load_messages_roundtrip(
            chat_id in any::<u32>(),
            anchor_id in any::<u32>(),
            limit in any::<u16>(),
            dir in 0u8..2,
        ) {
            let payload = LoadMessagesPayload::Paginate {
                chat_id,
                direction: LoadDirection::from_u8(dir).unwrap(),
                anchor_id,
                limit,
            };
            let mut buf = BytesMut::new();
            encode_load_messages(&mut buf, &payload).unwrap();
            let decoded = decode_load_messages(&mut buf).unwrap();
            prop_assert_eq!(decoded, payload);
        }

        #[test]
        fn presence_result_roundtrip(count in 0usize..50) {
            let entries: Vec<PresenceEntry> = (0..count as u32)
                .map(|i| PresenceEntry {
                    user_id: i,
                    status: if i % 2 == 0 { PresenceStatus::Online } else { PresenceStatus::Offline },
                    last_seen: if i % 2 == 0 { 0 } else { 1_711_100_000 },
                })
                .collect();
            let mut buf = BytesMut::new();
            encode_presence_result(&mut buf, &entries).unwrap();
            let decoded = decode_presence_result(&mut buf).unwrap();
            prop_assert_eq!(decoded, entries);
        }
    }
}
