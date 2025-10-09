extends Node

var client_peer = WebSocketPeer.new()

const SERVER_URL = "ws://localhost"
var connection_established = false
var connection_attempt_started = false

func _ready() -> void:
	connect_to_server()

func connect_to_server():
	var err = client_peer.connect_to_url(SERVER_URL)
	if err == OK:
		connection_attempt_started = true
	else:
		connection_attempt_started = false
		print("Failed to start connection",error_string(err))
		
func _process(_delta: float) -> void:
	client_peer.poll()
	
	var state = client_peer.get_ready_state()
	match state:
		WebSocketPeer.STATE_OPEN:
			if not connection_established:
				connection_established = true
				connection_attempt_started = false
				
			if client_peer.get_available_packet_count() > 0:
				var text = client_peer.get_packet().get_string_from_utf8()
				_handle_received_data(text)
		WebSocketPeer.STATE_CLOSED,WebSocketPeer.STATE_CLOSING:
			if connection_established:
				connection_established = false
			elif connection_attempt_started:
				connection_attempt_started = false
				
		WebSocketPeer.STATE_CONNECTING:
			pass
func _handle_received_data(text: String):
	var parse_result = JSON.parse_string(text)
	
	if typeof(parse_result) == TYPE_DICTIONARY:
		pass
	else:
		pass		
		
