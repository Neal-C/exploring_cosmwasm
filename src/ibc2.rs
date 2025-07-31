use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    from_json, to_json_vec, Binary, DepsMut, Env, Ibc2Msg, Ibc2PacketAckMsg, Ibc2PacketReceiveMsg, Ibc2PacketSendMsg, Ibc2PacketTimeoutMsg, Ibc2Payload, IbcBasicResponse, IbcReceiveResponse, StdAck, StdError, StdResult
};
use cosmwasm_std::entry_point;

#[cw_serde]
pub struct PingPongMsg {
    pub counter: u64,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc2_packet_ack(
    _deps: DepsMut,
    _env: Env,
    _msg: Ibc2PacketAckMsg,
) -> StdResult<IbcBasicResponse> {
    // Do nothing
 
    Ok(IbcBasicResponse::default())
}
 
/// Handles the receipt of an IBCv2 packet and responds by incrementing the counter
/// and sending it back to the source.
///
/// # Arguments
/// - `_deps`: Mutable dependencies of the contract.
/// - `env`: The current blockchain environment.
/// - `msg`: The received IBCv2 packet message.
///
/// # Returns
/// - `StdResult<IbcReceiveResponse>`: Response including a new IBC packet and a successful ack.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc2_packet_receive(
    _deps: DepsMut,
    env: Env,
    msg: Ibc2PacketReceiveMsg,
) -> StdResult<IbcReceiveResponse> {
    let binary_payload = &msg.payload.value;
    let json_payload: PingPongMsg = from_json(binary_payload)?;
 
    let new_payload = Ibc2Payload::new(
        // Swap the source with destination ports to send the message back to the source contract
        msg.payload.destination_port,
        msg.payload.source_port,
        msg.payload.version,
        msg.payload.encoding,
        Binary::new(to_json_vec(&PingPongMsg {
            counter: json_payload.counter + 1,
        })?),
    );
 
    let new_msg = Ibc2Msg::SendPacket {
        source_client: msg.source_client,
        payloads: vec![new_payload],
        timeout: env.block.time.plus_minutes(5_u64),
    };
 
    Ok(IbcReceiveResponse::new(StdAck::success(b"\x01")).add_message(new_msg))
}
 
/// Handles timeouts of previously sent IBC packets. Automatically resends the message
/// without validation or retry limits.
///
/// # Arguments
/// - `_deps`: Mutable dependencies of the contract.
/// - `env`: The current blockchain environment.
/// - `msg`: The timeout message with the failed payload.
///
/// # Returns
/// - `StdResult<IbcBasicResponse>`: Response with the resend attempt.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc2_packet_timeout(
    _deps: DepsMut,
    env: Env,
    msg: Ibc2PacketTimeoutMsg,
) -> StdResult<IbcBasicResponse> {
    // Let's resend the message without any check.
    // It'd be good to constrain the number of trials.
 
    let msg = Ibc2Msg::SendPacket {
        source_client: msg.source_client,
        payloads: vec![msg.payload],
        timeout: env.block.time.plus_minutes(5_u64),
    };
 
    Ok(IbcBasicResponse::default().add_message(msg))
}
 
/// Called when an IBCv2 packet is sent. Validates that the sender is the contract itself.
///
/// # Arguments
/// - `_deps`: Mutable dependencies of the contract.
/// - `_env`: The current blockchain environment.
/// - `msg`: The packet send message.
///
/// # Returns
/// - `StdResult<IbcBasicResponse>`: Default response if sender is valid, error otherwise.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc2_packet_send(
    _deps: DepsMut,
    _env: Env,
    msg: Ibc2PacketSendMsg,
) -> StdResult<IbcBasicResponse> {
    if msg.signer != _env.contract.address {
        return Err(StdError::msg(
            "Only this contract can send messages from its IBCv2 port ID",
        ));
    }
    Ok(IbcBasicResponse::default())
}