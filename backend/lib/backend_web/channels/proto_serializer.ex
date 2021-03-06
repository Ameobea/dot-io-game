defmodule BackendWeb.ProtoSerializer do

  @behaviour Phoenix.Transports.Serializer

  alias Phoenix.Socket.{Reply, Message, Broadcast}

  def fastlane!(%Broadcast{} = msg) do
    {:socket_push, :binary, Backend.ProtoMessage.encode_socket_message(%Message{
      topic: msg.topic,
      event: msg.event,
      payload: msg.payload
    })}
  end

  def encode!(%Reply{} = reply) do
    {:socket_push, :binary, Backend.ProtoMessage.encode_socket_message(%Message{
      topic: reply.topic,
      event: "phx_reply",
      ref: reply.ref,
      payload: %{status: reply.status, response: reply.payload}
    })}
  end

  def encode!(%Message{} = msg) do
    {:socket_push, :binary, Backend.ProtoMessage.encode_socket_message(msg)}
  end

  def decode!(message, _opts) do
    decoded = Backend.ProtoMessage.ClientChannelMessage.decode(message)

    %Phoenix.Socket.Message{
      topic: decoded.topic,
      event: decode_event(decoded.event),
      payload: decoded.payload,
      ref: decoded.ref,
    }
  end

  defp decode_event(%Backend.ProtoMessage.Event{payload: {:phoenix_event, event_name}}) do
    "phx_" <> (event_name |> Atom.to_string |> String.downcase)
  end
  defp decode_event(%Backend.ProtoMessage.Event{payload: {:custom_event, event_name}}), do: event_name
end
