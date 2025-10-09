extends CharacterBody3D

@export var speed := 5.0
@export var gravity: float = ProjectSettings.get_setting("physics/3d/default_gravity") 
@export var mouse_sensitivity: float = 0.002

@onready var head = $SpringArm3Dw

const MAX_PITCH: float = deg_to_rad(85.0)
const MIN_PITCH: float = deg_to_rad(-85.0)

func _ready() -> void:
	Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED)

func _input(event: InputEvent) -> void:
	if event is InputEventMouseMotion:
		var delta = event.relative
		rotate_y(-delta.x * mouse_sensitivity)
		
		var new_pitch = head.rotation.x - (delta.y * mouse_sensitivity)
		new_pitch = clamp(new_pitch,MIN_PITCH,MAX_PITCH)
		head.rotation.x = new_pitch
		
func _physics_process(delta: float) -> void:
	var input_vector = Input.get_vector("ui_left", "ui_right", "ui_up", "ui_down")
	# 2. Handle Gravity
	if not is_on_floor():
		# Apply gravity downwards (-Y)
		velocity.y -= gravity * delta
	else:
		velocity.y = 0
	
	var direction = (transform.basis * Vector3(input_vector.x,0,input_vector.y)).normalized()
	
	if direction:
		velocity.x = direction.x * speed
		velocity.z = direction.z * speed
	else:
		velocity.x = move_toward(velocity.x,0,speed * delta * 10.0)
		velocity.z = move_toward(velocity.z,0,speed * delta * 10.0)
	move_and_slide()
