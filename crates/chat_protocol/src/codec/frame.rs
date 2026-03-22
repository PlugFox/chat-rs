//! Unified frame encode/decode — symmetric serialization of `Frame`.

use bytes::{Buf, BufMut};

use crate::error::CodecError;
use crate::types::*;

use super::header::*;
use super::message::*;
use super::payload::*;

/// Encode a complete `Frame` (header + payload) into the buffer.
pub fn encode_frame(buf: &mut impl BufMut, frame: &Frame) -> Result<(), CodecError> {
    let header = FrameHeader {
        kind: frame.payload.kind(),
        seq: frame.seq,
        event_seq: frame.event_seq,
    };
    encode_header(buf, &header);

    match &frame.payload {
        FramePayload::Hello(p) => encode_hello(buf, p),
        FramePayload::Welcome(p) => encode_welcome(buf, p),
        FramePayload::Ping | FramePayload::Pong => Ok(()),
        FramePayload::RefreshToken(p) => {
            encode_refresh_token(buf, p);
            Ok(())
        }

        FramePayload::SendMessage(p) => {
            encode_send_message(buf, p);
            Ok(())
        }
        FramePayload::EditMessage(p) => {
            encode_edit_message(buf, p);
            Ok(())
        }
        FramePayload::DeleteMessage(p) => {
            encode_delete_message(buf, p);
            Ok(())
        }
        FramePayload::ReadReceipt(p) => {
            encode_read_receipt(buf, p);
            Ok(())
        }
        FramePayload::Typing(p) => {
            encode_typing(buf, p);
            Ok(())
        }
        FramePayload::GetPresence(p) => {
            encode_get_presence(buf, p);
            Ok(())
        }
        FramePayload::LoadChats(p) => encode_load_chats(buf, p),
        FramePayload::Search(p) => {
            encode_search(buf, p);
            Ok(())
        }
        FramePayload::Subscribe(p) => {
            encode_subscribe(buf, p);
            Ok(())
        }
        FramePayload::Unsubscribe(p) => {
            encode_unsubscribe(buf, p);
            Ok(())
        }
        FramePayload::LoadMessages(p) => encode_load_messages(buf, p),
        FramePayload::AddReaction(p) => {
            encode_add_reaction(buf, p);
            Ok(())
        }
        FramePayload::RemoveReaction(p) => {
            encode_remove_reaction(buf, p);
            Ok(())
        }
        FramePayload::PinMessage(p) => {
            encode_pin_message(buf, p);
            Ok(())
        }
        FramePayload::UnpinMessage(p) => {
            encode_unpin_message(buf, p);
            Ok(())
        }
        FramePayload::ForwardMessage(p) => {
            encode_forward_message(buf, p);
            Ok(())
        }

        FramePayload::MessageNew(msg) | FramePayload::MessageEdited(msg) => encode_message(buf, msg),
        FramePayload::MessageDeleted(p) => {
            encode_message_deleted(buf, p);
            Ok(())
        }
        FramePayload::ReceiptUpdate(p) => {
            encode_receipt_update(buf, p);
            Ok(())
        }
        FramePayload::TypingUpdate(p) => {
            encode_typing_update(buf, p);
            Ok(())
        }
        FramePayload::MemberJoined(p) => {
            encode_member_joined(buf, p);
            Ok(())
        }
        FramePayload::MemberLeft(p) => {
            encode_member_left(buf, p);
            Ok(())
        }
        FramePayload::PresenceResult(entries) => encode_presence_result(buf, entries),
        FramePayload::ChatUpdated(e) | FramePayload::ChatCreated(e) => encode_chat_entry(buf, e),
        FramePayload::ReactionUpdate(p) => {
            encode_reaction_update(buf, p);
            Ok(())
        }
        FramePayload::UserUpdated(e) => encode_user_entry(buf, e),
        FramePayload::ChatDeleted(p) => {
            encode_chat_deleted(buf, p);
            Ok(())
        }
        FramePayload::MemberUpdated(p) => {
            encode_member_updated(buf, p);
            Ok(())
        }

        FramePayload::Ack(_) => {
            // Ack payloads are context-dependent — encode_frame writes only the
            // raw bytes. Callers that construct Ack frames are responsible for
            // placing the correct payload bytes.
            Ok(())
        }
        FramePayload::Error(p) => {
            encode_error(buf, p);
            Ok(())
        }

        FramePayload::CreateChat(p) => {
            encode_create_chat(buf, p);
            Ok(())
        }
        FramePayload::UpdateChat(p) => {
            encode_update_chat(buf, p);
            Ok(())
        }
        FramePayload::DeleteChat(p) => {
            encode_delete_chat(buf, p);
            Ok(())
        }
        FramePayload::GetChatInfo(p) => {
            encode_get_chat_info(buf, p);
            Ok(())
        }
        FramePayload::GetChatMembers(p) => {
            encode_get_chat_members(buf, p);
            Ok(())
        }
        FramePayload::InviteMembers(p) => {
            encode_invite_members(buf, p);
            Ok(())
        }
        FramePayload::UpdateMember(p) => {
            encode_update_member(buf, p);
            Ok(())
        }
        FramePayload::LeaveChat(p) => {
            encode_leave_chat(buf, p);
            Ok(())
        }
        FramePayload::MuteChat(p) => {
            encode_mute_chat(buf, p);
            Ok(())
        }
        FramePayload::UnmuteChat(p) => {
            encode_unmute_chat(buf, p);
            Ok(())
        }
        FramePayload::GetUser(p) => {
            encode_get_user(buf, p);
            Ok(())
        }
        FramePayload::GetUsers(p) => {
            encode_get_users(buf, p);
            Ok(())
        }
        FramePayload::UpdateProfile(p) => {
            encode_update_profile(buf, p);
            Ok(())
        }
        FramePayload::BlockUser(p) => {
            encode_block_user(buf, p);
            Ok(())
        }
        FramePayload::UnblockUser(p) => {
            encode_unblock_user(buf, p);
            Ok(())
        }
        FramePayload::GetBlockList(p) => {
            encode_get_block_list(buf, p);
            Ok(())
        }
    }
}

/// Decode a complete `Frame` (header + payload) from the buffer.
///
/// For `Ack` frames the remaining bytes are captured as raw payload
/// (context-dependent decoding is left to the caller).
pub fn decode_frame(buf: &mut impl Buf) -> Result<Frame, CodecError> {
    let header = decode_header(buf)?;
    let payload = match header.kind {
        FrameKind::Hello => FramePayload::Hello(decode_hello(buf)?),
        FrameKind::Welcome => FramePayload::Welcome(decode_welcome(buf)?),
        FrameKind::Ping => FramePayload::Ping,
        FrameKind::Pong => FramePayload::Pong,
        FrameKind::RefreshToken => FramePayload::RefreshToken(decode_refresh_token(buf)?),

        FrameKind::SendMessage => FramePayload::SendMessage(decode_send_message(buf)?),
        FrameKind::EditMessage => FramePayload::EditMessage(decode_edit_message(buf)?),
        FrameKind::DeleteMessage => FramePayload::DeleteMessage(decode_delete_message(buf)?),
        FrameKind::ReadReceipt => FramePayload::ReadReceipt(decode_read_receipt(buf)?),
        FrameKind::Typing => FramePayload::Typing(decode_typing(buf)?),
        FrameKind::GetPresence => FramePayload::GetPresence(decode_get_presence(buf)?),
        FrameKind::LoadChats => FramePayload::LoadChats(decode_load_chats(buf)?),
        FrameKind::Search => FramePayload::Search(decode_search(buf)?),
        FrameKind::Subscribe => FramePayload::Subscribe(decode_subscribe(buf)?),
        FrameKind::Unsubscribe => FramePayload::Unsubscribe(decode_unsubscribe(buf)?),
        FrameKind::LoadMessages => FramePayload::LoadMessages(decode_load_messages(buf)?),
        FrameKind::AddReaction => FramePayload::AddReaction(decode_add_reaction(buf)?),
        FrameKind::RemoveReaction => FramePayload::RemoveReaction(decode_remove_reaction(buf)?),
        FrameKind::PinMessage => FramePayload::PinMessage(decode_pin_message(buf)?),
        FrameKind::UnpinMessage => FramePayload::UnpinMessage(decode_unpin_message(buf)?),
        FrameKind::ForwardMessage => FramePayload::ForwardMessage(decode_forward_message(buf)?),

        FrameKind::MessageNew => FramePayload::MessageNew(decode_message(buf)?),
        FrameKind::MessageEdited => FramePayload::MessageEdited(decode_message(buf)?),
        FrameKind::MessageDeleted => FramePayload::MessageDeleted(decode_message_deleted(buf)?),
        FrameKind::ReceiptUpdate => FramePayload::ReceiptUpdate(decode_receipt_update(buf)?),
        FrameKind::TypingUpdate => FramePayload::TypingUpdate(decode_typing_update(buf)?),
        FrameKind::MemberJoined => FramePayload::MemberJoined(decode_member_joined(buf)?),
        FrameKind::MemberLeft => FramePayload::MemberLeft(decode_member_left(buf)?),
        FrameKind::PresenceResult => FramePayload::PresenceResult(decode_presence_result(buf)?),
        FrameKind::ChatUpdated => FramePayload::ChatUpdated(decode_chat_entry(buf)?),
        FrameKind::ChatCreated => FramePayload::ChatCreated(decode_chat_entry(buf)?),
        FrameKind::ReactionUpdate => FramePayload::ReactionUpdate(decode_reaction_update(buf)?),
        FrameKind::UserUpdated => FramePayload::UserUpdated(decode_user_entry(buf)?),
        FrameKind::ChatDeleted => FramePayload::ChatDeleted(decode_chat_deleted(buf)?),
        FrameKind::MemberUpdated => FramePayload::MemberUpdated(decode_member_updated(buf)?),

        FrameKind::Ack => {
            // Capture remaining bytes — caller decodes based on original request kind.
            let remaining = buf.remaining();
            if remaining > 0 {
                let raw = buf.copy_to_bytes(remaining).to_vec();
                FramePayload::Ack(AckPayload::MessageBatch(raw))
            } else {
                FramePayload::Ack(AckPayload::Empty)
            }
        }
        FrameKind::Error => FramePayload::Error(decode_error(buf)?),

        FrameKind::CreateChat => FramePayload::CreateChat(decode_create_chat(buf)?),
        FrameKind::UpdateChat => FramePayload::UpdateChat(decode_update_chat(buf)?),
        FrameKind::DeleteChat => FramePayload::DeleteChat(decode_delete_chat(buf)?),
        FrameKind::GetChatInfo => FramePayload::GetChatInfo(decode_get_chat_info(buf)?),
        FrameKind::GetChatMembers => FramePayload::GetChatMembers(decode_get_chat_members(buf)?),
        FrameKind::InviteMembers => FramePayload::InviteMembers(decode_invite_members(buf)?),
        FrameKind::UpdateMember => FramePayload::UpdateMember(decode_update_member(buf)?),
        FrameKind::LeaveChat => FramePayload::LeaveChat(decode_leave_chat(buf)?),
        FrameKind::MuteChat => FramePayload::MuteChat(decode_mute_chat(buf)?),
        FrameKind::UnmuteChat => FramePayload::UnmuteChat(decode_unmute_chat(buf)?),

        FrameKind::GetUser => FramePayload::GetUser(decode_get_user(buf)?),
        FrameKind::GetUsers => FramePayload::GetUsers(decode_get_users(buf)?),
        FrameKind::UpdateProfile => FramePayload::UpdateProfile(decode_update_profile(buf)?),
        FrameKind::BlockUser => FramePayload::BlockUser(decode_block_user(buf)?),
        FrameKind::UnblockUser => FramePayload::UnblockUser(decode_unblock_user(buf)?),
        FrameKind::GetBlockList => FramePayload::GetBlockList(decode_get_block_list(buf)?),
    };

    Ok(Frame {
        seq: header.seq,
        event_seq: header.event_seq,
        payload,
    })
}
